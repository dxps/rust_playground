// Prevent console window in addition to Slint window in Windows release builds when,
// e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    main_window.run()
}
