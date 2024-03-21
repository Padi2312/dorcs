use std::{fs, io::Write, path::PathBuf};

use handlebars::Handlebars;
use markdown::{CompileOptions, Options};
use regex::Regex;
use serde_json::json;

use super::{
    navigation::Navigation,
    templates::{CSS_TEMPLATE_FILE, NAVIGATION_TEMPLATE_FILE, TEMPLATE_FILE},
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

pub struct SectionHandler<'a> {
    pub root_section: Section,
    handlebars: Handlebars<'a>,
}

impl<'a> SectionHandler<'a> {
    pub fn new(root_section: Section) -> SectionHandler<'a> {
        let mut reg = Handlebars::new();
        reg.register_template_string("default", TEMPLATE_FILE)
            .unwrap();
        reg.register_template_string("navigation_template", NAVIGATION_TEMPLATE_FILE)
            .unwrap();
        SectionHandler {
            root_section,
            handlebars: reg,
        }
    }

    pub fn execute(&self, out_dir: &str) {
        println!("Root section: {:#?}", self.root_section);
        let navigation_list = generate_navigation(&self.root_section);
        if navigation_list.is_err() {
            panic!("No navigation list generated");
        }
        let navigation_list = navigation_list.unwrap();
        println!("Navigation list: {:#?}", navigation_list);
        self.process(&self.root_section, out_dir, &navigation_list);
        self.write_css_to_output(out_dir);
    }

    fn process(&self, section: &Section, out_dir: &str, navigation_list: &Vec<Navigation>) {
        for file in &section.files {
            // If file is .md file we process it
            // otherwise we assume it's an asset file
            if file.extension().unwrap() != "md" {
                self.copy_asset_file(file, out_dir);
            } else {
                self.process_md_file(file, out_dir, navigation_list);
            }
        }

        if let Some(sections) = &section.sections {
            for section in sections {
                self.process(section, out_dir, navigation_list);
            }
        }
    }

    fn copy_asset_file(&self, file: &PathBuf, out_dir: &str) {
        let relative_path = file.strip_prefix(&self.root_section.path).unwrap();
        let save_path = PathBuf::from(out_dir).join(relative_path);
        let save_dir = save_path.parent().unwrap();
        fs::create_dir_all(save_dir).unwrap();
        fs::copy(file, save_path).unwrap();
    }

    fn write_css_to_output(&self, out_dir: &str) {
        let save_path = PathBuf::from(out_dir).join("default.css");
        let mut file = fs::File::create(save_path).unwrap();
        file.write_all(CSS_TEMPLATE_FILE.as_bytes()).unwrap();
    }

    fn process_md_file(&self, file: &PathBuf, out_dir: &str, navigation_list: &Vec<Navigation>) {
        let raw_content = fs::read_to_string(file).unwrap();
        let parsed_content = self.remove_meta_data(&raw_content);

        // Process the content
        let html = self.to_html(&parsed_content);
        let data = json!({
            "content": html,
            "page_title": "Dorcs",
            "navigation": navigation_list,
        });

        let html = self.handlebars.render("default", &data);
        if html.is_err() {
            panic!("Error rendering template: {:?}", html.err());
        }
        let html = html.unwrap();

        let save_path = self.get_save_path(file, &out_dir);
        let save_dir = save_path.parent().unwrap();
        fs::create_dir_all(save_dir).unwrap();
        let mut file = fs::File::create(save_path).unwrap();
        file.write_all(html.as_bytes()).unwrap();
    }

    fn remove_meta_data(&self, content: &String) -> String {
        let re = Regex::new(r"(?s)^---\s*(.*?)\s*---").unwrap();
        let content = re.replace(content, "");
        content.to_string()
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
