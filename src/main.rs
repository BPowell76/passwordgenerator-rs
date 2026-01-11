#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::rc::Rc;
use arboard::{
    Clipboard
};
use rand::Rng;
use eframe::egui;
use egui::style::HandleShape;
use egui::FontId;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 200.0]),
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
    use_spec_char: bool,
    password: String,
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self {
            password_length: 8,
            use_spec_char: true,
            password: "".to_string(),
        }
    }
}

impl eframe::App for PasswordGenerator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 20.0;

            ui.vertical_centered(|ui| {
                ui.label("To generate a password, adjust the slider to the desired password length and click on the Generate Password button. If you want to disable special characters, uncheck the option.");
                ui.horizontal(|ui| {
                    ui.add(egui::Slider::new(&mut self.password_length, 8..=24)
                        .handle_shape(HandleShape::Circle)
                        .trailing_fill(true)
                        .text("Password Length"));

                    ui.add(egui::Checkbox::new(&mut self.use_spec_char, "Use Special Characters"));
                });

                if ui.button("Generate Password").clicked() {
                    let mut rng = rand::rng();
                    let mut character: char;
                    let mut password_tmp_string: String = String::new();
                    let mut counter: u8 = 0;

                    if self.use_spec_char {
                        let special_character_vec: Vec<u32> = build_spec_char_vec();
                        let vec_length: u8 = special_character_vec.len() as u8;

                        while counter < self.password_length {
                            let index:u32 = (&mut rng).random_range(..vec_length) as u32;
                            character = char::from_u32(special_character_vec[index as usize]).unwrap();
                            password_tmp_string = password_tmp_string + character.to_string().as_str();
                            counter += 1;
                        }
                    }
                    else {
                        let character_vec: Vec<u32> = build_char_vec();
                        let vec_length: u8 = character_vec.len() as u8;

                        while counter < self.password_length {
                            let index = rng.random_range(..vec_length) as u32;
                            character = char::from_u32(character_vec[index as usize]).unwrap();
                            password_tmp_string = password_tmp_string + character.to_string().as_str();
                            counter += 1;
                        }
                    }

                    self.password = password_tmp_string;
                }

                ui.add(egui::TextEdit::singleline(&mut self.password)
                    .desired_width(300.0)
                    .char_limit(24)
                    .horizontal_align(egui::Align::Center)
                    .font(FontId::monospace(18.0)));
            });
        });
    }
}

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