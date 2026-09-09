use std::path::{Path, PathBuf};

use notify::event::{AccessKind, AccessMode};
use notify::{Event, EventKind};
use notify_rust::{Notification, Timeout};

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
    watcher: PathWatcher,
    client: QBitTorrentClient,

    port: Option<Port>,
    log_offset: usize,
}

impl App {
    pub fn new(cfg: AppConfig) -> App {
        App {
            log_file_path: cfg.log_file_path.clone(),
            port: None,
            watcher: cfg.watcher,
            client: cfg.client,
            log_offset: 0,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        self.watcher.start()?;

        let events = self.watcher.events();

        let mut handler = Handler {
            log_file_path: &self.log_file_path,
            client: &self.client,
            port: &mut self.port,
            offset: &mut self.log_offset,
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
    client: &'a QBitTorrentClient,
    port: &'a mut Option<u16>,
    offset: &'a mut usize,
}

impl Handler<'_> {
    fn handle_event(&mut self, evt: Event) -> Result<()> {
        let EventKind::Access(AccessKind::Close(AccessMode::Write)) = evt.kind else {
            return Ok(());
        };
        println!("Received event: {:?}", evt);

        let Some(match_) = extract_last_port(self.log_file_path, *self.offset) else {
            println!("No last port forwarded found in logfile");
            return Ok(());
        };

        if *self.port != Some(match_.port) {
            self.client.update_port(match_.port)?;
            *self.port = Some(match_.port);
            *self.offset = match_.offset;
        }

        let msg = format!("Updated QBitTorrent port: {}", match_.port);
        println!("{}", msg);
        Notification::new()
            .summary("ProqBit")
            .body(&msg)
            .timeout(Timeout::Milliseconds(10000))
            .show()?;
        Ok(())
    }
}
