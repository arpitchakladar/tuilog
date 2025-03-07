# TUILog
TUILog is a simple login manager designed to run entirely in the TTY environment without a display server. This guide provides steps to install, configure, and run TUILog on your system.

# Preview
![Preview Image](https://github.com/user-attachments/assets/b1c5b568-7387-4f74-bfdd-77e819963889)

# NixOS Users
If you are using NixOS, you can integrate TUILog using the provided Nix flake. Add the flake to your configuration and enable the module:

1. Add the flake to your `flake.nix`:

```nix
inputs.tuilog.url = "github:arpitchakladar/tuilog";
```

2. Import the module in your configuration:

```nix
imports = [ tuilog.nixosModules.tuilog ];
```

3. Enable TUILog:

```nix
display-server.tuilog.enable = true;
display-server.tuilog.ttys = [ 1 ];
```

4. Rebuild your system:

```sh
sudo nixos-rebuild switch
```

# Other users
## Prerequisites
Before proceeding, disable any existing login manager on your system.

### Dependencies
Ensure the following dependencies are installed:

- `cargo`
- `clang`
- `libclang`
- `pkg-config`
- `linux-pam`

## Building TUILog
If TUILog is not already built, use the following commands:

```sh
cargo build --release
sudo cp target/release/tuilog /usr/local/bin/
```

## PAM Configuration
Copy PAM service file for TUILog:

```sh
sudo cp ./config/pam/tuilog /etc/pam.d/tuilog
```

## Systemd Service Setup
Create a systemd service file for TUILog on TTY1:

```sh
sudo cp ./config/systemd/tuilog.service /etc/systemd/system/tuilog@tty1.service
```
## Asset Configuration

Copy all contents from the `assets/` directory to `/etc/tuilog/`. The directory structure should look like this:

```sh
sudo cp -r ./assets/* /etc/tuilog/
```

## Starting Systemd Services
Stop and disable the default TTY services (switch to different tty before doing so):

```sh
sudo systemctl stop getty@tty1
sudo systemctl disable getty@tty1

sudo systemctl stop autovt@tty1
sudo systemctl disable autovt@tty1
```

Enable and start the TUILog service:

```sh
sudo systemctl enable tuilog@tty1
sudo systemctl start tuilog@tty1
```
