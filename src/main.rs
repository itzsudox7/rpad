use eframe::egui;
use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;

#[derive(Default)]
struct MyApp {
    text: String,
    current_file: Option<PathBuf>,
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Rpad",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.ctx().set_visuals(egui::Visuals::dark());

        // topbar
        egui::Panel::top("menu").show_inside(ui, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if ui.button("New").clicked() {
                    self.text.clear();
                    self.current_file = None;
                }

                if ui.button("Open").clicked() {
                    if let Some(path) = FileDialog::new().pick_file() {
                        if let Ok(content) = fs::read_to_string(&path) {
                            self.text = content;
                            self.current_file = Some(path);
                        }
                    }
                }

                if ui.button("Save").clicked() {
                    if let Some(path) = &self.current_file {
                        let _ = fs::write(path, &self.text);
                    } else {
                        if let Some(path) = FileDialog::new().save_file() {
                            let _ = fs::write(&path, &self.text);
                            self.current_file = Some(path);
                        }
                    }
                }

                ui.separator();

                // show current file
                let name = self
                    .current_file
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or("Untitled".to_string());

                ui.label(format!("{}", name));
            });

            ui.add_space(4.0);
        });

        // status bar
        egui::Panel::bottom("status").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Length: {}", self.text.len()));

                ui.separator();

                ui.label(format!("Lines: {}", self.text.lines().count()));
            });
        });

        // editor
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.add_space(8.0);

            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.text)
                    .desired_rows(25)
                    .lock_focus(true),
            );
        });
    }
}
