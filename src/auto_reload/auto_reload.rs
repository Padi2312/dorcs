use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

pub struct DirWatcher {
    dir_path: PathBuf,
    sender: Sender<PathBuf>, // Use Tokio's mpsc Sender to send paths
}

impl DirWatcher {
    pub fn new(dir_path: PathBuf, sender: Sender<PathBuf>) -> Self {
        DirWatcher { dir_path, sender }
    }

    pub async fn watch(&self) -> Result<(), Box<dyn Error>> {
        let sender = self.sender.clone();
        // Stelle sicher, dass dir_path als relativer Pfad vom aktuellen Verzeichnis aus behandelt wird
        let base_dir = env::current_dir()?.join(&self.dir_path);
        let dir_path = Arc::new(base_dir);
        let org_dir_path = Arc::new(self.dir_path.clone());

        tokio::task::spawn_blocking(move || {
            let dir_path_clone = Arc::clone(&dir_path);
            let mut watcher = RecommendedWatcher::new(
                move |res: Result<notify::Event, notify::Error>| match res {
                    Ok(event) => match event.kind {
                        EventKind::Create(_) | EventKind::Modify(_) => {
                            for path in event.paths {
                                // Behandle den Ereignispfad relativ zum aktuellen Arbeitsverzeichnis
                                let event_path = env::current_dir().unwrap().join(path);
                                if let Ok(rel_path) =
                                    event_path.strip_prefix(dir_path_clone.as_ref())
                                {
                                    let rel_path =
                                        PathBuf::from(org_dir_path.as_ref()).join(rel_path);
                                    if let Err(e) = sender.try_send(rel_path.to_path_buf()) {
                                        eprintln!("Failed to send path: {:?}", e);
                                    }
                                } else {
                                    eprintln!("Failed to strip prefix from path: {:?}", event_path);
                                }
                            }
                        }
                        _ => (),
                    },
                    Err(e) => eprintln!("watch error: {:?}", e),
                },
                Default::default(),
            )
            .unwrap();

            watcher
                .watch(dir_path.as_ref(), RecursiveMode::Recursive)
                .unwrap();

            std::thread::park();
        });

        Ok(())
    }
}
