{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain with cross-compilation target
            (rust-bin.stable.latest.default.override {
              targets = [ "x86_64-unknown-linux-gnu" ];
            })

            # Rust development tools
            cargo-nextest
            cargo-watch
            just

            # Cross-compilation tools
            zig
            cargo-zigbuild
          ] ++ lib.optionals stdenv.isDarwin [
            # macOS SDK (provides all system frameworks)
            apple-sdk
          ];

          # Environment variables
          RUST_BACKTRACE = "1";
        };
      }
    );
}
