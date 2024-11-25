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

    // Download the binary file
    let binary_asset = assets.iter().find(|asset| {
        let name = asset["name"].as_str().unwrap_or("");
        name.ends_with(".exe")
    }).ok_or("No compatible binary asset found")?;
    let binary_url = binary_asset["browser_download_url"]
        .as_str()
        .ok_or("Invalid binary download URL")?;
    let binary_temp_file = env::temp_dir().join(binary_asset["name"].as_str().unwrap_or("catsh_update.exe"));

    println!("Downloading binary...");
    let mut file = fs::File::create(&binary_temp_file)?;
    let mut response = client.get(binary_url).send()?;
    response.copy_to(&mut file)?;
    println!("Binary download complete: {}", binary_temp_file.display());

    // Download the icon file
    let icon_asset = assets.iter().find(|asset| {
        let name = asset["name"].as_str().unwrap_or("");
        name.ends_with(".ico")
    }).ok_or("No compatible icon asset found")?;
    let icon_url = icon_asset["browser_download_url"]
        .as_str()
        .ok_or("Invalid icon download URL")?;
    let icon_temp_file = env::temp_dir().join(icon_asset["name"].as_str().unwrap_or("catsh.ico"));

    println!("Downloading icon...");
    let mut file = fs::File::create(&icon_temp_file)?;
    let mut response = client.get(icon_url).send()?;
    response.copy_to(&mut file)?;
    println!("Icon download complete: {}", icon_temp_file.display());

    // Move the binary to the application directory
    let app_dir = PathBuf::from(env::var("LOCALAPPDATA")?).join("catsh");
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)?;
    }
    let binary_destination = app_dir.join("catsh.exe");
    println!("Moving binary to application directory...");
    fs::copy(&binary_temp_file, &binary_destination)?;

    // Move the icon to the application directory
    let icon_destination = app_dir.join("catsh.ico");
    println!("Moving icon to application directory...");
    fs::copy(&icon_temp_file, &icon_destination)?;

    // Create the desktop shortcut
    println!("Creating desktop shortcut...");
    let desktop = dirs::desktop_dir().ok_or("Failed to locate the desktop directory")?;
    let shortcut_path = desktop.join("catsh.lnk");
    create_shortcut(&shortcut_path, &binary_destination, &icon_destination)?;

    println!("Update successfully applied.");
    Ok(())
}

fn create_shortcut(shortcut_path: &PathBuf, target_path: &PathBuf, icon_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            r#"
            $WScript = New-Object -ComObject WScript.Shell;
            $Shortcut = $WScript.CreateShortcut('{}');
            $Shortcut.TargetPath = '{}';
            $Shortcut.IconLocation = '{}';
            $Shortcut.Save();
            "#,
            shortcut_path.display(),
            target_path.display(),
            icon_path.display()
        ))
        .output()?; // Capture output

    if !output.status.success() {
        eprintln!(
            "PowerShell failed with status: {}\nError: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
        return Err("Failed to create shortcut with icon".into());
    }

    println!(
        "PowerShell executed successfully:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );

    Ok(())
}
