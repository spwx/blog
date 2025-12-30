{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rustc
            cargo
            rustfmt
            clippy

            # Rust development tools
            cargo-nextest
            cargo-llvm-cov
            cargo-watch
            just

            # Cross-compilation tools
            zig
            cargo-zigbuild
          ] ++ lib.optionals stdenv.isDarwin [
            # macOS-specific frameworks
            darwin.apple_sdk.frameworks.SystemConfiguration
            darwin.apple_sdk.frameworks.CoreServices
          ];

          # Environment variables
          RUST_BACKTRACE = "1";
        };
      }
    );
}
