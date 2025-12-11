# catalysh

A Rust-based CLI shell for interacting with Cisco Catalyst Center through its API. catalysh provides a user-friendly interface for managing and monitoring your Cisco network infrastructure.

## Features

- Interactive shell with command history and auto-completion
- Secure credential storage
- Command-line completion with Tab
- Comprehensive network device management
- SSL certificate verification options

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/catalysh.git
cd catalysh
```

2. Build and run using Cargo:
```bash
cargo build --release
cargo run
```

## Initial Setup

On first run, catalysh will guide you through the setup process:

1. Enter your Cisco Catalyst Center URL (e.g., https://dnac.example.com)
2. Provide your username
3. Enter your password (input is hidden for security)
4. Choose whether to verify SSL certificates

## Available Commands

### Top-Level Commands

- `show` - Display information about network devices and configurations
- `config` - Enter configuration mode
- `app` - Application-specific commands
- `clear` - Clear the screen
- `exit` - Exit the application
- `help` - Display help information

### Show Commands

- `show ap` - Display AP information (config, rf-profile)
- `show client` - Display client information (detail, enrichment)
- `show device` - Display device information (list, detail, enrichment)
- `show issue` - Display issues in Catalyst Center
- `show site` - Display site information
- Additional show commands available via `show -help`

### App Commands

- `app config` - Manage application configuration (reset, show, set-url, set-verify-ssl, reset-credentials)
- `app version` - Display application version
- `app update` - Update to the latest release

### Command Help

Get help for any command by adding `-help`:
```bash
show device -help
config -help
app -help
```

## For Developers

### Adding New Commands

Adding new commands is now easier than ever! See [docs/ADDING_COMMANDS.md](docs/ADDING_COMMANDS.md) for a comprehensive guide.

Key features of the refactored command system:
- **No boilerplate**: Common operations (auth, config) are handled automatically
- **Consistent patterns**: All commands follow the same structure
- **Easy to test**: Modular design makes testing straightforward
- **Well documented**: Comprehensive guide with examples

## Data Storage and Security

catalysh prioritizes security in handling sensitive data:

1. **Configuration Storage**
- Configuration stored in `~/Library/Application Support/catalysh/config.yml`
- Contains non-sensitive settings like API URLs and preferences

2. **Credential Security**
- Credentials stored securely in an encrypted SQLite database
- Located at `~/Library/Application Support/catalysh/credentials.db`
- Passwords are encrypted using industry-standard encryption
- No plaintext passwords stored anywhere

3. **Session Management**
- Authentication tokens managed securely in memory
- Automatic token refresh handling
- Secure password input with hidden characters

## Troubleshooting

1. **Authentication Issues**
- Use `app config reset` to clear the full applicatoin config (You will be prompted to set it back up on the next command that utilizes the Catalyst Center API)
- Verify your Catalyst Center URL is correct using:
```
app config show

```
- Ensure your user account has appropriate permissions

2. **SSL Certificate Issues**
- During setup, choose 'n' for SSL verification if using self-signed certificates
- For production environments, always use valid certificates and enable verification

## Support

For issues, questions, or contributions:
- Open an issue on GitHub
- Check the CONTRIBUTING.md file for development guidelines
