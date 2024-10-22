use anyhow::Result;
use async_std::net::TcpStream;
use once_cell::sync::Lazy;
use tiberius::{Client, Config, SqlBrowser};

static JDBC_CONN_STRING: Lazy<String> = Lazy::new(|| {
    std::env::var("JDBC_CONN_STRING").unwrap_or_else(|_| {
        "jdbc:sqlserver://127.0.0.1:1433;database=test;user=developer;password=developer;trustServerCertificate=true".to_owned()
    })
});

/// Initialize the client by connecting to SQL Server.
pub async fn init_client() -> anyhow::Result<Client<TcpStream>> {
    //
    let config = Config::from_jdbc_string(&JDBC_CONN_STRING)?;

    let tcp = TcpStream::connect_named(&config).await?;
    tcp.set_nodelay(true)?;

    let client = Client::connect(config, tcp).await?;

    Ok(client)
}

/// Uninitilize the client by closing the connection to SQL Server.
pub async fn uninit_client(client: Client<TcpStream>) -> anyhow::Result<()> {
    Ok(client.close().await?)
}

#[derive(Debug)]
pub struct UserSession {
    id: String,
    username: String,
}

impl UserSession {
    pub fn new(id: String, username: String) -> Self {
        Self { id, username }
    }
}

pub async fn get_user_session(
    client: &mut Client<TcpStream>,
    token: String,
) -> Result<Option<UserSession>> {
    //
    let result = client
        .query("SELECT id, username from test.dbo.user_tokens where token = hashbytes('SHA2_256', @P1);", &[&token])
        .await?;

    let row = result.into_row().await?;
    if row.is_none() {
        return Ok(None);
    }
    let row = row.unwrap();

    let id: Option<&str> = row.get(0);
    let name: Option<&str> = row.get(1);
    let user_session = UserSession::new(id.unwrap().to_string(), name.unwrap().to_string());

    Ok(Some(user_session))
}

pub async fn get_user_session_using_query(
    client: &mut Client<TcpStream>,
    token: String,
) -> Result<Option<UserSession>> {
    //
    let query = format!(
        "SELECT id, username from test.dbo.user_tokens where token = hashbytes('SHA2_256', '{}');",
        token
    );
    let result = client.query(&query, &[]).await?;

    let row = result.into_row().await?;
    if row.is_none() {
        return Ok(None);
    }
    let row = row.unwrap();

    let id: Option<i32> = row.get(0);
    let name: Option<&str> = row.get(1);
    let user_session = UserSession::new(id.unwrap().to_string(), name.unwrap().to_string());

    Ok(Some(user_session))
}
