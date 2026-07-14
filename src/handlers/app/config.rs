use crate::app::config::{self, CredentialMode};
use crate::commands::app::config::{AppConfigCommands, CredentialModeAction, SetVerifySslAction};
use log::error;

pub fn handle_app_config_command(subcommand: AppConfigCommands) {
    match subcommand {
        AppConfigCommands::Reset => {
            if let Err(e) = config::reset_config() {
                error!("Failed to reset configuration: {}", e);
            } else {
                println!("Configuration reset successfully.");
            }
        }
        AppConfigCommands::Show => match config::load_config() {
            Ok(cfg) => {
                println!("Current Configuration:");
                println!("---------------------");
                println!("DNA Center URL:    {}", cfg.dnac_url);
                println!("Username:          {}", cfg.username);
                println!("Verify SSL:        {}", cfg.verify_ssl);
                println!("Credential mode:   {}", cfg.credential_mode);
                match config::list_certs() {
                    Ok(certs) if !certs.is_empty() => {
                        println!("Custom CA certs:   {}", certs.join(", "))
                    }
                    _ => println!("Custom CA certs:   none"),
                }
            }
            Err(e) => {
                error!("Failed to read configuration: {}", e);
            }
        },
        AppConfigCommands::SetUrl { url } => {
            if let Err(e) = config::update_dnac_url(url) {
                error!("Failed to update DNA Center URL: {}", e);
            }
        }
        AppConfigCommands::SetVerifySsl { action } => {
            let enable = matches!(action, SetVerifySslAction::Enable);
            if let Err(e) = config::update_verify_ssl(enable) {
                error!("Failed to update SSL verification setting: {}", e);
            }
        }
        AppConfigCommands::ResetCredentials => {
            if let Err(e) = config::reset_credentials() {
                error!("Failed to reset credentials: {}", e);
            }
        }
        AppConfigCommands::SetCredentialMode { mode } => {
            let credential_mode = match mode {
                CredentialModeAction::Store => CredentialMode::StoreOnDevice,
                CredentialModeAction::Session => CredentialMode::SessionOnly,
            };
            if let Err(e) = config::update_credential_mode(credential_mode) {
                error!("Failed to update credential mode: {}", e);
            }
        }
        AppConfigCommands::InstallCert { path } => match config::install_cert(&path) {
            Ok(name) => println!(
                "Certificate '{}' installed. Re-enable SSL verification with:\n  app config set-verify-ssl enable",
                name
            ),
            Err(e) => error!("Failed to install certificate: {}", e),
        },
        AppConfigCommands::ListCerts => match config::list_certs() {
            Ok(certs) if certs.is_empty() => {
                println!("No custom CA certificates installed.");
            }
            Ok(certs) => {
                println!("Installed custom CA certificates:");
                for cert in certs {
                    println!("  {}", cert);
                }
            }
            Err(e) => error!("Failed to list certificates: {}", e),
        },
        AppConfigCommands::RemoveCert { name } => match config::remove_cert(&name) {
            Ok(()) => println!("Certificate '{}' removed.", name),
            Err(e) => error!("Failed to remove certificate: {}", e),
        },
    }
}
