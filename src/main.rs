// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rfd;
use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    ui.on_load_button_clicked({
        let ui_handle = ui.as_weak();
        move || {
            let folder = rfd::FileDialog::new()
                .set_title("Select a directory")
                .set_directory("/")
                .pick_folder();
            let ui = ui_handle.unwrap();
            if let Some(path) = folder {
                let path = path.to_str();
                ui.set_test(Into::into(path.unwrap_or("No Path selected!")));
            }
        }
    });

    ui.run()?;

    Ok(())
}

fn show_open_dialog() {

}
