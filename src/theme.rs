// theme - colors and styles for different editor themes

/// available themes
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum Theme {
    #[default]
    Dark,
    Light,
    Monokai,
    Solarized,
    Nord,
}

// theme colors and helpers
impl Theme {
    // get list of all available themes
    pub fn all() -> Vec<Theme> {
        vec![Theme::Dark, Theme::Light, Theme::Monokai, Theme::Solarized, Theme::Nord]
    }

    // get the name to show in menus
    pub fn label(&self) -> &'static str {
        match self {
            Theme::Dark => "Dark",
            Theme::Light => "Light",
            Theme::Monokai => "Monokai",
            Theme::Solarized => "Solarized",
            Theme::Nord => "Nord",
        }
    }

    // returns (dot_char, dot_color) for status bar / menu
    pub fn dot(&self) -> (&'static str, eframe::egui::Color32) {
        match self {
            Theme::Dark => ("*", eframe::egui::Color32::from_rgb(82, 139, 255)),
            Theme::Light => ("*", eframe::egui::Color32::from_rgb(60, 110, 220)),
            Theme::Monokai => ("*", eframe::egui::Color32::from_rgb(249, 38, 114)),
            Theme::Solarized => ("*", eframe::egui::Color32::from_rgb(38, 139, 210)),
            Theme::Nord => ("*", eframe::egui::Color32::from_rgb(136, 192, 208)),
        }
    }

    // get the accent color for buttons and highlights
    pub fn accent_color(&self) -> eframe::egui::Color32 {
        match self {
            Theme::Dark => eframe::egui::Color32::from_rgb(82, 139, 255),
            Theme::Light => eframe::egui::Color32::from_rgb(60, 110, 220),
            Theme::Monokai => eframe::egui::Color32::from_rgb(249, 38, 114),
            Theme::Solarized => eframe::egui::Color32::from_rgb(38, 139, 210),
            Theme::Nord => eframe::egui::Color32::from_rgb(136, 192, 208),
        }
    }

    // get the background color for the editor
    pub fn bg_color(&self) -> eframe::egui::Color32 {
        match self {
            Theme::Dark => eframe::egui::Color32::from_rgb(24, 24, 27),
            Theme::Light => eframe::egui::Color32::from_rgb(248, 248, 250),
            Theme::Monokai => eframe::egui::Color32::from_rgb(39, 40, 34),
            Theme::Solarized => eframe::egui::Color32::from_rgb(0, 43, 54),
            Theme::Nord => eframe::egui::Color32::from_rgb(46, 52, 64),
        }
    }

    // get the background color for the top bar
    pub fn topbar_bg(&self) -> eframe::egui::Color32 {
        match self {
            Theme::Dark => eframe::egui::Color32::from_rgb(18, 18, 22),
            Theme::Light => eframe::egui::Color32::from_rgb(235, 235, 242),
            Theme::Monokai => eframe::egui::Color32::from_rgb(30, 31, 26),
            Theme::Solarized => eframe::egui::Color32::from_rgb(0, 33, 43),
            Theme::Nord => eframe::egui::Color32::from_rgb(36, 41, 51),
        }
    }

    // get the main text color
    pub fn text_color(&self) -> eframe::egui::Color32 {
        match self {
            Theme::Dark => eframe::egui::Color32::from_rgb(212, 212, 212),
            Theme::Light => eframe::egui::Color32::from_rgb(30, 30, 40),
            Theme::Monokai => eframe::egui::Color32::from_rgb(248, 248, 242),
            Theme::Solarized => eframe::egui::Color32::from_rgb(131, 148, 150),
            Theme::Nord => eframe::egui::Color32::from_rgb(236, 239, 244),
        }
    }

    // get the muted color for secondary text
    pub fn muted_color(&self) -> eframe::egui::Color32 {
        match self {
            Theme::Dark => eframe::egui::Color32::from_rgb(90, 90, 100),
            Theme::Light => eframe::egui::Color32::from_rgb(160, 160, 175),
            Theme::Monokai => eframe::egui::Color32::from_rgb(117, 113, 94),
            Theme::Solarized => eframe::egui::Color32::from_rgb(88, 110, 117),
            Theme::Nord => eframe::egui::Color32::from_rgb(76, 86, 106),
        }
    }
}