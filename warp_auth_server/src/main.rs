use std::collections::HashMap;
use std::sync::Arc;

use argon2::{self, Config};
use rand::Rng;

use serde::Deserialize;
use tokio::sync::Mutex;
use warp::{http::StatusCode, Filter};

#[derive(Debug, Deserialize)]
struct User {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    let users_store = Arc::new(Mutex::new(HashMap::<String, User>::new()));
    let users_store_route = warp::any().map(move || Arc::clone(&users_store));

    // Notes:
    // - `warp::any()` is a Filter (ex: `warp::any().map(|| { "Hello!" });`).
    // - `warp::path()` accepts any request to the path matching the given string, otherwise it `reject`s it.
    // - `Filter`s can be combined and thus form a chain using:
    //      - `or()`: if path is not matched, the request is sent down to the next `Filter` chain.
    //      - `and()`: to chain filters together when the request isn't rejected

    let register_route = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(users_store_route.clone())
        .and_then(register);

    let login_route = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and(users_store_route.clone())
        .and_then(login);

    // let logout_route = warp::path("logout").map(|| "Logout Route");

    let routes = register_route.or(login_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await
}

async fn register(
    user: User,
    db: Arc<Mutex<HashMap<String, User>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut users = db.lock().await;
    if users.contains_key(&user.username) {
        return Ok(StatusCode::BAD_REQUEST);
    }
    // a user having hashed password will be saved instead
    // users.insert(user.username.clone(), user);
    let hashed_user = User {
        username: user.username,
        password: hash(user.password.as_bytes()),
    };
    users.insert(hashed_user.username.clone(), hashed_user);
    Ok(StatusCode::CREATED)
}

async fn login(
    credentials: User,
    db: Arc<Mutex<HashMap<String, User>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let users = db.lock().await;
    match users.get(&credentials.username) {
        None => Ok(StatusCode::BAD_REQUEST),
        Some(user) => {
            // if credentials.password == user.password {
            if verify(&user.password, credentials.password.as_bytes()) {
                Ok(StatusCode::OK)
            } else {
                Ok(StatusCode::UNAUTHORIZED)
            }
        }
    }
}

fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}
