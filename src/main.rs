#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
slint::include_modules!();
fn main() {
    let main_window = MainWindow::new().unwrap();
    let main_window_weak = main_window.as_weak();

    main_window.run();
}
