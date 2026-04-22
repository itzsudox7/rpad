use std::fmt::format;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Rpad",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    text: String,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // topbar
        egui::Panel::top("menu").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button("New").clicked() {
                    self.text.clear();
                }

                if ui.button("Open").clicked() {
                    println!("Open clicked");
                }

                if ui.button("Save").clicked() {
                    println!("Save clicked");
                }
            });
        });

        // status bar
        egui::Panel::bottom("status").show_inside(ui, |ui| {
            ui.label(format!("Length: {}", self.text.len()));
        });

        // editor
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut self.text)
                    .desired_rows(25)
                    .lock_focus(true),
            );
        });
    }
}
