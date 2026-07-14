# Adding New Commands to catalysh

This is the authoritative developer reference for adding a new API-backed `show` command. Follow every step; each layer is required and each has a specific responsibility.

The example below adds a hypothetical `show vlan list` command backed by a `/dna/intent/api/v1/vlan` endpoint.

---

## Overview: the four-layer pattern

```
src/api/vlans/mod.rs             Layer 1 — serde structs + async HTTP fetch
src/commands/show/vlan.rs        Layer 2 — clap Subcommand definitions
src/handlers/show/vlan.rs        Layer 3 — execute_with_context dispatch
src/helpers/utils.rs             Layer 4 — print_vlans() table/JSON helper
```

Plus three wiring lines:
- `pub mod vlans;` in `src/api/mod.rs`
- variant in `ShowCommands` in `src/commands/show/mod.rs`
- dispatch arm in `src/handlers/show/mod.rs`

---

## Step 1 — Create the API module

Create `src/api/vlans/mod.rs`. All response types must derive both `Deserialize` and `Serialize` so they can be passed to `print_json`.

```rust
// src/api/vlans/mod.rs

use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vlan {
    pub vlan_number: Option<u32>,
    pub vlan_type: Option<String>,
    pub ip_address: Option<String>,
    pub prefix: Option<String>,
    pub network_address: Option<String>,
}

#[derive(Debug, Deserialize)]
struct VlanResponse {
    response: Vec<Vlan>,
}

pub async fn get_vlans(config: &Config, token: &Token) -> Result<Vec<Vlan>> {
    // Always use http::build_client — it loads custom CA certs and respects verify_ssl.
    let client = http::build_client(config)?;

    let url = format!("{}/dna/intent/api/v1/vlan", config.dnac_url);

    let resp = client
        .get(&url)
        .header("X-Auth-Token", &token.value)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!("Failed to fetch VLANs: HTTP {}", resp.status()));
    }

    let body: VlanResponse = resp.json().await?;
    Ok(body.response)
}
```

**Key rules:**
- Always call `http::build_client(config)` — never `reqwest::Client::builder()` directly.
- Return `anyhow::Result<T>` and propagate errors with `?` or `anyhow!`.
- Inner response wrappers (`VlanResponse`) do not need to be `pub` or `Serialize`.
- Use `#[serde(rename_all = "camelCase")]` to match the Catalyst Center JSON field names.

---

## Step 2 — Register the module in `src/api/mod.rs`

```rust
// src/api/mod.rs  (add one line)
pub mod vlans;
```

---

## Step 3 — Define the clap subcommand

Create `src/commands/show/vlan.rs`. This file contains **only** struct/enum definitions — no logic.

```rust
// src/commands/show/vlan.rs

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum VlanCommands {
    /// List all VLANs
    List {
        /// Filter by VLAN type (e.g. "Access", "Trunk")
        #[arg(long)]
        vlan_type: Option<String>,
    },
}
```

Add arguments with `#[arg(...)]`. Common patterns:

```rust
// Required positional argument
device_id: String,

// Optional positional argument
hostname: Option<String>,

// Named flag (--limit 50)
#[arg(long, default_value_t = 100)]
limit: u32,

// Enum with allowed values
#[arg(long, value_parser = ["access", "trunk"])]
vlan_type: Option<String>,

// Nested sub-subcommand
#[command(subcommand)]
filter: VlanFilter,
```

---

## Step 4 — Add the variant to `ShowCommands`

In `src/commands/show/mod.rs`, add:

1. `pub mod vlan;` at the top with the other `pub mod` lines.
2. A new variant in the `ShowCommands` enum.

```rust
// src/commands/show/mod.rs

pub mod vlan;  // ← add this

// In the ShowCommands enum:
/// Show VLAN information
Vlan {
    #[command(subcommand)]
    subcommand: vlan::VlanCommands,
},
```

---

## Step 5 — Create the handler

Create `src/handlers/show/vlan.rs`. Always use `command_utils::execute_with_context` — it handles Tokio runtime creation, config loading, and authentication. The closure receives a `CommandContext` with `.config` and `.token`.

```rust
// src/handlers/show/vlan.rs

use crate::api::vlans;
use crate::commands::show::vlan::VlanCommands;
use crate::helpers::{command_utils, utils};
use log::error;

pub fn handle_vlan_command(subcommand: VlanCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            VlanCommands::List { vlan_type } => {
                match vlans::get_vlans(&ctx.config, &ctx.token).await {
                    Ok(mut vlans) => {
                        if let Some(ref filter) = vlan_type {
                            vlans.retain(|v| {
                                v.vlan_type.as_deref().unwrap_or("") == filter.as_str()
                            });
                        }
                        utils::print_vlans(vlans);
                    }
                    Err(e) => error!("Failed to retrieve VLANs: {}", e),
                }
            }
        }
        Ok(())
    });
}
```

