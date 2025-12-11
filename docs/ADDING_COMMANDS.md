# Adding New Commands to Catalysh

This guide provides step-by-step instructions for adding new commands to catalysh. The refactored command system makes it much easier to add new functionality.

## Overview

Commands in catalysh follow a structured pattern with three main components:
1. **Command Definition** - Define the command structure in `src/commands/`
2. **Handler Implementation** - Implement command logic in `src/handlers/`
3. **API Integration** (optional) - Add API calls if needed in `src/api/`

## Quick Start: Adding a Simple Command

Let's add a simple command that doesn't require API access (like `app version`):

### Step 1: Define the Command

Edit the appropriate command enum in `src/commands/`:

```rust
// src/commands/app/mod.rs
#[derive(Debug, Subcommand)]
pub enum AppCommands {
    // ... existing commands ...
    
    /// Show application version
    Version,
}
```

### Step 2: Add the Handler

Edit the handler module in `src/handlers/`:

```rust
// src/handlers/app/mod.rs
pub fn handle_app_command(subcommand: AppCommands) {
    match subcommand {
        // ... existing handlers ...
        
        AppCommands::Version => handle_version_command(),
    }
}

fn handle_version_command() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const NAME: &str = env!("CARGO_PKG_NAME");
    
    println!("{} version {}", NAME, VERSION);
    println!("A command line utility for interacting with Cisco Catalyst Center");
}
```

That's it! Your command is ready to use.

## Adding a Command with API Integration

For commands that need to fetch data from Catalyst Center, follow these steps:

### Example: Adding `show site list` command

#### Step 1: Create the API Module

Create a new API module to handle the Catalyst Center API calls:

```rust
// src/api/sites/mod.rs
pub mod getsitelist;
```

```rust
// src/api/sites/getsitelist.rs
use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    pub id: Option<String>,
    pub name: Option<String>,
    pub site_name_hierarchy: Option<String>,
    // Add other fields as needed
}

#[derive(Debug, Deserialize)]
struct SitesResponse {
    response: Vec<Site>,
}

pub async fn get_all_sites(config: &Config, token: &Token) -> Result<Vec<Site>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(!config.verify_ssl)
        .build()?;

    let sites_url = format!("{}/dna/intent/api/v1/site", config.dnac_url);

    let resp = client
        .get(&sites_url)
        .header("X-Auth-Token", &token.value)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!("Failed to fetch sites: {}", resp.status()));
    }

    let sites_response: SitesResponse = resp.json().await?;
    Ok(sites_response.response)
}
```

Don't forget to add the module to `src/api/mod.rs`:

```rust
// src/api/mod.rs
pub mod sites;
```

#### Step 2: Define the Command

```rust
// src/commands/show/site.rs
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum SiteCommands {
    /// List all sites
    List,
}
```

Add to `src/commands/show/mod.rs`:

```rust
pub mod site;

#[derive(Debug, Subcommand)]
pub enum ShowCommands {
    // ... existing commands ...
    
    /// Show site information
    Site {
        #[command(subcommand)]
        subcommand: site::SiteCommands,
    },
}
```

#### Step 3: Implement the Handler

```rust
// src/handlers/show/site.rs
use crate::commands::show::site::SiteCommands;
use crate::api::sites::getsitelist;
use crate::helpers::command_utils;
use log::error;
use prettytable::{table, row};

pub fn handle_site_command(subcommand: SiteCommands) {
    // Use command_utils for automatic auth and config loading!
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            SiteCommands::List => {
                match getsitelist::get_all_sites(&ctx.config, &ctx.token).await {
                    Ok(sites) => {
                        println!("\nSites:");
                        let mut table = table!([FbFy => "Site Name", "Hierarchy"]);

                        for site in &sites {
                            table.add_row(row![
                                site.name.as_deref().unwrap_or("N/A"),
                                site.site_name_hierarchy.as_deref().unwrap_or("N/A")
                            ]);
                        }
                        table.printstd();
                    }
                    Err(e) => error!("Failed to retrieve sites: {}", e),
                }
            }
        }
        Ok(())
    });
}
```

Add to `src/handlers/show/mod.rs`:

```rust
pub mod site;

pub fn handle_show_command(subcommand: ShowCommands) {
    match subcommand {
        // ... existing handlers ...
        
        ShowCommands::Site { subcommand } => site::handle_site_command(subcommand),
    }
}
```

## Key Benefits of the Refactored System

### 1. No Boilerplate for Auth and Config

Before refactoring, every handler needed 30+ lines of boilerplate:

```rust
// OLD WAY - Don't do this anymore!
pub fn handle_device_command(subcommand: DeviceCommands) {
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    runtime.block_on(async {
        let config = match config::load_config() {
            Ok(cfg) => cfg,
            Err(e) => {
                error!("Failed to load configuration: {}", e);
                return;
            }
        };

        let token = match auth::authenticate(&config).await {
            Ok(t) => t,
            Err(e) => {
                error!("Authentication failed: {}", e);
                return;
            }
        };
        
        // Your actual logic here...
    });
}
```

Now it's just one line:

```rust
// NEW WAY - Use this!
pub fn handle_site_command(subcommand: SiteCommands) {
    command_utils::execute_with_context(|ctx| async move {
        // Your actual logic here with ctx.config and ctx.token already available!
        Ok(())
    });
}
```

### 2. Consistent Error Handling

The `command_utils` module provides consistent error handling across all commands.

### 3. Easy to Test

The modular structure makes it easy to test individual components.

## Command Structure Examples

### Command with Arguments

```rust
#[derive(Debug, Subcommand)]
pub enum DeviceCommands {
    /// Show device details
    Detail {
        #[command(subcommand)]
        filter: DeviceDetailFilter,
    },
}

#[derive(Debug, Subcommand)]
pub enum DeviceDetailFilter {
    /// Show device detail by hostname
    Hostname {
        /// The hostname of the device
        hostname: String,
    },
    /// Show device detail by IP address
    Ip {
        /// The IP address of the device
        ip_address: String,
    },
}
```

### Command with Optional Arguments

```rust
#[derive(Debug, Subcommand)]
pub enum DeviceCommands {
    /// List devices
    List {
        #[command(subcommand)]
        filter: DeviceListFilter,
    },
}

#[derive(Debug, Subcommand)]
pub enum DeviceListFilter {
    /// List all devices
    All,
    /// List devices filtered by hostname
    Hostname {
        /// Optional partial hostname to filter by
        partial_hostname: Option<String>,
    },
}
```

## Testing Your New Command

1. Build the project:
```bash
cargo build
```

2. Run the application:
```bash
cargo run
```

3. Test your command:
```bash
catalysh> app version
catalysh> show site list
```

## Best Practices

1. **Use `command_utils::execute_with_context`** for any command that needs API access
2. **Keep command definitions simple** - complex logic belongs in the handler
3. **Use descriptive help text** - it appears in the CLI help
4. **Handle errors gracefully** - use `log::error!` for error messages
5. **Follow existing patterns** - consistency makes the codebase easier to maintain
6. **Add comprehensive documentation** for complex commands

## Summary

Adding a new command now requires:
- ✅ 1 command definition file (or add to existing)
- ✅ 1 handler file (or add to existing)
- ✅ 1 API file (only if fetching data)
- ✅ 2-3 lines to wire it up in mod.rs files

That's it! No more boilerplate, consistent error handling, and easy to test.
