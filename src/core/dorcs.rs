use super::{dorcs_file::DorcsFile, embedded::Asset, file_handler::FileHandler, wizard};
use crate::auto_reload::auto_reload::DirWatcher;
use crate::config::config::Config;
use crate::navigation::{node::SerializableNavigationNode, tree::NavigationTree};
use crate::server::server::Server;
use serde_json::json;


use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

pub struct Dorcs {
    pub config: Config,
    pub file_handler: FileHandler,
    pub navigation_tree: NavigationTree,
}

impl Dorcs {
    pub fn new() -> Dorcs {
        let config = Config::load();
        let file_handler = FileHandler::new(&config.source);
        let navigation_tree = NavigationTree::new(&config.source);
        Dorcs {
            config,
            file_handler,
            navigation_tree,
        }
    }

    pub fn init(&mut self) {
        wizard::setup();

        self.write_files();
        self.copy_assets();
        self.write_navigation();
        self.write_page_settings();
        self.write_frontend_files();
    }

    fn write_files(&mut self) {
        let markdown_files = self.file_handler.get_markdown_files();
        for file in markdown_files {
            let file = DorcsFile::new(file);
            self.process_file(file)
        }
    }

    fn process_file(&mut self, file: DorcsFile) {
        self.navigation_tree.insert(&file);

        // Remove markdown extension and add html extension
        let file_path = file.path.clone();

        // Due to the frontend files, we need to put the generated HTML files
        // in a pages folder to avoid conflicts
        let file_path = file_path.strip_prefix(&self.config.source).unwrap();
        let file_path = Path::new("pages").join(file_path);
        let file_path = file_path.with_extension("html");

        let out_path = self.get_output_path(&file_path);

        // Convert content to HTML and write file to output path
        let content = file.to_html();
        fs::create_dir_all(&out_path.parent().unwrap()).unwrap();
        let mut file = File::create(&out_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    fn copy_assets(&self) {
        let asset_files = self.file_handler.get_asset_files();
        for file in asset_files {
            let out_path = self.get_output_path(&file);
            fs::create_dir_all(&out_path.parent().unwrap()).unwrap();
            fs::copy(file, out_path).unwrap();
        }
    }

    fn write_frontend_files(&self) {
        for file in Asset::iter() {
            let asset_file = Asset::get(&file);
            if asset_file.is_none() {
                panic!("No data found for file: {:?}", file);
            }
            let asset_file = asset_file.unwrap();
            let data = asset_file.data.as_ref();

            let asset_path = PathBuf::from(file.as_ref());
            let save_path = self.get_output_path(&asset_path);
            fs::create_dir_all(&save_path.parent().unwrap()).unwrap();
            let mut file = fs::File::create(save_path).unwrap();
            file.write_all(data).unwrap();
        }
    }

    fn get_output_path(&self, path: &Path) -> PathBuf {
        let stripped_path = path.strip_prefix(&self.config.source);
        if stripped_path.is_err() {
            Path::new(&self.config.output).join(path)
        } else {
            Path::new(&self.config.output).join(stripped_path.unwrap())
        }
    }

    fn write_navigation(&mut self) {
        let root_node = self.navigation_tree.get_root_node();
        let root_node = root_node.borrow().clone();
        let serialize_root = SerializableNavigationNode::from_navigation_node(&root_node);
        let children = serialize_root.children.clone();

        let save_path = Path::new(&self.config.output).join("routes.json");
        let save_dir = save_path.parent().unwrap();
        fs::create_dir_all(save_dir).unwrap();
        let mut file = File::create(save_path).unwrap();
        file.write_all(json!(children).to_string().as_bytes())
            .unwrap();
    }

    fn write_page_settings(&self) {
        let page_settings = self.config.page_settings.to_json();
        let page_settings_path = format!("{}/page_settings.json", self.config.output);
        std::fs::write(page_settings_path, page_settings).unwrap();
    }

    pub async fn serve(&mut self) {
        let output = self.config.output.clone();
        let source = self.config.source.clone();
        let port = self.config.server.port.clone();

        // Start the server in a separate thread.
        let server_handle = tokio::spawn(async move {
            Server::new(output.as_str(), &port).run().await.unwrap();
        });
        if self.config.server.auto_reload {
            println!(
                "Auto-reload enabled. Watching for file changes in '{}'",
                source
            );
            let (tx, mut rx) = tokio::sync::mpsc::channel::<PathBuf>(100);
            let dir_watcher = DirWatcher::new(PathBuf::from(source), tx);

            tokio::spawn(async move {
                // Start watching in a separate task
                if let Err(e) = dir_watcher.watch().await {
                    eprintln!("Error while watching directory: {:?}", e);
                }
            });

            // Listen for messages on the receiver in the main context
            while let Some(path) = rx.recv().await {
                println!("File changed or created: {:?}", path);
                let file = DorcsFile::new(path);
                self.process_file(file);
                // Process the file change or creation here
            }
        } else {
            let _ = server_handle.await;
        }
    }
}
