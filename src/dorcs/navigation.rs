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
    pub is_section: bool,
}

// Generates a navigation structure from a given section.
pub fn generate_navigation(
    section: &Section,
    root_path: &Path,
) -> Result<Vec<Navigation>, Box<dyn std::error::Error>> {
    let mut navigation_list = Vec::new();

    // Process markdown files in the section.
    for file in &section.files {
        if is_markdown_file(file) {
            if let Ok(navigation) = create_navigation_from_file(file, root_path) {
                navigation_list.push(navigation);
            }
        }
    }

    // Process subsections recursively.
    if let Some(sections) = &section.sections {
        for subsection in sections {
            // Create navigation entries for subsections.
            if let Some(subsection_nav) = create_section_navigation(subsection, root_path)? {
                navigation_list.push(subsection_nav);
            }
        }
    }

    // Sort navigation entries by their position, placing entries without a position at the end.
    navigation_list.sort_by_key(|nav| nav.position.unwrap_or(std::i32::MAX));
    Ok(navigation_list)
}

// Creates a navigation entry for a given section.
fn create_section_navigation(
    section: &Section,
    root_path: &Path,
) -> Result<Option<Navigation>, Box<dyn std::error::Error>> {
    let mut children = Vec::new();

    // Process markdown files in the section.
    for file in &section.files {
        if is_markdown_file(file) {
            if let Ok(navigation) = create_navigation_from_file(file, root_path) {
                children.push(navigation);
            }
        }
    }

    // Process subsections recursively.
    if let Some(sections) = &section.sections {
        for subsection in sections {
            let subsection_nav = create_section_navigation(subsection, root_path)?;
            children.extend(subsection_nav);
        }
    }

    // Sort navigation entries by their position, placing entries without a position at the end.
    children.sort_by_key(|nav| nav.position.unwrap_or(std::i32::MAX));
    

    // Only create a navigation entry if the section has a title.
    let navigation = Navigation {
        title: section.path.file_name().unwrap().to_string_lossy().to_string(),
        url: "".to_string(),
        position: None,
        children: Some(children),
        is_section: true,
    };
    Ok(Some(navigation))
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
        is_section: false,
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
