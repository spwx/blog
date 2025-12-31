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
