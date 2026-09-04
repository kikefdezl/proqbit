use std::path::{Path, PathBuf};
use std::sync::mpsc;

use notify::{Event, RecommendedWatcher, RecursiveMode, Result, Watcher};

pub struct PathWatcher {
    path: PathBuf,
    watcher: RecommendedWatcher,
    rx: mpsc::Receiver<Result<Event>>,
}

impl PathWatcher {
    pub fn try_new(path: &Path) -> Result<PathWatcher> {
        let (tx, rx) = mpsc::channel::<Result<Event>>();

        let watcher = notify::recommended_watcher(tx)?;

        Ok(PathWatcher {
            path: path.to_path_buf(),
            watcher,
            rx,
        })
    }

    pub fn start(&mut self) -> Result<()> {
        println!("Watching {:?}", self.path);
        self.watcher
            .watch(&self.path, RecursiveMode::NonRecursive)?;
        Ok(())
    }

    pub fn events(&self) -> impl Iterator<Item = Result<Event>> + '_ {
        self.rx.iter()
    }
}
