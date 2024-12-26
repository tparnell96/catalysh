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

## Adding New Commands

1. **Command Handler Structure**
```rust
// src/handlers/your_command/mod.rs
pub struct YourCommandHandler {
    // Handler state
}

impl CommandHandler for YourCommandHandler {
    fn execute(&self, args: &[String]) -> Result<()> {
        // Implementation
    }
}
```

2. **Register Command**
- Add command to top-level dispatcher in `main.rs`
- Create help text and argument parsing
- Implement error handling

3. **Testing**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_command() {
        // Test implementation
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

