#[allow(unused_imports)]
use crate::app::update; // Corrected import

pub fn handle_update_command() {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        if let Err(e) = update::update_to_latest() {
            eprintln!("Update failed: {}", e);
        } else {
            println!("Update completed successfully.");
        }
    }

    #[cfg(target_os = "windows")]
    {
        println!("Please download and run the latest `windows_installer.exe` to update the application.");
    }
}

