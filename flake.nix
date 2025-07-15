{
	description = "Flake for TUILog.";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.05";
	};

	outputs = { self, nixpkgs }:
	let
		pkgs = nixpkgs.legacyPackages."x86_64-linux";
		shellHook = ''
			export LIBCLANG_PATH=${pkgs.libclang.lib}/lib
			export CFLAGS=-I${pkgs.linux-pam}/include:$CFLAGS
			export CPPFLAGS=-I${pkgs.linux-pam}/include:$CPPFLAG
			export LDFLAGS=-L${pkgs.linux-pam}/lib:$LDFLAGS
		'';
	in {
		devShells."x86_64-linux".default = pkgs.mkShell {
			packages = with pkgs; [
				cargo
				rustfmt
				clang
				libclang.lib
				pkg-config
				linux-pam
			];

			shellHook = ''
${shellHook}
export TUILOG_CONFIG_DIR=$(pwd)/assets
'';
		};

		packages."x86_64-linux".tuilog =
		let
			cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
		in pkgs.rustPlatform.buildRustPackage rec {
			name = "tuilog-${cargoToml.package.version}";
			pname = "tuilog";
			version = cargoToml.package.version;
			src = ./.;
			buildInputs = with pkgs; [
				linux-pam
				pkg-config
			];

			cargoBuildOptions = [ "--release" ];
			cargoLock.lockFile = ./Cargo.lock;

			nativeBuildInputs = with pkgs; [
				clang
				libclang.lib
			];

			buildPhase = ''
				${shellHook}

				cargo build --release
			'';

			# skip check phase
			checkPhase = "true";

			installPhase = ''
				mkdir -p $out/bin
				cp target/release/tuilog $out/bin/
			'';
		};

		nixosModules.tuilog = { lib, config, pkgs, ... }: {
			options.services.displayManager.tuilog = {
				enable = lib.mkEnableOption "Enable TUILog login manager.";
				ttys = lib.mkOption {
					type = lib.types.listOf lib.types.int;
					description = "List of virtual terminal (TTY) numbers to use for TUILog login manager.";
					default = [ 1 ];
				};
				config = {
					cache_dir = lib.mkOption {
						type = lib.types.str;
						description = "Path to the cache directory for tuilog.";
						default = "/var/cache/tuilog";
					};
					ascii_art = {
						background = lib.mkOption {
							type = lib.types.str;
							description = "Path to the background ascii art under /etc/tuilog.";
							default = "background/nixos.txt";
						};
						background_art_color = lib.mkOption {
							type = lib.types.str;
							description = "Foreground color of the background ascii art.";
							default = "White";
						};
						error_icon = lib.mkOption {
							type = lib.types.str;
							description = "Path to the background ascii art under /etc/tuilog.";
							default = "icons/error.txt";
						};
					};
					sessions = {
						type = lib.types.listOf (lib.types.submodule {
							name = lib.mkOption {
								type = lib.types.str;
								description = "Display name of the session.";
							};
							exec = lib.mkOption {
								type = lib.types.str;
								description = "The command to start the session.";
							};
						});
						description = "Add sessions for tuilog.";
						default = [
							{
								name = "shell";
								exec = "";
							}
						];
					};
				};
			};

			config =
			let
				generateServices = tty:
				let
					stty = toString tty;
				in {
					"tuilog@tty${stty}" = {
						description = "TUILog Login Manager for tty${stty}.";
						after = [ "network.target" "systemd-user-sessions.service" ];
						requires = [ "systemd-user-sessions.service" ];
						serviceConfig = {
							ExecStart = "${self.packages."x86_64-linux".tuilog}/bin/tuilog";
							Restart = "always";
							RestartSec = "0";
							StandardInput = "tty";
							StandardOutput = "tty";
							TTYPath = "/dev/tty${stty}";
							TTYReset = "yes";
							TTYVHangup = "yes";
							TTYVTDisallocate = "yes";
							KillMode = "process";
							Environment = "XDG_SESSION_TYPE=tty XDG_SEAT=seat0 XDG_SESSION_CLASS=user XDG_VTNR=${stty} TTY=/dev/tty${stty}";
						};
						wantedBy = [ "multi-user.target" ];
					};

					"getty@tty${stty}".enable = false;
					"autovt@tty${stty}".enable = false;
				};
			in lib.mkIf config.services.displayManager.tuilog.enable {
				environment.systemPackages = with pkgs; [
					linux-pam
					systemd
					self.packages."x86_64-linux".tuilog
				];

				environment.etc."tuilog/background" = {
					source = ./assets/background;
				};
				environment.etc."tuilog/icons" = {
					source = ./assets/icons;
				};
				environment.etc."config.toml".text = with config.services.displayManager.tuilog.config; ''
cache_dir = "${cache_dir}"

[ascii_art]
background = "${ascii_art.background}"
background_art_color = "${ascii_art.background_art_color}"
error_icon = "${ascii_art.error_icon}"

${lib.concatMapStringsSep "\n" (session: "
[[sessions]]
name = \"${session.name}\"
exec = \"${session.exec}\"
") sessions}
'';

				security.pam.services.tuilog = {
					text = ''
						auth     required  ${pkgs.linux-pam}/lib/security/pam_unix.so
						account  required  ${pkgs.linux-pam}/lib/security/pam_unix.so
						password required  ${pkgs.linux-pam}/lib/security/pam_unix.so
						session  required  ${pkgs.linux-pam}/lib/security/pam_unix.so
						session  required  ${pkgs.systemd}/lib/security/pam_systemd.so
						session  required  ${pkgs.linux-pam}/lib/security/pam_loginuid.so
						session  required  ${pkgs.linux-pam}/lib/security/pam_env.so readenv=1 user_readenv=1
						session  required  ${pkgs.linux-pam}/lib/security/pam_limits.so
					'';
				};

				# Iterate over each TTY and define corresponding systemd service configurations.
				systemd.services =
					lib.mkMerge
						(map generateServices config.services.displayManager.tuilog.ttys);
			};
		};
	};
}

