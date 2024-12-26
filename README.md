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
- `exit` - Exit the application
- `help` - Display help information

### Show Commands

- `show ap rf-profile` - Display AP RF profiles
- `show device` - List network devices
- Additional show commands available via `show -help`

### App Configuration

- `app config reset` - Reset application configuration
- `app config show` - Display current configuration

### Command Help

Get help for any command by adding `-help`:
```bash
show device -help
config -help
```

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
- Use `app config reset` to clear stored credentials
- Verify your Catalyst Center URL is correct
- Ensure your user account has appropriate permissions

2. **SSL Certificate Issues**
- During setup, choose 'n' for SSL verification if using self-signed certificates
- For production environments, always use valid certificates and enable verification

## Support

For issues, questions, or contributions:
- Open an issue on GitHub
- Check the CONTRIBUTING.md file for development guidelines
