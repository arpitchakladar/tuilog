{
	description = "Flake for development environment for tuilog.";

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
				llvm
				clang
				libclang.lib
				pkg-config
				pam
				dbus.dev
			];

			shellHook = ''
				export LIBCLANG_PATH=${pkgs.libclang.lib}/lib
				export CFLAGS=-I${pkgs.pam}/include $CFLAGS
				export CPPFLAGS=-I${pkgs.pam}/include $CPPFLAG
				export LDFLAGS=-L${pkgs.pam}/lib $LDFLAGS
			'';
		};
	};
}
