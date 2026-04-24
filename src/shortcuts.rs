// shortcuts - handles keyboard shortcuts for the editor
use eframe::egui;

pub struct Shortcuts;

// keyboard handling for ctrl+s, ctrl+o, ctrl+n, etc
impl Shortcuts {
    pub fn handle(
        ui: &egui::Ui,
        file_state: &mut crate::file::FileState,
        show_search: &mut bool,
        font_size: &mut f32,
    ) -> bool {
        let ctrl = ui.input(|i| i.modifiers.ctrl || i.modifiers.command);

        if ctrl {
            if ui.input(|i| i.key_pressed(egui::Key::S)) {
                if ui.input(|i| i.modifiers.shift) {
                    file_state.save_as();
                } else {
                    file_state.save();
                }
                return true;
            }

            if ui.input(|i| i.key_pressed(egui::Key::O)) {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    file_state.open(path);
                }
                return true;
            }

            if ui.input(|i| i.key_pressed(egui::Key::N)) {
                file_state.new_file();
                return true;
            }

            if ui.input(|i| i.key_pressed(egui::Key::F)) {
                *show_search = !*show_search;
                return true;
            }

            if ui.input(|i| i.key_pressed(egui::Key::Plus) || i.key_pressed(egui::Key::Equals)) {
                *font_size = (*font_size + 1.0).min(32.0);
                return true;
            }

            if ui.input(|i| i.key_pressed(egui::Key::Minus)) {
                *font_size = (*font_size - 1.0).max(8.0);
                return true;
            }
        }

        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            *show_search = false;
            return true;
        }

        false
    }
}