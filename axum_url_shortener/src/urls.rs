use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use lazy_static::lazy_static;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::sync::Mutex;

lazy_static! {
    static ref URL_MAPPING: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn encode_short_url(long_url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(long_url.to_owned().as_bytes());
    let result = hasher.finalize();

    let b64 = URL_SAFE_NO_PAD.encode(result);
    let short_url = b64[..16].to_string();

    URL_MAPPING
        .lock()
        .unwrap()
        .insert(short_url.clone(), long_url.to_string());

    short_url
}

pub fn decode_long_url(short_url: &str) -> Result<String, Error> {
    print_url_mapping();

    let long_url: Option<String> = URL_MAPPING.lock().unwrap().get(short_url).cloned();

    long_url.ok_or_else(|| Error::new(ErrorKind::NotFound, "Short URL not found".to_string()))
}

pub fn print_url_mapping() {
    tracing::debug!("URL_MAPPING: {:?}", URL_MAPPING.lock().unwrap());
}
