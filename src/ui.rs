use std::fs::File;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::Command;

use rfd::*;

pub static MAX_DISPLAY_ROWS: usize = 43;

/// Opens a file dialog and returns the path to the selected csv file.
pub fn get_csv() -> Option<PathBuf> {
    FileDialog::new()
        .set_title("Choose a csv file")
        .add_filter("CSV File", &["csv"])
        .pick_file()
}

/// Opens a save file dialog and returns the path to the selected file and a file handle.
pub fn save_file(default: &str) -> Option<(PathBuf, File)> {
    let file_path = FileDialog::new()
        .set_title("Save a file")
        .set_file_name(default)
        .set_can_create_directories(true)
        .save_file()?;

    let file = File::create(&file_path).ok()?;
    Some((file_path, file))
}

/// Displays a dialog with the given title and message.
pub fn display_dialog(title: &str, message: &str) {
    let rows = message.split('\n').collect::<Vec<_>>();
    let mut rows_iter = rows.iter();

    let mut messages = Vec::new();

    // The message is split into multiple dialogs if it exceeds the maximum number of rows.
    // This is to avoid a quirk where the dialog box would freeze if the message was too long.
    for _ in 0..(rows.len() / MAX_DISPLAY_ROWS + 1) {
        let message = rows_iter
            .by_ref()
            .take(MAX_DISPLAY_ROWS)
            .map(Deref::deref)
            .collect::<Vec<_>>()
            .join("\n");

        messages.push(message);
    }

    // Display all the dialogs.
    for message in messages {
        Command::new("osascript")
            .arg("-e")
            .arg(format!(
                "display dialog \"{}\" with title \"{}\" buttons {{\"OK\"}} default button \"OK\"",
                message, title
            ))
            .output()
            .expect("failed to execute process");
    }
}

// Displays a dialog with the given title and message and returns true if the user clicked "Yes".
pub fn display_option(title: &str, message: &str) -> bool {
    let output = Command::new("osascript")
         .arg("-e")
         .arg(format!(
             "display dialog \"{}\" with title \"{}\" buttons {{\"Yes\", \"No\"}} default button \"No\"",
             message, title
         ))
         .output()
         .expect("failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);
    output.contains("Yes")
}
