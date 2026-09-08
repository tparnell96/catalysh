# Contributing to catalysh

Thank you for your interest in contributing! This document covers the project architecture, development environment, conventions, and the process for submitting changes.

---

## Project structure

```
.
├── src/
│   ├── api/        # Catalyst Center API types and HTTP operations
│   ├── app/        # Configuration, credentials, updates, and platform setup
│   ├── commands/   # Clap command definitions and routing
│   ├── handlers/   # Command orchestration and presentation logic
│   ├── helpers/    # Shared HTTP, output, and device-resolution utilities
│   ├── tui/        # Interactive terminal interfaces
│   └── main.rs     # One-shot CLI and REPL entry point
├── docs/           # Focused developer guides
└── .github/workflows/
    ├── ci.yml      # Pull-request and main-branch quality gates
    └── release.yml # Tag-gated binary and crates.io releases
```

Modules below `api`, `commands`, and `handlers` are grouped by product domain. Prefer
following an existing neighboring domain over documenting every file here; this overview
should remain stable as commands are added.

---

## Development setup

### Standard (cargo)

```bash
# Linux prerequisites
sudo apt-get install pkg-config libssl-dev

# macOS — OpenSSL via Homebrew if needed
brew install openssl

git clone https://github.com/hexabyte8/catalysh.git
cd catalysh
cargo build
```

### NixOS / nix develop

```bash
git clone https://github.com/hexabyte8/catalysh.git
cd catalysh
nix develop   # drops you into a shell with Rust, clippy, rustfmt, cargo-watch, openssl
cargo build
```

The dev shell sets `PKG_CONFIG_PATH`, `OPENSSL_DIR`, and `OPENSSL_LIB_DIR` automatically.

### Useful dev commands

```bash
cargo build                         # debug build
cargo test                          # run tests
cargo clippy --all-targets -- -D warnings   # lint
cargo fmt --all                     # format
cargo watch -x check                # watch mode (requires cargo-watch)
```

---

## Architecture overview

catalysh supports one-shot commands, a persistent REPL, and interactive TUI views. API-backed
commands generally flow through four layers:

### Layer 1 — API (`src/api/<category>/`)

Pure data: serde structs (`Deserialize + Serialize`) and `async fn` that call `crate::helpers::http::build_client()` to construct a `reqwest::Client` (respecting `verify_ssl` and loading custom CA certificates), then perform the HTTP request and deserialize the response.

```rust
// src/api/sites/getsitelist.rs
use crate::helpers::http;

pub async fn get_all_sites(config: &Config, token: &Token) -> Result<Vec<Site>> {
    let client = http::build_client(config)?;
    let url = format!("{}/dna/intent/api/v1/site", config.dnac_url);
    let resp = client.get(&url).header("X-Auth-Token", &token.value).send().await?;
    // ...
}
```

**Do not** construct a `reqwest::Client` directly; always use `http::build_client(config)` so custom certificates are picked up automatically.

### Layer 2 — Commands (`src/commands/show/<category>.rs`)

Clap `Subcommand` enums only — no logic. Each variant maps to one leaf command and declares its arguments with `#[arg(...)]` or nested `#[command(subcommand)]`.

```rust
// src/commands/show/site.rs
#[derive(Debug, Subcommand)]
pub enum SiteCommands {
    /// List all sites
    List,
    /// Show site health scores
    Health {
        #[arg(long)]
        site_type: Option<String>,
    },
}
```

### Layer 3 — Handlers (`src/handlers/show/<category>.rs`)

Bridge between the parsed command and the API. Always call `command_utils::execute_with_context` — it creates a Tokio runtime, loads config, authenticates, and hands you a `CommandContext` with `.config` and `.token`. Then dispatch to the API function and call the appropriate `utils::print_*` helper.

```rust
// src/handlers/show/site.rs
pub fn handle_site_command(subcommand: SiteCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            SiteCommands::List => {
                let sites = getsitelist::get_all_sites(&ctx.config, &ctx.token).await?;
                utils::print_sites(sites);
            }
            SiteCommands::Health { site_type } => { /* ... */ }
        }
        Ok(())
    });
}
```

### Layer 4 — Print helpers (`src/helpers/utils.rs`)

One `print_*` function per data type. Every function must check `crate::helpers::output::is_json()` first and delegate to `crate::helpers::output::print_json()` when true; otherwise render a `prettytable` table.

