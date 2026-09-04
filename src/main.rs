use std::path::PathBuf;

use app::{App, AppConfig};
use client::{QBitTorrentClientConfig, QBitTorrentCredentials};
use config::Config;
use error::Result;
use watcher::PathWatcher;

mod app;
mod client;
mod config;
mod error;
mod parsing;
mod watcher;

fn main() -> Result<()> {
    let config = Config::load()?;
    println!("Loaded config{:?}", config);

    let log_file = PathBuf::from(config.proton.log_file);
    let watcher = PathWatcher::try_new(&log_file).expect("PathWatcher should init without error");

    let creds = if !config.qbittorrent.user.is_empty() && !config.qbittorrent.password.is_empty() {
        Some(QBitTorrentCredentials::new(
            config.qbittorrent.user,
            config.qbittorrent.password,
        ))
    } else {
        None
    };

    let client_config = QBitTorrentClientConfig::default()
        .with_host(config.qbittorrent.host)
        .with_port(config.qbittorrent.port)
        .with_creds(creds);
    let client = client_config.build()?;

    let config = AppConfig {
        log_file_path: log_file,
        watcher,
        client,
    };

    let mut app = App::new(config);

    app.start()?;
    Ok(())
}
