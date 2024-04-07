use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

pub struct DirWatcher<F>
where
    F: Fn(PathBuf) + Send + Sync + 'static,
{
    dir_path: PathBuf,
    callback: Arc<F>,
}

impl<F> DirWatcher<F>
where
    F: Fn(PathBuf) + Send + Sync + 'static,
{
    pub fn new(dir_path: PathBuf, callback: F) -> Self {
        DirWatcher {
            dir_path,
            callback: Arc::new(callback),
        }
    }

    pub async fn watch(&self) -> Result<(), Box<dyn Error>> {
        let callback = self.callback.clone();
        let dir_path = self.dir_path.clone();

        tokio::task::spawn_blocking(move || {
            let mut watcher = RecommendedWatcher::new(
                move |res: Result<notify::Event, notify::Error>| match res {
                    Ok(event) => match event.kind {
                        EventKind::Create(_) | EventKind::Modify(_) => {
                            for path in event.paths {
                                callback(path);
                            }
                        }
                        _ => (),
                    },
                    Err(e) => eprintln!("watch error: {:?}", e),
                },
                Default::default(),
            )
            .unwrap();

            watcher.watch(&dir_path, RecursiveMode::Recursive).unwrap();

            // Block this thread indefinitely. In a real application, you might use a more sophisticated method to wait and handle shutdown signals.
            std::thread::park();
        });

        Ok(())
    }
}