```rust
pub fn print_sites(sites: Vec<Site>) {
    if crate::helpers::output::is_json() {
        crate::helpers::output::print_json(&sites);
        return;
    }
    let mut table = Table::new();
    table.add_row(row!["Name", "Hierarchy"]);
    for site in sites {
        table.add_row(row![
            site.name.as_deref().unwrap_or("N/A"),
            site.site_name_hierarchy.as_deref().unwrap_or("N/A"),
        ]);
    }
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.printstd();
}
```

---

## Adding a new command

The step-by-step developer guide with full code examples is in [docs/ADDING_COMMANDS.md](docs/ADDING_COMMANDS.md).

---

## Output format support

Every `print_*` function in `src/helpers/utils.rs` **must** support both table and JSON output:

```rust
pub fn print_my_data(items: Vec<MyStruct>) {
    if crate::helpers::output::is_json() {
        crate::helpers::output::print_json(&items);  // pretty JSON to stdout
        return;
    }
    // ... prettytable rendering ...
}
```

`MyStruct` must derive `Serialize` (in addition to `Deserialize`) so `print_json` can serialise it. If the struct is defined in `src/api/`, add `#[derive(Serialize)]` there; `Deserialize` is always required for API responses.

---

## Code style

- **Formatting**: `cargo fmt --all`. PRs with formatting changes will not be merged.
- **Linting**: `cargo clippy --all-targets --all-features -- -D warnings`. All warnings are treated as errors in CI.
- **Comments**: Add comments only where the code is genuinely non-obvious. Avoid restating what the code already says.
- **`unwrap()`**: Avoid in library/handler code. Use `?` propagation or `anyhow::Context` for error context. `unwrap()` is acceptable in tests.

---

## CI/CD

### What runs in CI

| Check | Command |
|---|---|
| Formatting | `cargo fmt --all -- --check` |
| Linting | `cargo clippy --all-targets --all-features -- -D warnings` |
| Tests | `cargo test --all-features --verbose` |
| Build matrix | `cargo build` for linux-gnu, linux-musl, linux-aarch64, macos-x86_64, macos-aarch64, windows-x86_64 |

The same checks run for pull requests and pushes to `main`. Superseded runs on the same
branch are cancelled.

### Release process

1. Merge the release-ready version bump in `Cargo.toml` to `main` and wait for CI to pass.
2. Tag that commit with a SemVer tag matching the package version, then push it:
   `git tag v1.2.3 && git push origin v1.2.3`.
3. The release workflow verifies that the tag is valid SemVer, matches `Cargo.toml`, and
   points to a commit contained in `main`. It reruns formatting, linting, and tests before
   building binaries for all supported targets.
4. After all builds succeed, the workflow publishes the GitHub Release and then the crate
   to crates.io.

Tags created from feature branches or detached commits are rejected. Pre-releases use a
SemVer suffix such as `v1.2.3-rc.1` and are marked as pre-releases on GitHub.

Crate publishing uses the `CARGO_REGISTRY_TOKEN` GitHub Actions repository secret. Keep
the token scoped to publishing this crate and rotate it according to the project's
credential policy.

Published crates.io versions are immutable. If a version is published accidentally, yank
it with `cargo yank --vers <version> catalysh`; existing lockfiles can still use it, but
Cargo excludes it from new resolutions. A lower, previously unused version can then be
published, although the higher version remains visible in the registry history.

---

## Pull request process

### Branch naming

```
feat/<short-description>     # new feature or command
fix/<short-description>      # bug fix
docs/<short-description>     # documentation only
refactor/<short-description> # code change with no functional difference
```

### Commit messages

Use the imperative mood in the subject line (`Add show eox summary command`, not `Added` or `Adding`). Keep the subject under 72 characters. Reference issues with `Fixes #N` in the body when applicable.

### What reviewers check

- All CI checks pass.
- New commands follow the four-layer pattern described above.
- `print_*` functions support both `table` and `json` output.
- No direct `reqwest::Client::builder()` calls — use `http::build_client(config)`.
- `execute_with_context` is used for all API-backed commands (no manual Tokio runtime creation).
- Help strings on all `#[arg]` and `#[command]` entries are clear and complete.
- No `unwrap()` in non-test code without justification.
