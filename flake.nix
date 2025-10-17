{
	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";

		hooks = {
			url = "github:cachix/git-hooks.nix";
			inputs.nixpkgs.follows = "nixpkgs";
		};

		fenix = {
			url = "github:nix-community/fenix";
			inputs.nixpkgs.follows = "nixpkgs";
		};
	};

	outputs = {
		self,
		hooks,
		fenix,
		nixpkgs,
	}: let
		inherit (nixpkgs) lib;

		systems = [
			"aarch64-linux"
			"i686-linux"
			"x86_64-linux"
			"aarch64-darwin"
			"x86_64-darwin"
		];

		forAllSystems = f:
			lib.genAttrs systems (system:
					f rec {
						pkgs =
							import nixpkgs {
								inherit system;
								overlays = [self.overlays.default];
							};

						nativeBuildInputs =
							builtins.attrValues {
								inherit (pkgs) pkg-config;
							};

						buildInputs =
							builtins.attrValues {
								inherit (pkgs) openssl sqlite;
							};

						inherit system;
					});
	in {
		overlays.default = final: prev: {
			rustToolchain = let
				pkgs = fenix.packages.${prev.stdenv.hostPlatform.system};
			in
				pkgs.combine (with pkgs.stable; [
						rustc
						cargo
						clippy
						rust-src
						pkgs.default.rustfmt
					]);
		};

		devShells =
			forAllSystems ({
					pkgs,
					system,
					buildInputs,
					nativeBuildInputs,
				}: let
					check = self.checks.${system}.pre-commit-check;
				in {
					default =
						pkgs.mkShell {
							inherit (check) shellHook;

							packages =
								check.enabledPackages
								++ (builtins.attrValues {
										inherit
											(pkgs)
											rustToolchain
											cargo-deny
											cargo-edit
											cargo-semver-checks
											cargo-watch
											rust-analyzer
											;
									})
								++ buildInputs
								++ nativeBuildInputs;

							env = {
								RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
								LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
							};
						};
				});

		packages =
			forAllSystems ({
					pkgs,
					buildInputs,
					nativeBuildInputs,
					...
				}: {
					default =
						(pkgs.makeRustPlatform {
								cargo = pkgs.rustToolchain;
								rustc = pkgs.rustToolchain;
							}).buildRustPackage {
							inherit buildInputs nativeBuildInputs;

							pname = "flussomodoro";
							version = "1.0.0";
							src = ./.;
							cargoLock.lockFile = ./Cargo.lock;

							OPENSSL_NO_VENDOR = 1;
						};
				});

		checks =
			forAllSystems ({system, ...}: let
					hooksLib = hooks.lib.${system};
				in {
					pre-commit-check =
						hooksLib.run {
							src = ./.;
							hooks = {
								convco.enable = true;
								alejandra.enable = true;
								clippy = {
									enable = true;
									package = fenix.packages.${system}.stable.clippy;
								};
								rustfmt = {
									enable = true;
									package = fenix.packages.${system}.default.rustfmt;
								};
								statix = {
									enable = true;
									settings.ignore = ["/.direnv"];
								};
							};
						};
				});

		formatter = forAllSystems ({pkgs, ...}: pkgs.alejandra);
	};
}
