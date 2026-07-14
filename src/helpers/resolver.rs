// src/helpers/resolver.rs
// Device resolver: look up a device by hostname, IP, or MAC address.

use crate::api::authentication::auth::Token;
use crate::api::devices::getdevicelist::{self, AllDevices};
use crate::app::config::Config;
use anyhow::{anyhow, Result};

/// Normalise a MAC address string by removing all separators (`:`, `-`, `.`) and
/// converting to lowercase so different formats compare equal.
fn norm_mac(mac: &str) -> String {
    mac.to_lowercase().replace([':', '-', '.'], "")
}

/// Look up a device by any of:
/// - Exact management IP address (e.g. `192.168.1.1`)
/// - Exact or partial hostname (case-insensitive)
/// - MAC address in any common format — colons, dashes, dots, or bare hex
///   (matched against the device's management MAC **and** AP ethernet MAC)
///
/// Returns the first matching device from the Catalyst Center inventory.
pub async fn resolve_device(config: &Config, token: &Token, selector: &str) -> Result<AllDevices> {
    let devices = getdevicelist::get_all_devices(config, token).await?;

    let sel_lower = selector.to_lowercase();
    let sel_mac = norm_mac(selector);

    let matched = devices.into_iter().find(|d| {
        // 1. Exact management IP match
        if let Some(ref ip) = d.management_ip_address {
            if ip == selector {
                return true;
            }
        }

        // 2. Case-insensitive hostname match (partial allowed)
        if let Some(ref h) = d.hostname {
            if h.to_lowercase().contains(&sel_lower) {
                return true;
            }
        }

        // 3. Device management MAC match (normalised)
        if let Some(ref mac) = d.mac_address {
            if norm_mac(mac) == sel_mac {
                return true;
            }
        }

        // 4. AP ethernet MAC match (normalised)
        if let Some(ref mac) = d.ap_ethernet_mac_address {
            if norm_mac(mac) == sel_mac {
                return true;
            }
        }

        // 5. Serial number match (case-insensitive)
        if let Some(ref serial) = d.serial_number {
            if serial.to_lowercase() == sel_lower {
                return true;
            }
        }

        false
    });

    matched.ok_or_else(|| anyhow!("No device found matching '{}'", selector))
}

/// Resolve a device and return its UUID (`id` field).
/// Useful for commands that need a device UUID (e.g. command runner).
pub async fn resolve_device_id(config: &Config, token: &Token, selector: &str) -> Result<String> {
    let device = resolve_device(config, token, selector).await?;
    device
        .id
        .ok_or_else(|| anyhow!("Device '{}' has no UUID in inventory", selector))
}

/// Resolve a device and return its AP ethernet MAC address.
/// Used by `show ap config` so the caller can pass any identifier (hostname,
/// management IP, radio MAC, ethernet MAC) and always land on the right value.
pub async fn resolve_ap_ethernet_mac(
    config: &Config,
    token: &Token,
    selector: &str,
) -> Result<String> {
    let device = resolve_device(config, token, selector).await?;
    device.ap_ethernet_mac_address.ok_or_else(|| {
        anyhow!(
            "Device '{}' has no AP ethernet MAC — it may not be a wireless access point",
            selector
        )
    })
}
