{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    supportedSystems = ["x86_64-linux" "aarch64-linux"];
    forEachSupportedSystem = f:
      nixpkgs.lib.genAttrs supportedSystems (system:
        f {
          pkgs = import nixpkgs {
            inherit system;
            overlays = [rust-overlay.overlays.default self.overlays.default];
          };
        });
  in {
    overlays.default = final: prev: {
      rustToolchain = let
        rust = prev.rust-bin;
      in
        if builtins.pathExists ./rust-toolchain.toml then
          rust.fromRustupToolchainFile ./rust-toolchain.toml
        else
          rust.stable.latest.default.override {
            extensions = ["rust-src" "rustfmt"];
          };
    };

    devShells = forEachSupportedSystem ({ pkgs }: let
      buildInputs = with pkgs; [
        openssl
        sqlite.dev
      ];
    in {
      default = pkgs.mkShell {
        inherit buildInputs;

        packages = with pkgs; [
          rustToolchain
          pkg-config
          cargo-deny
          cargo-edit
          cargo-semver-checks
          cargo-watch
          rust-analyzer
        ];

        env = {
          RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
          LD_LIBRARY_PATH = "${nixpkgs.lib.makeLibraryPath buildInputs}";
        };
      };
    });
  };
}
