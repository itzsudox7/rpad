// file - handles reading, writing, and managing files
use std::fs;
use std::path::PathBuf;

// shows if a file has been saved or opened or if there was an error
#[derive(Default, Clone, PartialEq)]
pub enum FileStatus {
    #[default]
    None,
    Saved,
    Opened,
    Error(String),
}

pub struct FileState {
    pub current_file: Option<PathBuf>,
    pub text: String,
    pub last_saved_text: String,
    pub dirty: bool,
    pub recent_files: Vec<PathBuf>,
    pub status: FileStatus,
}

// creates a new empty file state
impl Default for FileState {
    fn default() -> Self {
        Self {
            current_file: None,
            text: String::new(),
            last_saved_text: String::new(),
            dirty: false,
            recent_files: Vec::new(),
            status: FileStatus::None,
        }
    }
}

// file operations like open, save, and new
impl FileState {
    // open a file from disk
    pub fn open(&mut self, path: PathBuf) -> FileStatus {
        match fs::read_to_string(&path) {
            Ok(content) => {
                self.text = content.clone();
                self.last_saved_text = content;
                self.dirty = false;
                if !self.recent_files.contains(&path) {
                    self.recent_files.insert(0, path.clone());
                    if self.recent_files.len() > 10 {
                        self.recent_files.pop();
                    }
                }
                self.current_file = Some(path);
                FileStatus::Opened
            }
            Err(_) => FileStatus::Error("Failed to open file".into()),
        }
    }

    // save the current text to the open file
    pub fn save(&mut self) -> FileStatus {
        if let Some(path) = &self.current_file {
            return match fs::write(path, &self.text) {
                Ok(_) => {
                    self.last_saved_text = self.text.clone();
                    self.dirty = false;
                    FileStatus::Saved
                }
                Err(_) => FileStatus::Error("Failed to save".into()),
            };
        }
        self.save_as()
    }

    // save the current text to a new file
    pub fn save_as(&mut self) -> FileStatus {
        if let Some(path) = rfd::FileDialog::new().save_file() {
            return match fs::write(&path, &self.text) {
                Ok(_) => {
                    self.last_saved_text = self.text.clone();
                    self.dirty = false;
                    self.current_file = Some(path);
                    FileStatus::Saved
                }
                Err(_) => FileStatus::Error("Failed to save".into()),
            };
        }
        FileStatus::None
    }

    // create a new empty file
    pub fn new_file(&mut self) {
        self.text.clear();
        self.last_saved_text.clear();
        self.current_file = None;
        self.dirty = false;
    }

    // check if the file has unsaved changes
    pub fn is_dirty(&self) -> bool {
        self.text != self.last_saved_text
    }

    // get the file name to show in the title bar
    pub fn file_name(&self) -> String {
        self.current_file
            .as_ref()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| "Untitled".to_string())
    }
}