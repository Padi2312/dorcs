use markdown::{CompileOptions, Options};
use regex::Regex;
use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct MetaData {
    pub title: String,
    pub position: Option<i32>,
}

impl MetaData {
    pub fn from_string(content: &String) -> MetaData {
        Self::parse_metadata(content)
    }

    // Parses the metadata from a markdown file.
    fn parse_metadata(raw_content: &String) -> MetaData {
        let re = Regex::new(r"(?s)^---\s*(.*?)\s*---").unwrap();
        let mut title = String::new();
        let mut position: Option<i32> = None;

        if let Some(cap) = re.captures_iter(raw_content.as_str()).next() {
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

    pub fn remove_meta_data(content: &String) -> String {
        let re = Regex::new(r"(?s)^---\s*(.*?)\s*---").unwrap();
        let content = re.replace(content, "");
        content.to_string()
    }
}

pub struct DorcsFile {
    pub path: PathBuf,
    pub meta_data: MetaData,
    pub content: String,
}

impl DorcsFile {
    pub fn new(path: PathBuf) -> DorcsFile {
        let content = fs::read_to_string(&path);
        let content = match content {
            Ok(content) => content,
            Err(_) => {
                println!("Error reading file: {:?}", path);
                String::new()
            }
        };

        let meta_data = MetaData::from_string(&content);

        let content = MetaData::remove_meta_data(&content);

        DorcsFile {
            path,
            meta_data,
            content,
        }
    }

    pub fn to_html(&self) -> String {
        let options = &Options {
            compile: CompileOptions {
                allow_dangerous_html: true,
                ..CompileOptions::default()
            },
            ..Options::gfm()
        };
        let html = markdown::to_html_with_options(&self.content, options);
        if html.is_err() {
            panic!("Error rendering markdown: {:?}", html.err());
        }
        html.unwrap()
    }
}
// TODO: Load the files and properly insert them into the navigation tree.
