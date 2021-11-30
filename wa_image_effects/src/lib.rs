use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_2 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) {
    log(&"encoded_file=".into(), &encoded_file.into())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
