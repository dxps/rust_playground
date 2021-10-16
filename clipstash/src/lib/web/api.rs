use crate::data::AppDatabase;
use crate::domain::clip::field::Password;
use crate::{service, Clip, ServiceError};
use rocket::http::{CookieJar, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use rocket::{Responder, State};
use serde::Serialize;
use std::str::FromStr;

use super::{HitCounter, PASSWORD_COOKIE};

pub const API_KEY_HEADER: &str = "x-api-key";

#[derive(Debug, Responder, thiserror::Error, Serialize)]
pub enum ApiKeyError {
    #[error("API key not found")]
    #[response(status = 404, content_type = "json")]
    NotFound(String),

    #[error("API key format is invalid")]
    #[response(status = 400, content_type = "json")]
    DecodeError(String),
}

#[derive(Debug, Clone)]
pub struct ApiKey(Vec<u8>);

impl ApiKey {
    pub fn to_base64(&self) -> String {
        base64::encode(self.0.as_slice())
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl Default for ApiKey {
    fn default() -> Self {
        let key = (0..16).map(|_| rand::random::<u8>()).collect();
        Self(key)
    }
}

impl FromStr for ApiKey {
    type Err = ApiKeyError;
    fn from_str(key: &str) -> Result<Self, Self::Err> {
        base64::decode(key)
            .map(ApiKey)
            .map_err(|e| Self::Err::DecodeError(e.to_string()))
    }
}

#[derive(Debug, Responder, thiserror::Error)]
pub enum ApiError {
    #[error("not found")]
    #[response(status = 404, content_type = "json")]
    NotFound(Json<String>),

    #[error("server error")]
    #[response(status = 500, content_type = "json")]
    Server(Json<String>),

    #[error("client error")]
    #[response(status = 401, content_type = "json")]
    Client(Json<String>),

    #[error("key error")]
    #[response(status = 400, content_type = "json")]
    KeyError(Json<ApiKeyError>),
}

impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::Clip(e) => Self::Client(Json(format!("clip parsing error: {}", e))),
            ServiceError::NotFound => Self::NotFound(Json("entity not found".to_owned())),
            ServiceError::Data(_) => Self::Server(Json("a server error occurred".to_owned())),
            ServiceError::PermissionError(msg) => Self::Client(Json(msg)),
        }
    }
}

// Allow [`ApiKey`] to be used as a [request guard](https://rocket.rs/v0.5-rc/guide/requests/#request-guards) in a route.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        //
        // inner functions, used internally multiple times
        //

        fn server_error() -> Outcome<ApiKey, ApiError> {
            Outcome::Failure((
                Status::InternalServerError,
                ApiError::Server(Json("server error".to_string())),
            ))
        }
        fn key_error(e: ApiKeyError) -> Outcome<ApiKey, ApiError> {
            Outcome::Failure((Status::BadRequest, ApiError::KeyError(Json(e))))
        }

        match req.headers().get_one(API_KEY_HEADER) {
            None => key_error(ApiKeyError::NotFound(
                "API key header not found".to_string(),
            )),
            Some(key) => {
                let db = match req.guard::<&State<AppDatabase>>().await {
                    Outcome::Success(db) => db,
                    _ => return server_error(),
                };
                let api_key = match ApiKey::from_str(key) {
                    Ok(key) => key,
                    Err(e) => return key_error(e),
                };

                match service::is_api_key_valid(api_key.clone(), db.get_pool()).await {
                    Ok(valid) if valid => Outcome::Success(api_key),
                    Ok(valid) if !valid => {
                        key_error(ApiKeyError::NotFound("API key not found".to_owned()))
                    }
                    _ => server_error(),
                }
            }
        }
    }
}

#[rocket::get("/key")]
pub async fn new_api_key(database: &State<AppDatabase>) -> Result<Json<&str>, ApiError> {
    let api_key = service::generate_api_key(database.get_pool()).await?;
    println!("ApiKey: {}", api_key.to_base64());
    Ok(Json("Api Key generated. See logs for details"))
}

#[rocket::get("/<shortcode>")]
pub async fn get_clip(
    shortcode: &str,
    database: &State<AppDatabase>,
    cookies: &CookieJar<'_>,
    hit_counter: &State<HitCounter>,
    _api_key: ApiKey,
) -> Result<Json<Clip>, ApiError> {
    let req = service::GetClip {
        shortcode: shortcode.into(),
        password: cookies
            .get(PASSWORD_COOKIE)
            .map(|cookie| cookie.value())
            .map(|raw_pwd| Password::new(raw_pwd.to_string()).ok())
            .flatten()
            .unwrap_or_else(Password::default),
    };

    let clip = service::get_clip(req, database.get_pool()).await?;
    hit_counter.hit(shortcode.into(), 1);
    Ok(Json(clip))
}

#[rocket::post("/", data = "<req>")]
pub async fn new_clip(
    req: Json<service::NewClip>,
    db: &State<AppDatabase>,
    _api_key: ApiKey,
) -> Result<Json<Clip>, ApiError> {
    let clip = service::new_clip(req.into_inner(), db.get_pool()).await?;
    Ok(Json(clip))
}

#[rocket::put("/", data = "<req>")]
pub async fn update_clip(
    req: Json<service::UpdateClip>,
    db: &State<AppDatabase>,
    _api_key: ApiKey,
) -> Result<Json<Clip>, ApiError> {
    let clip = service::update_clip(req.into_inner(), db.get_pool()).await?;
    Ok(Json(clip))
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![new_api_key, get_clip, new_clip, update_clip]
}

pub mod catcher {
    use rocket::serde::json::Json;
    use rocket::Request;
    use rocket::{catch, catchers, Catcher};

    #[catch(default)]
    fn default(req: &Request) -> Json<&'static str> {
        eprintln!("General error: {:?}", req);
        Json("something went wrong")
    }

    #[catch(500)]
    fn internal_error(req: &Request) -> Json<&'static str> {
        eprintln!("Internal error: {:?}", req);
        Json("internal server error")
    }

    #[catch(400)]
    fn missing_api_key() -> Json<&'static str> {
        Json("missing or invalid API key")
    }

    #[catch(401)]
    fn request_error() -> Json<&'static str> {
        Json("request error")
    }

    #[catch(404)]
    fn not_found() -> Json<&'static str> {
        Json("404")
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![
            default,
            internal_error,
            missing_api_key,
            request_error,
            not_found
        ]
    }
}