**Key rules:**
- `execute_with_context` takes a closure `|ctx| async move { ... }` that returns `Result<()>`.
- Use `log::error!` for recoverable errors shown to the user.
- Use `?` for errors that should abort the closure (they will be caught and logged by `execute_with_context`).
- Call a `utils::print_*` function for output — never write `println!` table rendering in a handler.

---

## Step 6 — Wire the handler in `src/handlers/show/mod.rs`

```rust
// src/handlers/show/mod.rs

pub mod vlan;  // ← add this

// In handle_show_command():
ShowCommands::Vlan { subcommand } => vlan::handle_vlan_command(subcommand),
```

---

## Step 7 — Add the print helper to `src/helpers/utils.rs`

Every `print_*` function must support both output modes. Check `is_json()` first; return early with `print_json()` if true. Otherwise render a `prettytable` table.

```rust
// src/helpers/utils.rs  (append)

use crate::api::vlans::Vlan;  // add to imports at top of file

pub fn print_vlans(vlans: Vec<Vlan>) {
    if crate::helpers::output::is_json() {
        crate::helpers::output::print_json(&vlans);
        return;
    }

    let mut table = Table::new();
    table.add_row(row!["VLAN", "Type", "IP Address", "Network"]);

    for vlan in vlans {
        table.add_row(row![
            vlan.vlan_number.map(|n| n.to_string()).unwrap_or_else(|| "N/A".to_string()),
            vlan.vlan_type.as_deref().unwrap_or("N/A"),
            vlan.ip_address.as_deref().unwrap_or("N/A"),
            vlan.network_address.as_deref().unwrap_or("N/A"),
        ]);
    }

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.printstd();
}
```

**Key rules:**
- The type passed to `print_json` must implement `serde::Serialize`.
- Use `format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR` for consistent table formatting.
- Every optional field should fall back to `"N/A"` for readability.

---

## Checklist

Before opening a PR, verify:

- [ ] `src/api/vlans/mod.rs` — structs derive `Serialize + Deserialize`; fetch function uses `http::build_client`
- [ ] `src/api/mod.rs` — `pub mod vlans;` added
- [ ] `src/commands/show/vlan.rs` — clap enum with doc comments on every variant and arg
- [ ] `src/commands/show/mod.rs` — `pub mod vlan;` and variant in `ShowCommands`
- [ ] `src/handlers/show/vlan.rs` — uses `execute_with_context`; delegates output to `utils::print_*`
- [ ] `src/handlers/show/mod.rs` — `pub mod vlan;` and dispatch arm added
- [ ] `src/helpers/utils.rs` — `print_vlans` checks `is_json()` before rendering table
- [ ] `cargo build` succeeds
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo fmt --all` applied

---

## Common mistakes

| Mistake | Correct approach |
|---|---|
| `reqwest::Client::builder().build()` | `http::build_client(config)?` |
| `tokio::runtime::Runtime::new()…block_on(…)` | `command_utils::execute_with_context(\|ctx\| async move { … })` |
| `println!("{:?}", data)` in a handler | `utils::print_*(data)` |
| Struct derives only `Deserialize` | Add `Serialize` so `print_json` works |
| Missing `pub mod` in `mod.rs` | Add to both `src/api/mod.rs` and `src/commands/show/mod.rs` and `src/handlers/show/mod.rs` |
| Omitting doc comment on clap variant | Clap uses `///` comments as help text — always add them |

---

## Reference: existing API modules

Browse the following as concrete examples before writing new code:

| Module | Endpoint pattern | Notes |
|---|---|---|
| `src/api/sites/getsitelist.rs` | `GET /dna/intent/api/v1/site` | Simplest pattern; single response wrapper |
| `src/api/devices/getdevicelist.rs` | `GET /dna/intent/api/v1/network-device` | Paginated list with multiple filters |
| `src/api/clients/getclientdetail.rs` | `GET /dna/intent/api/v1/client-detail` | Query params passed via `.query(&[…])` |
| `src/api/pathanalysis/mod.rs` | `POST` + `GET` | Two-step async: submit then poll |
| `src/api/commandrunner/mod.rs` | `POST` returning a task ID | Async task pattern |
