#[cfg(windows)]
use {
    futures_util::StreamExt,
    indicatif::{ProgressBar, ProgressStyle},
    reqwest::Client,
    serde_json::Value,
    std::{
        env,
        path::PathBuf,
        process::Command,
    },
    thiserror::Error,
    tokio::{
        fs,
        io::AsyncWriteExt,
        time::{sleep, Duration},
    },
};

const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 1000;
const TIMEOUT_SECS: u64 = 30;

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("Failed to create directory: {0}")]
    DirectoryCreation(#[from] std::io::Error),
    
    #[error("Failed to download release: {0}")]
    ReleaseDownload(String),
    
    #[error("Failed to parse release info: {0}")]
    ReleaseInfoParse(String),
    
    #[error("Failed to download file: {0}")]
    FileDownload(String),
    
    #[error("Failed to write file: {0}")]
    FileWrite(#[from] tokio::io::Error),
    
    #[error("Environment variable not found: {0}")]
    EnvVar(#[from] std::env::VarError),
    
    #[error("Failed to create shortcut: {0}")]
    ShortcutCreation(String),
}

pub type Result<T> = std::result::Result<T, SetupError>;
#[cfg(windows)]
pub async fn setup() -> Result<()> {
    let local_app_data = env::var("LOCALAPPDATA")?;
    let app_dir = PathBuf::from(local_app_data).join("catalysh");

    let progress = ProgressBar::new_spinner();
    progress.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    progress.set_message(format!("Setting up Catalysh in {}", app_dir.display()));

    // Create application directory if it doesn't exist
    fs::create_dir_all(&app_dir).await.map_err(SetupError::DirectoryCreation)?;
    
    // Download latest release and icon
    download_latest_release(&app_dir, &progress).await?;
    download_app_icon(&app_dir, &progress).await?;
    
    // Create desktop shortcut
    create_desktop_shortcut(&app_dir, &progress).await?;
    
    progress.finish_with_message("Setup completed successfully!");
    Ok(())
}

#[cfg(not(windows))]
pub async fn setup() -> Result<()> {
    Ok(()) // Do nothing on non-Windows platforms
}

#[cfg(windows)]
async fn download_latest_release(app_dir: &PathBuf, progress: &ProgressBar) -> Result<()> {
    progress.set_message("Fetching latest release information...");
    
    let client = Client::builder()
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .build()
        .map_err(|e| SetupError::ReleaseDownload(e.to_string()))?;
    
    let mut retries = 0;
    let release_info: Value = loop {
        match client
            .get("https://api.github.com/repos/tparnell96/catalysh/releases/latest")
            .header("User-Agent", "catalysh-installer")
            .send()
            .await
        {
            Ok(response) => {
                match response.json().await {
                    Ok(json) => break json,
                    Err(e) => {
                        if retries >= MAX_RETRIES {
                            return Err(SetupError::ReleaseInfoParse(e.to_string()));
                        }
                    }
                }
            }
            Err(e) => {
                if retries >= MAX_RETRIES {
                    return Err(SetupError::ReleaseDownload(e.to_string()));
                }
            }
        }
        retries += 1;
        progress.set_message(format!("Retrying release fetch ({}/{})", retries, MAX_RETRIES));
        sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
    };
    
    let assets = release_info["assets"].as_array()
        .ok_or_else(|| SetupError::ReleaseInfoParse("No assets found".into()))?;
    
    let windows_asset = assets.iter()
        .find(|asset| asset["name"].as_str()
            .map(|name| name.contains("windows"))
            .unwrap_or(false))
        .ok_or_else(|| SetupError::ReleaseInfoParse("Windows asset not found".into()))?;
    
    let download_url = windows_asset["browser_download_url"]
        .as_str()
        .ok_or_else(|| SetupError::ReleaseInfoParse("Download URL not found".into()))?;
    
    progress.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));
    
    let mut retries = 0;
    let response = loop {
        match client.get(download_url)
            .send()
            .await
        {
            Ok(response) => {
                let total_size = response.content_length().unwrap_or(0);
                progress.set_length(total_size);
                break response;
            }
            Err(e) => {
                if retries >= MAX_RETRIES {
                    return Err(SetupError::FileDownload(e.to_string()));
                }
                retries += 1;
                progress.set_message(format!("Retrying download ({}/{})", retries, MAX_RETRIES));
                sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
            }
        }
    };

    let binary_path = app_dir.join("catalysh.exe");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&binary_path)
        .await
        .map_err(SetupError::FileWrite)?;
        
    let mut stream = response.bytes_stream();
    let mut downloaded: u64 = 0;
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| SetupError::FileDownload(e.to_string()))?;
        file.write_all(&chunk).await.map_err(SetupError::FileWrite)?;
        downloaded += chunk.len() as u64;
        progress.set_position(downloaded);
    }
    
    progress.finish_with_message("Binary download complete");
    Ok(())
}

