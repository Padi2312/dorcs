use super::{meta_data::MetaData, sections::Section};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

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
    let mut meta_data = MetaData {
        title: "".to_string(),
        position: None,
    };
    let mut url = "".to_string();
    // Process markdown files in the section.
    for file in &section.files {
        if is_markdown_file(file) {
            if file.file_name().unwrap() == "index.md" {
                let content = fs::read_to_string(file)?;
                meta_data = MetaData::from_string(&content);
                let content = MetaData::remove_meta_data(&content);
                if !content.trim().is_empty() {
                    url = file
                        .with_extension("")
                        .strip_prefix(root_path)?
                        .to_string_lossy()
                        .to_string();
                }
            } else {
                if let Ok(navigation) = create_navigation_from_file(file, root_path) {
                    children.push(navigation);
                }
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

    let title = if meta_data.title.is_empty() {
        section
            .path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string()
    } else {
        meta_data.title
    };

    // Only create a navigation entry if the section has a title.
    let navigation = Navigation {
        title,
        url,
        position: meta_data.position,
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
    let meta_data = MetaData::from_string(&content);
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
