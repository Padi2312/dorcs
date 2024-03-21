use std::{fs, path::Path};

use regex::Regex;
use serde::{Deserialize, Serialize};

use super::sections::Section;

#[derive(Debug, Clone)]
pub struct MetaData {
    pub title: String,
    pub position: Option<i32>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Navigation {
    pub title: String,
    pub position: Option<i32>,
    pub url: String,
    pub children: Option<Vec<Navigation>>,
}

// Generates a navigation structure from a given root section.
pub fn generate_navigation(
    root_section: &Section,
) -> Result<Vec<Navigation>, Box<dyn std::error::Error>> {
    let mut navigation_list = Vec::new();
    // Process markdown files in the root section.
    for file in &root_section.files {
        if is_markdown_file(file) {
            if let Ok(navigation) = create_navigation_from_file(file, &root_section.path) {
                navigation_list.push(navigation);
            }
        }
    }
    // Process subsections recursively.
    if let Some(sections) = &root_section.sections {
        for section in sections {
            // Create navigation entries for subsections.
            if let Some(section_nav) = create_section_navigation(section, &root_section.path) {
                navigation_list.push(section_nav);
            }
        }
    }
    // Sort navigation entries by their position, placing entries without a position at the end.
    navigation_list.sort_by_key(|nav| nav.position.unwrap_or(std::i32::MAX));
    Ok(navigation_list)
}

// Creates a navigation entry from a markdown file.
fn create_navigation_from_file(
    file: &Path,
    root_path: &Path,
) -> Result<Navigation, Box<dyn std::error::Error>> {
    // TODO: Make this more efficient by reading the file only once. Currently we read it's content in SectionHanlder and here.
    let content = fs::read_to_string(file)?;
    let meta_data = parse_metadata(&content);
    let relative_url = file
        .with_extension("")
        .strip_prefix(root_path)?
        .to_string_lossy()
        .to_string();

    Ok(Navigation {
        title: meta_data.title,
        position: meta_data.position,
        url: format!("/{}", relative_url),
        children: None,
    })
}

// Creates a navigation entry for a section, including navigation entries for its files.
fn create_section_navigation(section: &Section, root_path: &Path) -> Option<Navigation> {
    let section_title = section.path.file_name()?.to_string_lossy().to_string();

    let children = section
        .files
        .iter()
        .filter_map(|file| {
            if is_markdown_file(file) {
                create_navigation_from_file(file, root_path).ok()
            } else {
                None
            }
        })
        .collect();

    Some(Navigation {
        title: section_title,
        position: None,
        url: "".to_string(),
        children: Some(children),
    })
}

// Checks if a file is a markdown file based on its extension.
fn is_markdown_file(file: &Path) -> bool {
    file.extension() == Some(std::ffi::OsStr::new("md"))
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
