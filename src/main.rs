#![windows_subsystem = "windows"]
slint::include_modules!();
use std::time::Duration;
use std::{fs, thread};
use std::io::Write;

use slint::SharedString;
use webbrowser;

const FILENAME_LINKS: &str = "links.txt";

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_request_open_links(|urls_text: SharedString| {
        let urls: Vec<&str> = urls_text.split("\n").collect();

        for url in urls {
            thread::sleep(Duration::from_millis(100));
            if let Err(err) = webbrowser::open(&url) {
                eprintln!("Failed to open {}: {}", url, err);
            }
        }
        save_to_file(&FILENAME_LINKS, &urls_text).unwrap();
    });

  
        let contents = fs::read_to_string(&FILENAME_LINKS).unwrap_or_else(|_| String::from(""));
        let ui = ui_handle.unwrap();
        ui.set_links_text(SharedString::from(contents));
  

    ui.run()
}

fn save_to_file(file_path: &str, content: &str) -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    file.write_all(content.as_bytes())?;
    Ok(())
}
