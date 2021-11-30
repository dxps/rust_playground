use base64::{decode, encode};
use image::{load_from_memory, ImageOutputFormat::Png};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&format!("encoded image is {} chars long", encoded_file.len()).into());
    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"decoded the base64 to binary".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"loaded the image from binary".into());

    img = img.grayscale();
    log(&"applied grayscale effect".into());

    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"written new image".into());

    let encoded_img = encode(&buffer);
    let img_data_url = format!("data:image/png;base64,{}", encoded_img);

    img_data_url
}
