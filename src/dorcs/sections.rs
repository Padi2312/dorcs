use markdown::{CompileOptions, Options};
use serde_json::json;
use std::{fs, io::Write, path::PathBuf};

use super::{
    meta_data::MetaData,
    templates::{CSS_TEMPLATE_FILE, HTML_FILE, JS_FILE},
};
use crate::dorcs::navigation::generate_navigation;

#[derive(Debug)]
pub struct Section {
    pub path: PathBuf,
    pub sections: Option<Vec<Section>>,
    pub files: Vec<PathBuf>,
}

pub struct SectionGenerator {
    pub source: PathBuf,
}

impl SectionGenerator {
    pub fn new(source: PathBuf) -> SectionGenerator {
        SectionGenerator { source }
    }

    pub fn construct(&self) -> Section {
        let mut section = Section {
            path: self.source.clone(),
            sections: Some(Vec::new()),
            files: Vec::new(),
        };

        self.construct_section(&mut section);
        section
    }

    fn construct_section(&self, section: &mut Section) {
        let entries = fs::read_dir(&section.path).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let mut new_section = Section {
                    path: path.clone(),
                    sections: None,
                    files: Vec::new(),
                };
                self.construct_section(&mut new_section);
                if let Some(sections) = &mut section.sections {
                    sections.push(new_section);
                } else {
                    section.sections = Some(vec![new_section]);
                }
            } else {
                section.files.push(path);
            }
        }
    }
}

pub struct SectionBuilder {
    pub root_section: Section,
}

impl SectionBuilder {
    pub fn new(root_section: Section) -> SectionBuilder {
        SectionBuilder { root_section }
    }

    pub fn execute(&self, out_dir: &str) {
        // println!("Root section: {:#?}", self.root_section);
        let navigation_list = generate_navigation(&self.root_section, &self.root_section.path);
        if navigation_list.is_err() {
            panic!("No navigation list generated");
        }
        let navigation_list = navigation_list.unwrap();
        // println!("Navigation list: {:#?}", navigation_list);
        // Write navagation list as json to routes.json in output dir
        let save_path = PathBuf::from(&out_dir).join("routes.json");
        let final_out_dir = PathBuf::from(&out_dir).join("pages");
        let final_out_dir_str = final_out_dir.to_str().unwrap();

        let save_dir = save_path.parent().unwrap();
        fs::create_dir_all(save_dir).unwrap();
        let mut file = fs::File::create(save_path).unwrap();
        file.write_all(json!(navigation_list).to_string().as_bytes())
            .unwrap();

        self.process(&self.root_section, final_out_dir_str);
        self.copy_templates_to_output(out_dir);
    }

    fn process(&self, section: &Section, out_dir: &str) {
        for file in &section.files {
            // If file is .md file we process it
            // otherwise we assume it's an asset file
            if file.extension().unwrap() != "md" {
                self.copy_asset_file(file, out_dir);
            } else {
                if file.file_name().unwrap() == "index.md" {
                    // Check if file is empty after removing meta   data
                    let raw_content = fs::read_to_string(file).unwrap();
                    let parsed_content = MetaData::remove_meta_data(&raw_content);
                    if parsed_content.is_empty() {
                        continue;
                    }
                }
                self.process_md_file(file, out_dir);
            }
        }

        if let Some(sections) = &section.sections {
            for section in sections {
                self.process(section, out_dir);
            }
        }
    }

    fn copy_asset_file(&self, file: &PathBuf, out_dir: &str) {
        let relative_path = file.strip_prefix(&self.root_section.path).unwrap();
        // Get parentdir from outdir due to /pages directory
        let binding = PathBuf::from(out_dir);
        let out_dir = binding.parent().unwrap().to_str().unwrap();
        let save_path = PathBuf::from(out_dir).join(relative_path);
        let save_dir = save_path.parent().unwrap();
        fs::create_dir_all(save_dir).unwrap();
        fs::copy(file, save_path).unwrap();
    }

    fn copy_templates_to_output(&self, out_dir: &str) {
        let save_dir = PathBuf::from(out_dir).join("static");
        fs::create_dir_all(&save_dir).unwrap();

        let save_path = &save_dir.join("index.css");
        let mut file = fs::File::create(save_path).unwrap();
        file.write_all(CSS_TEMPLATE_FILE.as_bytes()).unwrap();

        let save_path = &save_dir.join("index.js");
        let mut file = fs::File::create(save_path).unwrap();
        file.write_all(JS_FILE.as_bytes()).unwrap();

        let save_path = &save_dir.join("favicon.ico");
        let mut file = fs::File::create(save_path).unwrap();
        file.write_all(super::templates::FAVICON_FILE).unwrap();

        let save_path = PathBuf::from(out_dir).join("index.html");
        let mut file = fs::File::create(save_path).unwrap();
        file.write_all(HTML_FILE.as_bytes()).unwrap();
    }

    fn process_md_file(&self, file: &PathBuf, out_dir: &str) {
        let raw_content = fs::read_to_string(file).unwrap();
        let parsed_content = MetaData::remove_meta_data(&raw_content);

        let html = self.to_html(&parsed_content);
        let save_path = self.get_save_path(file, &out_dir);
        let save_dir = save_path.parent().unwrap();
        fs::create_dir_all(save_dir).unwrap();
        let mut file = fs::File::create(save_path).unwrap();
        file.write_all(html.as_bytes()).unwrap();
    }

    fn to_html(&self, content: &String) -> String {
        let options = &Options {
            compile: CompileOptions {
                allow_dangerous_html: true,
                ..CompileOptions::default()
            },
            ..Options::gfm()
        };
        let html = markdown::to_html_with_options(&content, options);
        if html.is_err() {
            panic!("Error rendering markdown: {:?}", html.err());
        }
        html.unwrap()
    }

    fn get_save_path(&self, file: &PathBuf, out_dir: &str) -> PathBuf {
        let relative_path = file.strip_prefix(&self.root_section.path).unwrap();
        let save_path = PathBuf::from(out_dir).join(relative_path);
        let save_path = save_path.with_extension("html");
        save_path
    }
}
