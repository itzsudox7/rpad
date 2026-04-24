// rpad - a notepad written in rust and egui
mod file;
mod search;
mod settings;
mod shortcuts;
mod theme;
mod ui;

use eframe::egui;
use file::FileState;
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
    }
}