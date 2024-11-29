use std::fs;
use std::env;
use reqwest;
use serde_json::Value;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt; // Import PermissionsExt for Unix systems

pub fn update_to_latest() -> Result<(), Box<dyn std::error::Error>> {
    let repo_url = "https://api.github.com/repos/tparnell96/catalysh/releases/latest";
    let client = reqwest::blocking::Client::new();
    let response = client.get(repo_url).header("User-Agent", "Rust-App").send()?;

    if !response.status().is_success() {
        return Err("Failed to fetch release information.".into());
    }

    let json: Value = response.json()?;
    let assets = json["assets"].as_array().ok_or("Invalid assets structure")?;
    let platform = if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    };

    let architecture = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        "unknown"
    };

    let asset = assets.iter().find(|asset| {
        let name = asset["name"].as_str().unwrap_or("");
        name.contains(platform) && name.contains(architecture)
    }).ok_or("No compatible asset found")?;

    let download_url = asset["browser_download_url"].as_str().ok_or("Invalid download URL")?;
    let temp_file = env::temp_dir().join(asset["name"].as_str().unwrap_or("app_update"));

    println!("Downloading update...");
    let mut file = fs::File::create(&temp_file)?;
    let mut response = client.get(download_url).send()?;
    response.copy_to(&mut file)?;
    println!("Download complete: {}", temp_file.display());

    let local_bin = dirs::home_dir()
        .ok_or("Failed to determine home directory")?
        .join(".local/bin");

    if !local_bin.exists() {
        fs::create_dir_all(&local_bin)?;
    }

    let destination = local_bin.join("catalysh");

    println!("Moving binary to ~/.local/bin...");
    fs::copy(&temp_file, &destination)?;

    #[cfg(unix)]
    {
        println!("Applying executable permissions...");
        fs::set_permissions(&destination, fs::Permissions::from_mode(0o755))?;
    }

    println!("Update successfully applied to ~/.local/bin/catalysh.");
    Ok(())
}

