// settings - editor display settings

/// editor settings
pub struct Settings {
    pub show_line_numbers: bool,
    pub word_wrap: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_line_numbers: true,
            word_wrap: false,
        }
    }
}