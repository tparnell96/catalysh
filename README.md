# catalysh

**A CLI for Cisco Catalyst Center**

![CI](https://github.com/hexabyte8/catalysh/actions/workflows/ci.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/catalysh.svg)](https://crates.io/crates/catalysh)

---

## What is catalysh?

catalysh is a command-line interface for [Cisco Catalyst Center](https://www.cisco.com/site/us/en/products/networking/catalyst-center/index.html) (formerly DNA Center) that wraps the Catalyst Center REST API in a familiar, shell-like experience. It runs in either a persistent interactive REPL or as a one-liner command — the same binary, the same subcommands, whichever fits your workflow. All commands support a `-o json` flag so output can be piped directly to `jq`, shell scripts, or monitoring pipelines.

---

## Installation

### From crates.io

```bash
cargo install catalysh
```

### Build from source

```bash
git clone https://github.com/hexabyte8/catalysh.git
cd catalysh
cargo install --path .
```

Requires Rust stable. On Linux you also need `pkg-config` and `libssl-dev` (or `openssl-devel`).

### Pre-built binaries

Binaries for Linux (glibc and musl), macOS (Intel and Apple Silicon), and Windows are published to the [GitHub Releases](https://github.com/hexabyte8/catalysh/releases) page with SHA-256 checksums. Download the archive for your platform, extract the binary, and place it somewhere on your `PATH`.

### NixOS / Nix

```bash
# Run without installing
nix run github:hexabyte8/catalysh

# Install into a profile
nix profile install github:hexabyte8/catalysh

# Build a local checkout
nix build
```

See [NixOS](#nixos) for the dev-shell.

---

## First-time setup

The first time any API-backed command runs, catalysh detects the missing configuration and launches an interactive setup wizard:

```
Enter Cisco DNAC URL without a / at the end (e.g., https://dnac.example.com): https://dnac.corp.example.com
Enter your username: admin
Verify SSL certificates? (y/n): y
Store credentials on this device? (y = store encrypted on-device, n = prompt each session) [y/n]: y
Enter your password:
Credentials stored securely.
Configuration complete.
```

Settings are saved to a YAML file (see [Data storage paths](#data-storage-paths)). You can change any individual setting later with `app config` subcommands without re-running the full wizard.

---

## Usage modes

### One-liner

Pass arguments directly on the command line. catalysh runs the command and exits — ideal for scripts and automation:

```bash
catalysh show device list all
catalysh show device list hostname spine
catalysh -o json show site list | jq '.[].name'
catalysh show health network
```

### REPL

Run `catalysh` with no arguments to enter the interactive prompt. Commands have full history (up/down arrows), and the prompt shows your current context:

```
$ catalysh
catalysh> show device count
catalysh> show client list --client-type wireless --limit 50
catalysh> app config show
catalysh> exit
```

History is persisted at `~/.catalysh/history` across sessions.

---

## Output formats

Every command accepts `-o <format>` (short: `-o`), which is a global flag and must come **before** the subcommand:

| Flag | Description |
|------|-------------|
| `-o table` | Human-readable table (default) |
| `-o json` | Pretty-printed JSON, suitable for `jq` |

Examples:

```bash
# Default table output
catalysh show device list all

# JSON output — pipe to jq
catalysh -o json show device list all | jq '.[] | {hostname, managementIpAddress}'

# JSON output in the REPL
catalysh> -o json show site list
```

---

## Full command reference

### show device

| Subcommand | Description |
|---|---|
| `show device list all` | List all devices |
| `show device list hostname [<partial>]` | Filter devices by hostname substring |
| `show device list ip [<partial>]` | Filter devices by management IP |
| `show device list wlc [<partial>]` | Filter devices by associated WLC IP |
| `show device detail hostname <hostname>` | Full device detail by hostname |
| `show device detail ip <ip>` | Full device detail by IP |
| `show device detail mac <mac>` | Full device detail by MAC address |
| `show device enrichment mac <mac>` | Device enrichment by MAC address |
| `show device enrichment ip <ip>` | Device enrichment by IP address |
| `show device count` | Total device count |
| `show device health [--device-role <role>]` | Device health scores; optional role filter (`ACCESS`, `CORE`, `DISTRIBUTION`, `BORDER ROUTER`) |
| `show device compliance [--compliance-type <type>]` | Device compliance status |

### show client

| Subcommand | Description |
|---|---|
| `show client list` | List clients (default limit: 100) |
| `show client list --mac <mac>` | Filter by MAC address |
| `show client list --ipv4 <ip>` | Filter by IPv4 address |
| `show client list --ssid <ssid>` | Filter by SSID |
| `show client list --client-type wired\|wireless` | Filter by connection type |
| `show client list --site-id <id>` | Filter by site UUID |
| `show client list --limit <n>` | Override result limit |
| `show client detail <mac>` | Full client detail by MAC address |
| `show client enrichment network_user_id <user>` | Enrichment by network user ID |
| `show client enrichment mac_address <mac>` | Enrichment by MAC address |
| `show client proximity <username> [--days <n>]` | APs/locations a user has connected from (default: 14 days) |

### show site

| Subcommand | Description |
|---|---|
| `show site list` | List all sites |
| `show site health [--site-type area\|building\|floor]` | Site health scores |

### show issue

| Subcommand | Description |
|---|---|
| `show issue list` | List all issues |
| `show issue list <search-option> <value>` | Filter issues; options: `start-time`, `end-time`, `site-id`, `device-id`, `mac-address`, `priority`, `ai-driven`, `issue-status` |

### show topology

| Subcommand | Description |
|---|---|
| `show topology physical` | Physical topology |
| `show topology sites` | Site topology |
| `show topology l3 <type>` | L3 topology by routing protocol (e.g., `OSPF`, `IS-IS`) |
| `show topology vlans` | VLAN names |

### show health

| Subcommand | Description |
|---|---|
| `show health network [--site-id <id>]` | Overall network health |
| `show health client [--site-id <id>]` | Client health summary |

### show wireless

| Subcommand | Description |
|---|---|
| `show wireless ssid --site-id <id>` | SSIDs configured for a site |

### show ap

| Subcommand | Description |
|---|---|
| `show ap config <selector>` | AP configuration by hostname, management IP, or MAC address |
| `show ap neighbors <selector>` | AP uplink switch, connected interface, status, VLAN, speed, and description |
| `show ap rf-profile` | All RF profiles |

### show tag

| Subcommand | Description |
|---|---|
| `show tag list [--name <name>]` | List tags, optionally filtered by name |
| `show tag members <tag-id>` | Members assigned to a tag |

### show path

| Subcommand | Description |
|---|---|
| `show path list` | List existing path trace flows |
| `show path trace --source-ip <ip> --dest-ip <ip>` | Start a new path trace; optional: `--source-port`, `--dest-port`, `--protocol` |
| `show path get <flow-id>` | Get path trace detail by flow analysis ID |

### show task

| Subcommand | Description |
|---|---|
| `show task list [--offset <n>] [--limit <n>]` | List tasks (default offset: 1, limit: 10) |
| `show task get <task-id>` | Task detail and status by ID |

### show eox

| Subcommand | Description |
|---|---|
| `show eox summary` | End-of-Life summary counts |
| `show eox devices [--limit <n>]` | Devices with active EoX alerts |
| `show eox device <device-id>` | EoX detail for a specific device UUID |

### show advisory

| Subcommand | Description |
|---|---|
| `show advisory list` | All security advisories |
| `show advisory device <device-id>` | Advisories affecting a specific device |
| `show advisory aggregate` | Advisory counts by severity |

### show discovery

| Subcommand | Description |
|---|---|
| `show discovery list [--limit <n>]` | List discovery jobs |
| `show discovery get <id>` | Discovery job detail by ID |

### show platform

| Subcommand | Description |
|---|---|
| `show platform release` | Catalyst Center platform release information |
| `show platform packages` | Installed platform packages |

### show network-settings

| Subcommand | Description |
|---|---|
| `show network-settings global` | Global network settings (DNS, NTP, domain) |

### run command

Run read-only CLI commands on managed devices via the Catalyst Center command runner.

| Subcommand | Description |
|---|---|
| `run command legit-reads` | List the CLI commands permitted by the command runner |
| `run command exec --devices <uuid1,uuid2,...> --commands <cmd> [<cmd> ...]` | Execute commands on devices; returns a task ID |

The `exec` subcommand is asynchronous. Use `show task get <task-id>` to poll for output.

### config

```bash
catalysh> config
```

Enters a sub-REPL in configuration mode for making configuration changes interactively.

### app config

| Subcommand | Description |
|---|---|
| `app config show` | Display current configuration |
| `app config reset` | Reset configuration and credentials (runs setup wizard on next API call) |
| `app config set-url <url>` | Update the Catalyst Center URL |
| `app config set-verify-ssl enable` | Enable SSL certificate verification |
| `app config set-verify-ssl disable` | Disable SSL certificate verification |
| `app config reset-credentials` | Wipe stored credentials and optionally re-enter them |
| `app config set-credential-mode store` | Store credentials encrypted on-device (default) |
| `app config set-credential-mode session` | Never persist credentials; prompt on each session |
| `app config install-cert <path>` | Install a custom CA certificate (PEM or DER) |
| `app config list-certs` | List installed custom CA certificates |
| `app config remove-cert <filename>` | Remove an installed certificate by filename |

### app update / app version

```bash
app update    # Download and replace the binary with the latest GitHub Release
app version   # Print the currently running version
```

### clear / exit

```bash
clear   # Clear the terminal screen
exit    # Exit the REPL (Ctrl-D also works)
```

---

## SSL certificates

### Why you might need a custom certificate

Catalyst Center deployments are commonly accessed with a certificate signed by a private corporate CA that is not in the system trust store. If `verify-ssl` is enabled and you see TLS errors, you have two options:

1. **Disable verification** — quick but not recommended for production:
   ```bash
   app config set-verify-ssl disable
   ```

2. **Install the CA certificate** — the correct approach:
   ```bash
   app config install-cert /path/to/your-corporate-ca.pem
   ```

catalysh stores the certificate in its own certs directory and loads it automatically on every request when `verify-ssl` is enabled. The system trust store is not modified.

### Obtaining the certificate

**From a browser (Chrome/Firefox):**
1. Navigate to the Catalyst Center URL.
2. Click the padlock → Certificate → download the root CA in PEM format.

**Using openssl:**
```bash
openssl s_client -connect dnac.corp.example.com:443 -showcerts </dev/null 2>/dev/null \
  | openssl x509 -outform PEM > dnac-ca.pem
```

### Managing certificates

```bash
# Install
app config install-cert ~/downloads/dnac-ca.pem

# Verify it was stored
app config list-certs

# Remove if no longer needed
app config remove-cert dnac-ca.pem
```

### Certificate + verify-ssl relationship

Custom certificates are only loaded when `verify-ssl` is **enabled**. If verification is disabled, certificates in the certs directory are ignored entirely.

---

## Credential storage

catalysh supports two credential modes, configured at setup or changed at any time with `app config set-credential-mode`:

| Mode | Behaviour |
|---|---|
| `store` *(default)* | Credentials are AES-GCM encrypted with a machine-bound key and stored in `credentials.db`. No password prompt after initial setup. |
| `session` | Credentials are never written to disk. You are prompted for a password whenever the cached authentication token expires. |

To switch modes:

```bash
app config set-credential-mode session   # removes credentials.db if present
app config set-credential-mode store     # prompts for password and stores it
```

---

## Data storage paths

catalysh follows OS conventions for application data:

| Platform | Base directory |
|---|---|
| Linux | `~/.config/catalysh/` |
| macOS | `~/Library/Application Support/catalysh/` |
| Windows | `%APPDATA%\catalysh\` |

Files in that directory:

| File | Contents |
|---|---|
| `config.yml` | URL, username, `verify_ssl`, `credential_mode` |
| `credentials.db` | AES-GCM encrypted credentials (store-on-device mode only) |
| `certs/` | Custom CA certificates installed via `app config install-cert` |

Command history is stored separately at `~/.catalysh/history` (10 000-line rolling buffer).

---

## NixOS

A `flake.nix` is included. Available outputs (per `flake-utils` default systems):

```bash
# Build the binary
nix build

# Run without installing
nix run

# Open a dev shell with Rust, clippy, rustfmt, cargo-watch, cargo-edit, and openssl
nix develop
```

The dev shell sets `PKG_CONFIG_PATH`, `OPENSSL_DIR`, and `OPENSSL_LIB_DIR` so `cargo build` works out of the box without extra environment setup.

---

## Troubleshooting

### Authentication errors

- **401 Unauthorized** — Wrong username or password. Run `app config reset-credentials` to re-enter them.
- **Config not found** — The wizard will run automatically; or run `app config reset` to force it.
- **Token expired mid-session** — catalysh reauthenticates transparently. If it loops, check that your Catalyst Center user account is not locked.

### SSL / TLS errors

- `certificate verify failed` — Your Catalyst Center uses a private CA. Install the cert: `app config install-cert <ca.pem>` and ensure `verify-ssl` is enabled.
- `unable to get local issuer certificate` — Same as above; you need the root CA, not the leaf certificate.
- Self-signed certificate for dev/lab — Use `app config set-verify-ssl disable` (not recommended for production).

### Connection errors

- Verify the URL has no trailing slash: `https://dnac.example.com` ✓, `https://dnac.example.com/` ✗.
- Confirm the host is reachable: `curl -k https://dnac.example.com/dna/system/api/v1/auth/token -u admin`.
- Check firewall rules — catalysh reaches Catalyst Center only on HTTPS (port 443).

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the architecture overview, development setup, and step-by-step guide to adding new commands. The detailed developer reference for adding API-backed show commands is in [docs/ADDING_COMMANDS.md](docs/ADDING_COMMANDS.md).
