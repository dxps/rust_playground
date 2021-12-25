use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();
    let dsn = "postgres://test:test@localhost:5437";
    let pg_mgr = PostgresConnectionManager::new_from_stringlike(dsn, NoTls).unwrap();

    println!("Hello, world!");
}
