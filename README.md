# proqbit

[![CI](https://github.com/kikefdezl/proqbit/actions/workflows/ci.yml/badge.svg)](https://github.com/kikefdezl/proqbit/actions/workflows/ci.yml)

Synchronize Proton VPN's forwarded port with qBittorrent's listening port

## Prerequisites

### 1. qBittorrent Web UI

In qBittorrent (**Options -> Web UI**):

- Check **Web User Interface (Remote control)**.
- *(Recommended)* Check **Bypass authentication for clients on localhost** if running locally. This avoids having to specify your credentials in plain text.

### 2. Build Tools

- Rust (`cargo`)
- [`just`](https://github.com/casey/just)

## Installation (as a systemd service)

```bash
just install
```

Builds and installs the binary to `~/.cargo/bin`, then enables and starts the `systemd --user` service.

## Configuration

Path: `~/.config/proqbit/config.toml`

```toml
[qbittorrent]
host = "http://localhost"
port = 8080
user = ""       # Leave empty if localhost bypass is enabled
password = ""   # Leave empty if localhost bypass is enabled

[proton]
log_file = "/home/<user>/.cache/Proton/VPN/logs/vpn-cli.log"
```

## Useful Commands

```bash
# View live logs
just logs

# Uninstall service and binary
just uninstall
```
