use std::{fs, path::PathBuf};

use crate::markdown_file::{MetaData, MarkdownFile};
use regex::Regex;
use serde_json::Value;

pub struct Loader {
    pub documents: Vec<MarkdownFile>,
}

impl Loader {
    pub fn new() -> Loader {
        Loader {
            documents: Vec::new(),
        }
    }

    pub fn load(&mut self, input_dir: &str) {
        let files_list = self.load_documents(input_dir);
        for file in files_list {
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
    }

    pub fn get_links(&self) -> Vec<Value> {
        let mut data = Vec::new();
        let sorted_documents = self.documents.clone();
        let mut sorted_documents: Vec<&MarkdownFile> = sorted_documents.iter().collect();
        sorted_documents.sort_by(|a, b| {
            a.meta_data
                .position
                .unwrap_or(0)
                .cmp(&b.meta_data.position.unwrap_or(0))
        });

        for doc in sorted_documents {
            let file_path = doc.path.replace("docs", "");
            let file_path = file_path.split('.').next().unwrap();
            let file_path = format!("{}.html", file_path);
            data.push(serde_json::json!({
                "title": doc.meta_data.title,
                "link": file_path
            }));
        }
        data
    }

    fn load_documents(&mut self, dir: &str) -> Vec<PathBuf> {
        let mut files_list = Vec::new();
        let dir_path = PathBuf::from(dir);

        if dir_path.is_dir() {
            if let Ok(entries) = fs::read_dir(&dir_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        // Convert the path to a string and recursively list files
                        files_list.extend(self.load_documents(path.to_str().unwrap_or("")));
                    } else {
                        // Add the file's path as a string
                        files_list.push(path);
                    }
                }
            }
        }

        files_list
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
