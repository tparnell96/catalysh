# Contributing to catalysh

Thank you for your interest in contributing to catalysh! This document provides guidelines and information for contributors.

## Project Structure


```
catalysh/
├── src/
│   ├── api/                    # API interaction layer
│   │   ├── authentication/     # Auth handling
│   │   └── endpoints/          # API endpoint implementations
│   ├── app/                    # Application core
│   │   ├── auth_storage.rs     # Secure credential storage
│   │   └── config.rs           # Configuration management
│   ├── handlers/               # Command handlers
│   │   ├── show/              # Show command implementations
│   │   └── config/            # Config command implementations
│   └── main.rs                 # Application entry point
├── Cargo.toml                  # Project dependencies
└── README.md                   # Project documentation
```

## Development Setup

1. **Environment Setup**

```bash
# Clone the repository
git clone https://github.com/yourusername/catalysh.git
cd catalysh

# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build
```

2. **Development Dependencies**
- Rust 1.56 or newer
- SQLite development libraries
- OpenSSL development libraries

## Command Flow Architecture

catalysh uses a Clap-based command routing system:

1. Commands are defined as variants in a root Commands enum
2. Clap handles command-line argument parsing using derive macros
3. Commands are routed via pattern matching on the enum variants
4. Each command implementation lives in its own module
5. The command execution flow is:
- Parse command line args -> Commands enum
- Match on Commands variant
- Execute specific command implementation
- Return result

## Adding New Commands

### 1. Command Structure Overview

Commands in catalysh follow a modular structure with three main types:
- Show commands (data display)
- App commands (application control)
- Config commands (configuration REPL)

### 2. Command Implementation Steps

#### Command Enum Definition

```rust
// src/commands/mod.rs
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Your command's help text
    YourCommand {
        #[clap(long, short)]
        parameter: String,
    },
}
```

#### Command Implementation

```rust
impl Commands {
    pub async fn execute(self) -> Result<()> {
        match self {
            Commands::YourCommand { parameter } => {
                // Command implementation here
            }
        }
    }
}
```

#### Command Routing

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::YourCommand { parameter } => {
            // Route to your command implementation
        }
    }
}
```

### 3. API Integration

#### API Structure

```rust
// src/api/your_category/mod.rs
pub struct YourApiEndpoint {
    client: HttpClient,
}

impl YourApiEndpoint {
    pub async fn fetch_data(&self, params: &RequestParams) -> ApiResult<Response> {
        let response = self.client
            .get(&self.endpoint_url())
            .query(params)
            .send()
            .await?;
        
        response.json::<ResponseType>().await
    }
}
```

#### Response Handling

```rust
#[derive(Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "response")]
    data: Vec<DataItem>,
}

impl From<ApiResponse> for DisplayableOutput {
    fn from(response: ApiResponse) -> Self {
        // Transform API response to display format
    }
}
```

### 4. Adding a New Command

1. Define the command variant in the Commands enum
2. Add command parameters with Clap attributes
3. Implement the command execution in the match arm:

```rust
// In src/commands/mod.rs
#[derive(Subcommand, Debug)]
pub enum Commands {
    ExistingCommand { /* ... */ },
    // Add your new command:
    NewCommand {
        #[clap(long, short)]
        parameter: String,
    }
}

impl Commands {
    pub async fn execute(self) -> Result<()> {
        match self {
            // Add your command's execution:
            Commands::NewCommand { parameter } => {
                // Implementation here
            }
        }
    }
}
```

### 5. Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::mock_api;

    #[test]
    fn test_command_execution() {
        let mock_client = mock_api::setup();
        let handler = YourCommandHandler::new(mock_client);
        
        let result = handler.execute(&["arg1", "arg2"]);
        assert!(result.is_ok());
    }
}
```

## Code Style Guidelines

1. **Rust Conventions**
- Follow Rust naming conventions
- Use `rustfmt` for formatting
- Run `cargo clippy` for linting

2. **Error Handling**
- Use `Result` types for error handling
- Implement custom errors when needed
- Provide meaningful error messages

3. **Documentation**
- Document all public APIs
- Include examples in documentation
- Keep README and CONTRIBUTING up to date

## Pull Request Process

1. **Before Submitting**
- Create a new branch for your feature
- Write tests for new functionality
- Update documentation as needed

2. **Submission Guidelines**
- Provide clear PR description
- Reference any related issues
- Ensure all tests pass
- Follow up on review comments

3. **Review Process**
- Code review by maintainers
- CI checks must pass
- Documentation review
- Final approval and merge

## Development Workflow

1. **Creating Features**

```bash
# Create feature branch
git checkout -b feature/your-feature-name

# Make changes and commit
git commit -m "feat: add new feature"

# Push to remote
git push origin feature/your-feature-name
```

2. **Testing**

```bash
# Run tests
cargo test

# Run linter
cargo clippy

# Check formatting
cargo fmt --check
```

## Getting Help

- Join our discussion forum
- Check existing issues and PRs
- Contact maintainers

Thank you for contributing to catalysh!

