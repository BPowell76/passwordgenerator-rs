#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::rc::Rc;
use arboard::{
    Clipboard
};
use rand::Rng;
use eframe::egui;
use egui::style::HandleShape;

fn build_spec_char_vec() -> Vec<u32> {
    let mut vector: Vec<u32> = Vec::new();
    let mut counter: u32 = 33;
    while counter < 127 {
        if counter == 34 {
            counter += 1;
            continue;
        }
        vector.push(counter);
        counter += 1;
    }

    return vector;
}

fn build_char_vec() -> Vec<u32> {
    let mut vector: Vec<u32> = Vec::new();
    let mut counter: u32 = 48;
    while counter < 123 {
        if counter == 58
            || counter == 59
            || counter == 60
            || counter == 61
            || counter == 62
            || counter == 63
            || counter == 64
            || counter == 91
            || counter == 92
            || counter == 93
            || counter == 94
            || counter == 95
            || counter == 96 {
            counter += 1;
            continue;
        }
        vector.push(counter);
        counter += 1;
    }
    return vector;
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Password Generator",
        options,
        Box::new(|_ctx| Ok(Box::new(PasswordGenerator::default()))),
    )
}

struct PasswordGenerator {
    password_length: u8,
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self {
            password_length: 8,
        }
    }
}

impl eframe::App for PasswordGenerator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.password_length, 8..=24)
                .handle_shape(HandleShape::Circle)
                .trailing_fill(true)
                .text("Password Length"));
        });
    }
}

/*
    main_window.on_generate_password( move || {
        let main = main_window_weak_password_gen.upgrade().unwrap();
        let password_length: u8 = main.get_password_length() as u8;
        let special_characters: bool = main.get_use_special_characters();
        let mut password: String = "".to_string();
        let mut counter: u8 = 0;
        let mut rng = rand::rng();
        let mut character: String;

        if special_characters {
            let special_character_vec: Vec<u32> = build_spec_char_vec();
            let vec_length: u8 = special_character_vec.len() as u8;

            while counter < password_length {
                let index = rng.random_range(..vec_length) as u32;
                character = char::from_u32(special_character_vec[index as usize]).unwrap().to_string();
                password = password + character.as_str();
                counter += 1;
            }
        }
        else {
            let character_vec: Vec<u32> = build_char_vec();
            let vec_length: u8 = character_vec.len() as u8;

            while counter < password_length {
                let index = rng.random_range(..vec_length) as u32;
                character = char::from_u32(character_vec[index as usize]).unwrap().to_string();
                password = password + character.as_str();
                counter += 1;
            }
        }

        main.set_password(password.to_shared_string());
    });

    main_window.on_copy_to_clipboard(move || {
        let main = main_window_weak_copy_clipboard.upgrade().unwrap();
        let mut clip = Clipboard::new().unwrap();
        let password_text: String = main.get_password().to_string();
        clip.set_text(password_text);
        println!("{:?}", clip.get_text());
    });

    main_window.run().unwrap();
     */
