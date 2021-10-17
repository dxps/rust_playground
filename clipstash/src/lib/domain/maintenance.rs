use std::time::Duration;

use tokio::runtime::Handle;

use crate::{data::DatabasePool, service};

pub struct Maintenance;

impl Maintenance {
    pub fn spawn(pool: DatabasePool, handle: Handle) -> Self {
        handle.spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                if let Err(e) = service::delete_expired(&pool).await {
                    eprintln!("failed to delete expired clips: {}", e);
                }
            }
        });
        Self
    }
}
