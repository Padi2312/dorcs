use std::{fs, path::PathBuf};

use super::markdown_file::{MarkdownFile, MetaData};
use regex::Regex;
use serde_json::Value;

pub struct Loader {
    input_dir: String,
    pub documents: Vec<MarkdownFile>,
    pub assets: Vec<PathBuf>,
}

impl Loader {
    pub fn new(input_dir: String) -> Loader {
        Loader {
            input_dir,
            documents: Vec::new(),
            assets: Vec::new(),
        }
    }

    pub fn load(&mut self) {
        let input_dir = self.input_dir.clone();
        let files_list = self.load_documents(&input_dir);

        let markdown_files = files_list.0;
        let static_files = files_list.1;
        for file in markdown_files{
            let title = file.file_name().unwrap().to_string_lossy().to_string();
            let content = fs::read_to_string(&file).unwrap();
            let mut meta_data = self.parse_metadata(&content);
            let content = self.remove_meta_data(content.as_str());

            if meta_data.title.is_empty() {
                meta_data.title = title;
            }

            let path = file.to_string_lossy().to_string();
            let document = MarkdownFile {
                meta_data,
                content,
                path,
            };
            self.documents.push(document);
        }

        self.assets = static_files;
    }

    pub fn get_links(&self) -> Vec<Value> {
        let mut data = Vec::new();
        let sorted_documents = self.documents.clone();
        let mut sorted_documents: Vec<&MarkdownFile> = sorted_documents.iter().collect();
        sorted_documents.sort_by(|a, b| {
            a.meta_data
                .position
                .unwrap_or(999)
                .cmp(&b.meta_data.position.unwrap_or(0))
        });

        for doc in sorted_documents {
            let file_path = doc.path.replace(&self.input_dir, "");
            let file_path = file_path.split('.').next().unwrap();
            let file_path = format!("{}.html", file_path);
            data.push(serde_json::json!({
                "title": doc.meta_data.title,
                "link": file_path
            }));
        }
        data
    }

    fn load_documents(&mut self, dir: &str) -> (Vec<PathBuf>, Vec<PathBuf>) {
        let mut files_list = Vec::new();
        let mut asset_files_list = Vec::new();
        let dir_path = PathBuf::from(dir);

        if dir_path.is_dir() {
            if let Ok(entries) = fs::read_dir(&dir_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        // Convert the path to a string and recursively list files
                        let list = self.load_documents(path.to_str().unwrap_or(""));
                        files_list.extend(list.0);
                        asset_files_list.extend(list.1);
                    } else {
                        // Add the file's path as a string
                        // Check if file has a markdown extension
                        // If no markdown extension, add to static files list
                        if let Some(ext) = path.extension() {
                            if ext == "md" {
                                files_list.push(path);
                            } else {
                                asset_files_list.push(path);
                            }
                        }
                    }
                }
            }
        }

        (files_list, asset_files_list)
    }

    fn parse_metadata(&self, content: &str) -> MetaData {
        let re = Regex::new(r"(?s)^---\s*(.*?)\s*---").unwrap();
        let mut title = String::new();
        let mut position: Option<i32> = None;

        if let Some(cap) = re.captures_iter(content).next() {
            let meta_str = &cap[1];

            for line in meta_str.lines() {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();

                    // Match the key and update the struct fields accordingly
                    match key {
                        "title" => title = value.to_string(),
                        "position" => position = value.parse::<i32>().ok(),
                        _ => {} // Ignore unknown keys
                    }
                }
            }
        }

        MetaData { title, position }
    }

    fn remove_meta_data(&self, content: &str) -> String {
        let re = Regex::new(r"(?s)^---\s*(.*?)\s*---").unwrap();
        let content = re.replace(content, "");
        content.to_string()
    }
}
