{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    hooks.url = "github:cachix/git-hooks.nix";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    hooks,
    nixpkgs,
    rust-overlay,
  }: let
    inherit (nixpkgs) lib;

    systems = [
      "aarch64-linux"
      "i686-linux"
      "x86_64-linux"
      "aarch64-darwin"
      "x86_64-darwin"
    ];

    forAllSystems = lib.genAttrs systems;

    overlaysFor = system: [
      rust-overlay.overlays.default
      self.overlays.default
    ];
  in {
    overlays.default = final: prev: {
      rustToolchain = let
        rust = prev.rust-bin;
      in
        if builtins.pathExists ./rust-toolchain.toml
        then rust.fromRustupToolchainFile ./rust-toolchain.toml
        else rust.stable.latest.default.override {extensions = ["rust-src" "rustfmt"];};
    };

    devShells = forAllSystems (system: let
      check = self.checks.${system}.pre-commit-check;
      pkgs = import nixpkgs {
        inherit system;
        overlays = overlaysFor system;
      };

      buildInputs = builtins.attrValues {
        inherit (pkgs) openssl;
        inherit (pkgs.sqlite) dev;
      };
    in {
      default = pkgs.mkShell {
        inherit (check) shellHook;

        packages =
          check.enabledPackages
          ++ (builtins.attrValues {
            inherit
              (pkgs)
              pkg-config
              rustToolchain
              cargo-deny
              cargo-edit
              cargo-semver-checks
              cargo-watch
              rust-analyzer
              ;
          })
          ++ buildInputs;

        env = {
          RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      };
    });

    checks = forAllSystems (system: let
      lib = hooks.lib.${system};
    in {
      pre-commit-check = lib.run {
        src = ./.;
        hooks = {
          alejandra.enable = true;
          rustfmt.enable = true;
          clippy.enable = true;
          convco.enable = true;
          statix = {
            enable = true;
            settings.ignore = ["/.direnv"];
          };
        };
      };
    });

    formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.alejandra);
  };
}
