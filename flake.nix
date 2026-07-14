{
  description = "catalysh — a CLI for Cisco Catalyst Center";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        # Use the Rust toolchain specified in rust-toolchain.toml if present,
        # otherwise fall back to the stable channel.
        rustToolchain = if builtins.pathExists ./rust-toolchain.toml
          then pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml
          else pkgs.rust-bin.stable.latest.default;

        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
        ];

        buildInputs = with pkgs; [
          openssl
        ];
      in
      {
        # ── Packages ──────────────────────────────────────────────────────────
        packages = {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "catalysh";
            version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;

            src = pkgs.lib.cleanSource ./.;

            cargoLock.lockFile = ./Cargo.lock;

            inherit nativeBuildInputs buildInputs;

            # rusqlite uses the "bundled" feature, so no system sqlite needed.
            # OpenSSL is required by reqwest's native-tls feature.
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

            meta = with pkgs.lib; {
              description = "A command-line utility for interacting with Cisco Catalyst Center";
              homepage = "https://github.com/hexabyte8/catalysh";
              license = licenses.mit;
              maintainers = [ ];
              mainProgram = "catalysh";
            };
          };
        };

        # ── Dev shell ─────────────────────────────────────────────────────────
        devShells.default = pkgs.mkShell {
          inherit buildInputs;
          nativeBuildInputs = nativeBuildInputs ++ (with pkgs; [
            cargo-watch
            cargo-edit
            clippy
            rustfmt
          ]);

          # Expose OpenSSL headers and pkg-config paths for cargo build
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
        };

        # ── Checks (cargo test) ────────────────────────────────────────────────
        checks.default = self.packages.${system}.default;
      }
    );
}
