# proqbit

Synchronize Proton VPN's forwarded port with qBittorrent's listening port

## Configuration

Configuration is done in `~/.config/proqbit/config.toml`.

Example:

```toml
[qbittorrent]
host = "http://localhost"
port = 8080
user = ""
password = ""

[proton]
log_file = "/home/user/.cache/Proton/VPN/logs/vpn-cli.log"
```

The values in the example above are the defaults.
