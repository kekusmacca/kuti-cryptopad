// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod crypto;

use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};
use crypto::{encrypt, decrypt};
//use native_dialog::FileDialog;


slint::include_modules!();


fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_save_as_dialog({
        let ui_handle = ui.as_weak();
        move |passkey| {
            let ui = ui_handle.unwrap();
            
            // Get the path to save the file using rfd
            let path = rfd::FileDialog::new()
                .set_directory("~/Desktop")
                .add_filter("Text File", &["txt"])
                .save_file();

            if let Some(path) = path {
                // The data we want to save
                let new_contents = ui.get_contents();  //.clone().to_string();

                // Encrypt the file
                let encrypted_new_contents = encrypt(&new_contents, &passkey);

                // Attempt to save the file
                if let Err(e) = write_file(path.to_str().unwrap(), &encrypted_new_contents) {
                    eprintln!("Failed to save file: {}", e);
                }
            } else {
                eprintln!("Failed to get the path to save the file.");
            }
        }
    });

    ui.on_save_current({
        let ui_handle = ui.as_weak();
        move |passkey| {
            let ui = ui_handle.unwrap();
            
             // The data you want to save
            let path = ui.get_file_path();
            
            // let new_contents = string.trim();
            let new_contents = ui.get_contents();

            // Encrypt and save the file
            let encrypted_new_contents = encrypt(&new_contents, &passkey);

            // Attempt to save the file
            if let Err(e) = write_file(&path, &encrypted_new_contents) {
                eprintln!("Failed to save file: {}", e);
            } else {
                ui.set_status(slint::SharedString::from("File saved"));
            }

            // Refresh the UI
            ui.set_contents(slint::SharedString::from(new_contents));
            
        }
    });

    
    ui.on_open_file_dialog({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            // Open the file dialog using rfd
            let path = rfd::FileDialog::new()
                .set_directory("~/Desktop")
                .add_filter("Text File", &["txt"])
                .pick_file();

            // This should only open the dialog and set the path for the UI, decryption is done next.
            if let Some(path) = path {
                // Update the UI with file path
                ui.set_file_path(slint::SharedString::from(path.display().to_string()));

                // invoke popup window asking for passkey
                let handle_copy = ui_handle.clone();
                let _ = slint::invoke_from_event_loop(move || handle_copy.unwrap().invoke_show_popup_open());
            } else {
                eprintln!("Failed to open file dialog.");
            }
        }
    });

    ui.on_decrypt_contents({
        let ui_handle = ui.as_weak();
        move |passkey| {
            let ui = ui_handle.unwrap();

            // decrypt the contents
            let path = ui.get_file_path();
            let str_path = path.to_string();

            // Read the encrypted contents from the file
            let encrypted_contents = read_file(&str_path).unwrap();

            // Try to decrypt the contents
            match decrypt(&encrypted_contents, &passkey) {
                Ok(decrypted_contents) => {
                    // Handle the decrypted contents
                    // Update the UI with the decrypted contents
                    ui.set_contents(slint::SharedString::from(decrypted_contents));
                    ui.set_bg_color(slint::Brush::from(slint::Color::from_rgb_u8(0, 63, 0)));
                    ui.set_status(slint::SharedString::from("Opened"));
                }
                Err(e) => {
                    // Handle the error
                    ui.set_contents(slint::SharedString::from("ERROR: -  Failed to decrypt contents - \n Please try again"));
                    ui.set_bg_color(slint::Brush::from(slint::Color::from_rgb_u8(63, 0, 0)));
                    ui.set_status(slint::SharedString::from("Error"));

                    eprintln!("ERROR: -  Failed to decrypt contents -  {:?}", e);
                    
                }
            }
        }
    });

    // Run the UI
    ui.run()?;

    Ok(())
}




// TODO: add file error handling
// Functions to read and write files
fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_file(path: &str, contents: &str) -> io::Result<()> {

    // Attempt to create and write to the file
    match File::create(&path) {
        Ok(mut file) => match file.write_all(contents.as_bytes()) {
            Ok(_) => {
                //println!("File saved successfully at {:?}", path);
                println!("File saved successfully");
            }
            Err(e) => {
                eprintln!("Failed to write to file: {}", e);
            }
        },
        Err(e) => {
            eprintln!("Failed to create file: {}", e);
        }
    }

    Ok(())
}
