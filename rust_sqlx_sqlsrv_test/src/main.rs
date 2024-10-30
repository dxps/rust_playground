use rust_sqlx_sqlsrv_test::{
    get_user_session_using_parameterized_query, get_user_session_using_string_constructed_query,
    init_client, uninit_client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    dotenvy::dotenv()?;

    let mut client = init_client().await?;
    let token = "someToken";

    println!(">>> Successfully connected to SQL Server.",);

    println!("\n>>> Trying to get user session using parameterized query ...",);
    match get_user_session_using_parameterized_query(&mut client, token.into()).await {
        Ok(user_session) => match user_session {
            Some(user_session) => {
                println!(">>> Found user session: {:?}", user_session);
            }
            None => {
                println!(">>> No result.");
            }
        },
        Err(e) => {
            println!(">>> Failed to get user session. Cause: '{e}'");
        }
    }

    println!("\n>>> Trying to get user session using string constructed query ...",);
    match get_user_session_using_string_constructed_query(&mut client, token.into()).await {
        Ok(user_session) => match user_session {
            Some(user_session) => {
                println!(">>> Found {:?}", user_session);
            }
            None => {
                println!(">>> No user session found.");
            }
        },
        Err(e) => {
            println!(">>> Failed to query for user session. Cause: '{e}'");
        }
    }

    uninit_client(client).await?;
    println!("\n>>> Successfully disconnected from SQL Server.");

    Ok(())
}
