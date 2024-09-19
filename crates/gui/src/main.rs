use native_dialog::{FileDialog, MessageDialog, MessageType};
use std::path::PathBuf;

fn open_file() -> Option<PathBuf> {
    FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("Text Files", &["txt"])
        .add_filter("All Files", &["*"])
        .show_open_single_file()
        .unwrap()
}

fn save_file() -> Option<PathBuf> {
    FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("Text Files", &["txt"])
        .show_save_single_file()
        .unwrap()
}

fn show_message(message: &str) {
    MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Information")
        .set_text(message)
        .show_alert()
        .unwrap();
}

fn main() {
    if let Some(path) = open_file() {
        show_message(&format!("Selected file: {:?}", path));
    }

    if let Some(path) = save_file() {
        show_message(&format!("File will be saved to: {:?}", path));
    }
}