#[cfg(windows)]
async fn download_app_icon(app_dir: &PathBuf, progress: &ProgressBar) -> Result<()> {
    progress.set_message("Downloading application icon...");
    progress.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));
    
    let client = Client::builder()
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .build()
        .map_err(|e| SetupError::FileDownload(e.to_string()))?;
        
    let icon_url = "https://raw.githubusercontent.com/tparnell96/catalysh/main/assets/icon.ico";
    
    let mut retries = 0;
    let response = loop {
        match client.get(icon_url)
            .send()
            .await
        {
            Ok(response) => {
                let total_size = response.content_length().unwrap_or(0);
                progress.set_length(total_size);
                break response;
            }
            Err(e) => {
                if retries >= MAX_RETRIES {
                    return Err(SetupError::FileDownload(e.to_string()));
                }
                retries += 1;
                progress.set_message(format!("Retrying icon download ({}/{})", retries, MAX_RETRIES));
                sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
            }
        }
    };

    let icon_path = app_dir.join("icon.ico");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&icon_path)
        .await
        .map_err(SetupError::FileWrite)?;
        
    let mut stream = response.bytes_stream();
    let mut downloaded: u64 = 0;
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| SetupError::FileDownload(e.to_string()))?;
        file.write_all(&chunk).await.map_err(SetupError::FileWrite)?;
        downloaded += chunk.len() as u64;
        progress.set_position(downloaded);
    }
    
    progress.finish_with_message("Icon download complete");
    Ok(())

#[cfg(windows)]
async fn create_desktop_shortcut(app_dir: &PathBuf, progress: &ProgressBar) -> Result<()> {
    progress.set_message("Creating desktop shortcut...");
    
    let desktop = if let Ok(home) = env::var("USERPROFILE") {
        PathBuf::from(home).join("Desktop")
    } else {
        return Err(SetupError::EnvVar(std::env::VarError::NotPresent));
    };
    
    let shortcut_path = desktop.join("Catalysh.lnk");
    let target_path = app_dir.join("catalysh.exe");
    let icon_path = app_dir.join("icon.ico");
    
    if !target_path.exists() {
        return Err(SetupError::FileDownload("Target executable not found".to_string()));
    }
    
    progress.set_message("Creating desktop shortcut...");
    let ps_script = format!(
        r#"
        $WshShell = New-Object -comObject WScript.Shell
        $Shortcut = $WshShell.CreateShortcut("{}")
        $Shortcut.TargetPath = "{}"
        $Shortcut.IconLocation = "{}"
        $Shortcut.Save()
        "#,
        shortcut_path.display(),
        target_path.display(),
        icon_path.display()
    );
    
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(&ps_script)
        .output()?;
        
    if !output.status.success() {
        return Err(SetupError::ShortcutCreation(
            String::from_utf8_lossy(&output.stderr).to_string()
        ));
    }
    
    progress.finish_with_message("Desktop shortcut created successfully");
    Ok(())
}
