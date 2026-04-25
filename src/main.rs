// rpad - a notepad written in rust and egui
mod file;
mod search;
mod settings;
mod shortcuts;
mod theme;
mod ui;

use eframe::egui;
use file::{FileState, FileStatus};
use search::SearchState;
use settings::Settings;
use theme::Theme;

pub struct MyApp {
    file: FileState,
    settings: Settings,
    theme: Theme,
    search: SearchState,
    show_search: bool,
    show_minimap: bool,
    cursor_line: usize,
    cursor_col: usize,
    font_size: f32,
    tab_size: usize,
    show_close_confirm: bool,
    allow_close: bool,
}

// creates the app with default settings
impl Default for MyApp {
    fn default() -> Self {
        Self {
            file: FileState::default(),
            settings: Settings::default(),
            theme: Theme::Dark,
            search: SearchState::default(),
            show_search: false,
            show_minimap: false,
            cursor_line: 1,
            cursor_col: 1,
            font_size: 14.0,
            tab_size: 4,
            show_close_confirm: false,
            allow_close: false,
        }
    }
}

// starts the editor window
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 700.0])
            .with_min_inner_size([600.0, 400.0])
            .with_title("Rpad"),
        ..Default::default()
    };

    eframe::run_native(
        "Rpad",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

// draws the editor ui
impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.file.dirty = self.file.is_dirty();
        let ctx = ui.ctx().clone();

        let close_requested = ctx.input(|i| i.viewport().close_requested());
        if close_requested && self.file.is_dirty() && !self.allow_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.show_close_confirm = true;
        }

        ui::UiState::apply_theme(ui.ctx(), self.theme, self.font_size);
        shortcuts::Shortcuts::handle(ui, &mut self.file, &mut self.show_search, &mut self.font_size);

        ui::UiState::topbar(
            ui,
            &mut self.file,
            &mut self.theme,
            &mut self.settings,
            &mut self.show_search,
            &mut self.show_minimap,
        );

        ui::UiState::statusbar(
            ui,
            self.theme,
            &self.file.text,
            self.cursor_line,
            self.cursor_col,
            self.tab_size,
        );

        if self.show_search {
            ui::UiState::searchbar(
                ui,
                self.theme,
                &mut self.search,
                &mut self.file.text,
                &mut self.show_search,
            );
        }

        ui::UiState::editor(
            ui,
            self.theme,
            &self.settings,
            &mut self.file.text,
            self.show_minimap,
            self.font_size,
            &mut (self.cursor_line, self.cursor_col),
        );

        if self.show_close_confirm {
            let screen_rect = ctx.content_rect();
            let painter = ctx.layer_painter(egui::LayerId::new(
                egui::Order::Middle,
                egui::Id::new("close_confirm_overlay"),
            ));
            painter.rect_filled(
                screen_rect,
                0.0,
                egui::Color32::from_rgba_premultiplied(0, 0, 0, 140),
            );

            let mut save_clicked = false;
            let mut dont_save_clicked = false;
            let mut cancel_clicked = false;

            egui::Window::new("Unsaved changes")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .collapsible(false)
                .resizable(false)
                .title_bar(true)
                .fixed_size(egui::vec2(430.0, 132.0))
                .frame(
                    egui::Frame::window(&ctx.global_style())
                        .inner_margin(egui::Margin::same(12))
                        .corner_radius(egui::CornerRadius::same(8)),
                )
                .show(&ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("!")
                                    .color(egui::Color32::from_rgb(255, 190, 90))
                                    .size(20.0),
                            );
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new("You have unsaved changes.")
                                    .strong()
                                    .size(15.0),
                            );
                        });

                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(
                                "Save your changes before closing the editor?",
                            )
                            .color(egui::Color32::from_rgb(195, 195, 205))
                            .size(13.0),
                        );

                        ui.add_space(6.0);
                        ui.separator();
                        ui.add_space(4.0);

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let cancel_btn =
                                egui::Button::new(egui::RichText::new("Cancel").size(12.5))
                                    .min_size(egui::vec2(90.0, 27.0));
                            cancel_clicked = ui.add(cancel_btn).clicked();

                            let dont_save_btn = egui::Button::new(
                                egui::RichText::new("Don't Save")
                                    .color(egui::Color32::from_rgb(255, 120, 120))
                                    .size(12.5),
                            )
                            .min_size(egui::vec2(110.0, 27.0));
                            dont_save_clicked = ui.add(dont_save_btn).clicked();

                            let save_btn =
                                egui::Button::new(egui::RichText::new("Save").strong().size(12.5))
                                    .fill(self.theme.accent_color())
                                    .stroke(egui::Stroke::NONE)
                                    .min_size(egui::vec2(90.0, 27.0));
                            save_clicked = ui.add(save_btn).clicked();
                        });
                    });
                });

            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                cancel_clicked = true;
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                save_clicked = true;
            }

            if cancel_clicked {
                self.show_close_confirm = false;
                self.allow_close = false;
            } else if dont_save_clicked {
                self.show_close_confirm = false;
                self.allow_close = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            } else if save_clicked {
                let status = self.file.save();
                self.file.status = status.clone();
                if matches!(status, FileStatus::Saved) {
                    self.show_close_confirm = false;
                    self.allow_close = true;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        }
    }
}