# Build and rename for Windows x86_64
cargo build --release --bin catalysh --target x86_64-pc-windows-gnu
mv ./target/x86_64-pc-windows-gnu/release/catalysh.exe ./target/x86_64-pc-windows-gnu/release/catalysh-windows-x86_64.exe

# Build and rename for macOS ARM64
cargo build --release --bin catalysh --target aarch64-apple-darwin
mv ./target/aarch64-apple-darwin/release/catalysh ./target/aarch64-apple-darwin/release/catalysh-macos-arm64

# Build and rename for Windows installer binary
cargo build --release --bin windows_installer --target x86_64-pc-windows-gnu
mv ./target/x86_64-pc-windows-gnu/release/windows_installer.exe ./target/x86_64-pc-windows-gnu/release/windows_installer-windows-x86_64.exe
