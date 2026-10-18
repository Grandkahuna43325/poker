{
  description = "A devShell example with a run app";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

  };
  outputs = { self, nixpkgs, rust-overlay, flake-utils, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        mkToolchain = fenix.packages.${system}.combine;
        toolchain = fenix.packages.${system}.stable;
        target1 = fenix.packages.${system}.targets."wasm32-unknown-unknown".stable;
        target2 = fenix.packages.${system}.targets."x86_64-unknown-linux-gnu".stable;

        rustTargets = mkToolchain (with toolchain; [
          cargo
          rustc
          rust-src
          target1.rust-std
          target2.rust-std
        ]);

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            openssl
            pkg-config
            libpq
            bacon
            trunk
            # wasm-bindgen-cli
            rustTargets
          ];
          shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [ pkgs.openssl pkgs.libpq ]};

            preexec() {
              printf "\033]0;$1\007"
            }
            trap 'preexec "$BASH_COMMAND"' DEBUG
          '';
        };
      }
    );
}
