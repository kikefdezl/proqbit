use std::path::{Path, PathBuf};

use notify::event::{AccessKind, AccessMode};
use notify::{Event, EventKind};

use crate::client::QBitTorrentClient;
use crate::error::Result;
use crate::parsing::extract_last_port;
use crate::watcher::PathWatcher;

type Port = u16;

pub struct AppConfig {
    pub log_file_path: PathBuf,
    pub watcher: PathWatcher,
    pub client: QBitTorrentClient,
}

pub struct App {
    log_file_path: PathBuf,
    port: Option<Port>,
    watcher: PathWatcher,
    client: QBitTorrentClient,
}

impl App {
    pub fn new(cfg: AppConfig) -> App {
        App {
            log_file_path: cfg.log_file_path.clone(),
            port: None,
            watcher: cfg.watcher,
            client: cfg.client,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        self.watcher.start()?;

        let events = self.watcher.events();

        let mut handler = Handler {
            log_file_path: &self.log_file_path,
            port: &mut self.port,
            client: &self.client,
        };

        for evt in events {
            match evt {
                Ok(e) => handler.handle_event(e)?,
                Err(e) => println!("Error in received event: {}", e),
            }
        }
        Ok(())
    }
}

struct Handler<'a> {
    log_file_path: &'a Path,
    port: &'a mut Option<u16>,
    client: &'a QBitTorrentClient,
}

impl Handler<'_> {
    fn handle_event(&mut self, evt: Event) -> Result<()> {
        let EventKind::Access(AccessKind::Close(AccessMode::Write)) = evt.kind else {
            return Ok(());
        };
        println!("Received event: {:?}", evt);

        let Some(port) = extract_last_port(self.log_file_path) else {
            println!("No last port forwarded found in logfile");
            return Ok(());
        };

        if *self.port != Some(port) {
            self.client.update_port(port)?;
            *self.port = Some(port);
        }
        Ok(())
    }
}
