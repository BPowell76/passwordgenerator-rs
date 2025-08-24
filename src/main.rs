#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::private_unstable_api::re_exports::Float;
use slint::ToSharedString;

slint::include_modules!();
fn main() {
    let main_window = MainWindow::new().unwrap();
    let main_window_weak = main_window.as_weak();

    main_window.on_generate_password( move || {
        let password_length: u8 = main_window_weak.unwrap().get_password_length() as u8;
        let special_characters: bool = main_window_weak.unwrap().get_use_special_characters();

        let mut password: &str = "";

        main_window_weak.unwrap().set_password(password.to_shared_string());
    });

    main_window.run();
}
