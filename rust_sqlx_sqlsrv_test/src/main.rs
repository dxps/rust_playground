use rust_sqlx_sqlsrv_test::{
    get_user_session, get_user_session_using_query, init_client, uninit_client,
};

#[derive(Debug)]
pub struct UserSession {
    id: String,
    username: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    dotenvy::dotenv()?;

    let mut client = init_client().await?;

    println!(">>> Successfully connected to SQL Server.",);

    println!(">>> Trying to get user session using parameterized query ...",);
    match get_user_session(&mut client, "someToken".into()).await {
        Ok(user_session) => match user_session {
            Some(user_session) => {
                println!(">>> Found user session: {:?}", user_session);
            }
            None => {
                println!(">>> No user session found.");
            }
        },
        Err(e) => {
            println!(">>> Failed to get user session. Cause: '{e}'");
        }
    }

    println!(">>> Trying to get user session using parameterized query ...",);
    match get_user_session_using_query(&mut client, "someToken".into()).await {
        Ok(user_session) => match user_session {
            Some(user_session) => {
                println!(">>> Found user session: {:?}", user_session);
            }
            None => {
                println!(">>> No user session found.");
            }
        },
        Err(e) => {
            println!(">>> Failed to get user session. Cause: '{e}'");
        }
    }

    uninit_client(client).await?;
    println!(">>> Successfully disconnected from SQL Server.");

    Ok(())
}
