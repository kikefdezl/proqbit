use std::env::temp_dir;
use std::fs::File;
use std::time::Duration;

use serde::Serialize;
use thiserror::Error;
use ureq::Agent;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("host is required")]
    MissingHost,
    #[error("port is required")]
    MissingPort,
    #[error("invalid credentials: {0}")]
    InvalidCredentials(String),
    #[error("ureq error: {0}")]
    Ureq(#[from] ureq::Error),
    #[error("serde json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[derive(Default)]
pub struct QBitTorrentCredentials {
    user: String,
    password: String,
}

impl QBitTorrentCredentials {
    pub fn new(user: impl Into<String>, password: impl Into<String>) -> QBitTorrentCredentials {
        QBitTorrentCredentials {
            user: user.into(),
            password: password.into(),
        }
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Default)]
pub struct QBitTorrentClientConfig {
    host: String,
    port: u16,
    creds: Option<QBitTorrentCredentials>,
}

impl QBitTorrentClientConfig {
    pub fn with_host(mut self, host: impl Into<String>) -> QBitTorrentClientConfig {
        self.host = host.into();
        self
    }

    pub fn with_port(mut self, port: u16) -> QBitTorrentClientConfig {
        self.port = port;
        self
    }

    pub fn with_creds(mut self, creds: Option<QBitTorrentCredentials>) -> QBitTorrentClientConfig {
        self.creds = creds;
        self
    }

    pub fn build(self) -> Result<QBitTorrentClient, ClientError> {
        if self.host.is_empty() {
            return Err(ClientError::MissingHost);
        }
        if self.port == 0 {
            return Err(ClientError::MissingPort);
        }
        if let Some(ref creds) = self.creds {
            if creds.user().is_empty() {
                return Err(ClientError::InvalidCredentials(
                    "User can't be empty".into(),
                ));
            }
            if creds.password().is_empty() {
                return Err(ClientError::InvalidCredentials(
                    "Password can't be empty".into(),
                ));
            }
        }
        let agent = Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(5)))
            .build()
            .into();

        Ok(QBitTorrentClient {
            host: self.host,
            port: self.port,
            creds: self.creds,
            agent,
        })
    }
}

pub struct QBitTorrentClient {
    host: String,
    port: u16,
    creds: Option<QBitTorrentCredentials>,
    agent: ureq::Agent,
}

impl QBitTorrentClient {
    pub fn update_port(&self, port: u16) -> Result<(), ClientError> {
        let url = format!("{}:{}/api/v2/auth/login", self.host, self.port);

        if let Some(ref creds) = self.creds {
            self.agent
                .post(url)
                .send_form([("username", creds.user()), ("password", creds.password())])?;
            println!("Sent auth request to qbit");

            let mut cookies_path = temp_dir();
            cookies_path.push("proqbit_cookies.json");
            let mut cookies_file = File::create(cookies_path).unwrap();
            let jar = self.agent.cookie_jar_lock();
            jar.save_json(&mut cookies_file).unwrap();
            jar.release();
        }

        #[derive(Serialize)]
        struct Data {
            listen_port: u16,
        }

        let url = format!("{}:{}/api/v2/app/setPreferences", self.host, self.port);
        let data = Data { listen_port: port };
        let json_str = serde_json::to_string(&data)?;
        self.agent
            .post(url)
            .send_form([("json", json_str.as_str())])?;

        println!("Updated QBitTorrent listening port to {}", port);

        Ok(())
    }
}
