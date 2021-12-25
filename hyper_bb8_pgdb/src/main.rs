use std::{convert::Infallible, net::SocketAddr};

use bb8::{Pool, RunError};
use bb8_postgres::PostgresConnectionManager;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Error, Response, Server,
};
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    let dsn = "postgres://test:test@localhost:5437/test";
    let pg_mgr = PostgresConnectionManager::new_from_stringlike(dsn, NoTls).unwrap();

    println!("Listening for HTTP requests on port {}", &addr.port());
    let pool = match Pool::builder().build(pg_mgr).await {
        Ok(pool) => pool,
        Err(e) => panic!("DB Conn Pool init error: {}", e),
    };

    let _ = Server::bind(&addr)
        .serve(make_service_fn(move |_| {
            let pool = pool.clone();
            async move {
                Ok::<_, Error>(service_fn(move |_| {
                    let pool = pool.clone();
                    async move {
                        Ok::<_, Infallible>(match handler(pool).await {
                            Ok(rsp) => rsp,
                            Err(e) => Response::new(Body::from(format!("Internal error: {:?}", e))),
                        })
                    }
                }))
            }
        }))
        .await;
}

async fn handler(
    pool: Pool<PostgresConnectionManager<NoTls>>,
) -> Result<Response<Body>, RunError<tokio_postgres::Error>> {
    println!("Got request");
    let conn = pool.get().await?;
    let stmt = conn.prepare("SELECT 1").await?;
    let row = conn.query_one(&stmt, &[]).await?;
    let v = row.get::<usize, i32>(0);
    println!("Responding back ...");
    Ok(Response::new(Body::from(format!("Result: {:?}", v))))
}
