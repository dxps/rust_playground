use std::{collections::HashMap, sync::Arc, time::Duration};

use crate::{
    data::{self, DatabasePool},
    service, ServiceError, ShortCode,
};
use crossbeam_channel::{unbounded, Sender};
use parking_lot::Mutex;
use tokio::runtime::Handle;

type HitStore = Arc<Mutex<HashMap<ShortCode, u32>>>;

enum HitCountMsg {
    Commit,
    Hit(ShortCode, u32),
}

#[derive(Debug, thiserror::Error)]
enum HitCountError {
    #[error("service error {0}")]
    Service(#[from] ServiceError),

    #[error("communication error: {0}")]
    Channel(#[from] crossbeam_channel::SendError<HitCountMsg>),
}

pub struct HitCounter {
    tx: Sender<HitCountMsg>,
}

impl HitCounter {
    pub fn new(pool: DatabasePool, handle: Handle) -> Self {
        let (tx, rx) = unbounded();
        let tx_clone = tx.clone();
        let rx_clone = rx.clone();

        let _ = std::thread::spawn(move || {
            println!("HitCounter thread spawned");
            let store: HitStore = Arc::new(Mutex::new(HashMap::new()));
            loop {
                match rx_clone.try_recv() {
                    Ok(msg) => {
                        if let Err(e) =
                            Self::process_msg(msg, store.clone(), handle.clone(), pool.clone())
                        {
                            eprintln!("message processing error: {}", e);
                        }
                    }
                    Err(e) => match e {
                        crossbeam_channel::TryRecvError::Empty => {
                            std::thread::sleep(Duration::from_secs(5));
                            if let Err(e) = tx_clone.send(HitCountMsg::Commit) {
                                eprintln!("error sending commit msg to hits channel: {}", e);
                            }
                        }
                        // We end in this case since it means that the channel is hung up,
                        // so we cannot continue no matter what.
                        _ => break,
                    },
                }
            }
        });
        Self { tx }
    }

    pub fn hit(&self, shortcode: ShortCode, count: u32) {
        if let Err(e) = self.tx.send(HitCountMsg::Hit(shortcode, count)) {
            eprintln!("hit count error: {}", e);
        }
    }

    fn process_msg(
        msg: HitCountMsg,
        hits: HitStore,
        handle: Handle,
        pool: DatabasePool,
    ) -> Result<(), HitCountError> {
        match msg {
            HitCountMsg::Commit => Self::commit_hits(hits.clone(), handle.clone(), pool.clone())?,
            HitCountMsg::Hit(shortcode, count) => {
                let mut hitcount = hits.lock();
                let hitcount = hitcount.entry(shortcode).or_insert(0);
                *hitcount += count;
            }
        }
        Ok(())
    }

    fn commit_hits(
        hits: HitStore,
        handle: Handle,
        pool: DatabasePool,
    ) -> Result<(), HitCountError> {
        let hits = Arc::clone(&hits);
        let hits: Vec<(ShortCode, u32)> = {
            let mut hits = hits.lock();
            let hits_vec = hits.iter().map(|(k, v)| (k.clone(), *v)).collect();
            hits.clear();
            hits_vec
        };
        handle.block_on(async move {
            let txn = data::begin_transaction(&pool).await?;
            for (shortcode, hits) in hits {
                if let Err(e) = data::increase_hit_count(&shortcode, hits, &pool).await {
                    eprintln!(" error increasing hit count: {}", e);
                }
            }
            Ok(data::commit_transaction(txn).await?)
        })
    }
}
