use std::fs;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use reqwest;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo_url = "https://api.github.com/repos/tparnell96/catsh/releases/latest";
    let client = reqwest::blocking::Client::new();
    let response = client.get(repo_url).header("User-Agent", "Rust-App").send()?;

    if !response.status().is_success() {
        return Err("Failed to fetch release information.".into());
    }

    let json: Value = response.json()?;
    let assets = json["assets"].as_array().ok_or("Invalid assets structure")?;
    let asset = assets.iter().find(|asset| {
        let name = asset["name"].as_str().unwrap_or("");
        name.ends_with(".exe")
    }).ok_or("No compatible asset found")?;

    let download_url = asset["browser_download_url"].as_str().ok_or("Invalid download URL")?;
    let temp_file = env::temp_dir().join(asset["name"].as_str().unwrap_or("catsh_update.exe"));

    println!("Downloading update...");
    let mut file = fs::File::create(&temp_file)?;
    let mut response = client.get(download_url).send()?;
    response.copy_to(&mut file)?;
    println!("Download complete: {}", temp_file.display());

    let app_dir = PathBuf::from(env::var("LOCALAPPDATA")?).join("Catsh");
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)?;
    }

    let destination = app_dir.join("catsh.exe");
    println!("Moving binary to application directory...");
    fs::copy(&temp_file, &destination)?;

    println!("Creating desktop shortcut...");
    let desktop = PathBuf::from(env::var("USERPROFILE")?).join("Desktop");
    let shortcut = desktop.join("catsh.lnk");
    create_shortcut(&shortcut, &destination)?;

    println!("Update successfully applied.");
    Ok(())
}

fn create_shortcut(shortcut_path: &PathBuf, target_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            r#"New-Object -ComObject WScript.Shell | ForEach-Object {{ $_.CreateShortcut('{}').TargetPath = '{}' }}"#,
            shortcut_path.display(),
            target_path.display()
        ))
        .status()?;

    Ok(())
}
