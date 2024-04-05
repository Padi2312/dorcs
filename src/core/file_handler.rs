use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub struct FileHandler {
    paths: Vec<PathBuf>,
}

impl FileHandler {
    pub fn new(src_dir: &str) -> Self {
        let paths = FileHandler::get_all_paths(PathBuf::from(src_dir));
        FileHandler { paths }
    }

    pub fn get_markdown_files(&self) -> Vec<PathBuf> {
        self.paths
            .iter()
            .filter(|p| p.is_file() && p.extension().unwrap() == "md")
            .map(|p| p.to_path_buf())
            .collect::<Vec<PathBuf>>()
    }

    pub fn get_asset_files(&self) -> Vec<PathBuf> {
        self.paths
            .iter()
            .filter(|p| p.is_file() && p.extension().unwrap() != "md")
            .map(|p| p.to_path_buf())
            .collect::<Vec<PathBuf>>()
    }

    fn get_all_paths(src_dir: PathBuf) -> Vec<PathBuf> {
        WalkDir::new(&src_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_path_buf())
            .collect::<Vec<PathBuf>>()
    }
}
