use dirs::{config_dir, home_dir};
use figment::Figment;
use figment::providers::{Format, Toml};
use serde::Deserialize;

use crate::error::Result;

const PROQBIT_CONFIG_DIR: &str = "proqbit";
const PROQBIT_CONFIG_FILE: &str = "config.toml";

const DEFAULT_QBITTORRENT_HOST: &str = "http://localhost";
const DEFAULT_QBITTORRENT_PORT: u16 = 8080;
const DEFAULT_PROTON_LOGFILE: &str = ".cache/Proton/VPN/logs/vpn-cli.log";

// ---

#[derive(Deserialize, Debug)]
pub struct QBitTorrentConfig {
    #[serde(default = "default_qbittorrent_host")]
    pub host: String,
    #[serde(default = "default_qbittorrent_port")]
    pub port: u16,
    #[serde(default)]
    pub user: String,
    #[serde(default)]
    pub password: String,
}

fn default_qbittorrent_host() -> String {
    DEFAULT_QBITTORRENT_HOST.into()
}

fn default_qbittorrent_port() -> u16 {
    DEFAULT_QBITTORRENT_PORT
}

#[derive(Deserialize, Debug)]
pub struct ProtonConfig {
    #[serde(default = "default_proton_logfile")]
    pub log_file: String,
}

fn default_proton_logfile() -> String {
    home_dir()
        .expect("Should have a home dir")
        .join(DEFAULT_PROTON_LOGFILE)
        .to_string_lossy()
        .into_owned()
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub qbittorrent: QBitTorrentConfig,
    pub proton: ProtonConfig,
}

impl Config {
    pub fn load() -> Result<Config> {
        let mut config_file = config_dir().expect("Should have a config dir");
        config_file.push(PROQBIT_CONFIG_DIR);
        config_file.push(PROQBIT_CONFIG_FILE);

        let config: Config = Figment::new().merge(Toml::file(config_file)).extract()?;

        Ok(config)
    }
}
