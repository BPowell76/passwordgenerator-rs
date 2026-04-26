#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod password;

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
    output: String,
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self {
            password_length: 8,
            use_spec_char: true,
            output: "".to_string(),
        }
    }
}

impl eframe::App for PasswordGenerator {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
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
                    self.output = password::create_password(self.use_spec_char, self.password_length);
                }

                ui.add(egui::TextEdit::singleline(&mut self.output)
                    .desired_width(300.0)
                    .char_limit(24)
                    .horizontal_align(egui::Align::Center)
                    .font(FontId::monospace(18.0)));
            });
        });
    }
}