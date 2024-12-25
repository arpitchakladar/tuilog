{
	description = "Flake for TUILog.";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
	};

	outputs = { self, nixpkgs }:
	let
		pkgs = nixpkgs.legacyPackages."x86_64-linux";
	in {
		devShells."x86_64-linux".default = pkgs.mkShell {
			packages = with pkgs; [
				cargo
				clang
				libclang.lib
				pkg-config
				linux-pam
			];

			shellHook = ''
				export LIBCLANG_PATH=${pkgs.libclang.lib}/lib
				export CFLAGS=-I${pkgs.linux-pam}/include $CFLAGS
				export CPPFLAGS=-I${pkgs.linux-pam}/include $CPPFLAG
				export LDFLAGS=-L${pkgs.linux-pam}/lib $LDFLAGS
			'';
		};

		packages."x86_64-linux".tuilog =
		let
			cargoVersion =
				let
					versionMatch = builtins.match "version = \"([^\"]+)\"" (builtins.readFile ./Cargo.toml);
				in
					if versionMatch != null then builtins.elemAt versionMatch 0 else "0.0.0";
		in pkgs.rustPlatform.buildRustPackage rec {
			pname = "tuilog";
			version = cargoVersion;
			src = ./.;
			buildInputs = with pkgs; [
				linux-pam
				pkg-config
			];

			cargoBuildOptions = [ "--release" ];
			cargoLock.lockFile = ./Cargo.lock;

			nativeBuildInputs = with pkgs; [
				clang               # clang should be added to nativeBuildInputs
				libclang.lib
			];

			buildPhase = ''
				export LIBCLANG_PATH=${pkgs.libclang.lib}/lib
				export CFLAGS=-I${pkgs.linux-pam}/include $CFLAGS
				export CPPFLAGS=-I${pkgs.linux-pam}/include $CPPFLAGS
				export LDFLAGS=-L${pkgs.linux-pam}/lib $LDFLAGS
				export CPATH=${pkgs.linux-pam}/include:$CPATH

				cargo build --release
			'';

			checkPhase = "true";

			installPhase = ''
				mkdir -p $out/bin
				cp target/release/tuilog $out/bin/
			'';
		};

		nixosModules.tuilog = { lib, config, pkgs, ... }: {
			options.display-server.tuilog = {
				enable = lib.mkEnableOption "Enable TUILog login manager.";
				vtnr = lib.mkOption {
					description = "Virtual terminal (tty) number to use for TUILog login manager.";
					type = lib.types.int;
					default = 1;
				};
			};

			config =
			let
				vtnr = builtins.toString config.display-server.tuilog.vtnr;
				ttyPath = "/dev/tty${vtnr}";
			in lib.mkIf config.display-server.tuilog.enable {
				services.xserver.autorun = false;
				services.libinput.enable = true;
				systemd.services."getty@tty${vtnr}".enable = false;
				systemd.services."autovt@tty${vtnr}".enable = false;

				environment.systemPackages = with pkgs; [
					linux-pam
					systemd
					self.packages."x86_64-linux".tuilog
				];

				environment.etc.tuilog = {
					source = ./assets;
				};

				security.pam.services.tuilog = {
					text = ''
						auth     required  ${pkgs.linux-pam}/lib/security/pam_unix.so
						account  required  ${pkgs.linux-pam}/lib/security/pam_unix.so
						password required  ${pkgs.linux-pam}/lib/security/pam_unix.so
						session  required  ${pkgs.linux-pam}/lib/security/pam_unix.so
						session  required  ${pkgs.systemd}/lib/security/pam_systemd.so
						session  required  ${pkgs.linux-pam}/lib/security/pam_loginuid.so
					'';
				};

				systemd.services.tuilog = {
					description = "TUILog Login Manager";
					after = [ "network.target" "systemd-user-sessions.service" ];
					requires = [ "systemd-user-sessions.service" ];
					serviceConfig = {
						ExecStart = "${self.packages."x86_64-linux".tuilog}/bin/tuilog";
						Restart = "always";
						RestartSec = "0";
						StandardInput = "tty";
						StandardOutput = "tty";
						TTYPath = ttyPath;
						TTYReset = "yes";
						TTYVHangup = "yes";
						TTYVTDisallocate = "yes";
						KillMode = "process";
						Environment = "XDG_SESSION_TYPE=tty XDG_SEAT=seat0 XDG_SESSION_CLASS=user XDG_VTNR=${vtnr} TTY=${ttyPath}";
					};
					wantedBy = [ "multi-user.target" ];
				};
			};
		};
	};
}
