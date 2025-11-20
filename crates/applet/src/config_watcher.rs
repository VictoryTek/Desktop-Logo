// Watches the config file for changes and triggers a callback
use notify::{Watcher, RecursiveMode, EventKind, RecommendedWatcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::thread;

pub fn watch_config<P: Into<PathBuf> + Send + 'static, F: Fn() + Send + 'static>(path: P, on_change: F) {
    let path = path.into();
    thread::spawn(move || {
        let (tx, rx) = channel();
        let mut watcher = RecommendedWatcher::new(tx, notify::Config::default()).expect("watcher");
        watcher.watch(&path, RecursiveMode::NonRecursive).expect("watch config");
        loop {
            match rx.recv() {
                Ok(event) => {
                    if let Ok(event) = event {
                        if matches!(event.kind, EventKind::Modify(_)) {
                            on_change();
                        }
                    }
                }
                Err(_) => break,
            }
        }
    });
}
