use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use serde_json::{json, Value};

use super::{
    config::Config,
    loader::Loader,
    templ_processor::{TemplProcessor, CSS_TEMPLATE_FILE},
};

pub struct Generator {
    pub config: Config,
}

impl Generator {
    pub fn new(config: Config) -> Generator {
        Generator { config }
    }

    pub fn generate_docs(&self) {
        let handler = TemplProcessor::new();
        let mut document_loader = Loader::new(self.config.source.to_string());
        document_loader.load();

        let links = document_loader.get_links();
        let links: Vec<Value> = links.iter().map(|link| link.to_json()).collect();

        let documents = document_loader.documents;
        for doc in documents {
            let html = doc.to_html();
            let processed_html = handler.process_json(json!({
                "content": html ,
                "title": doc.meta_data.title,
                "page_title":self.config.page_title,
                "links": links.clone(),
            }));

            let file_path = self.change_file_extension(&PathBuf::from(doc.path), "html");
            let save_path = self.get_save_path(&file_path, &self.config.output);
            self.create_parent_dirs(&save_path);

            let mut file = fs::File::create(save_path).unwrap();
            file.write_all(processed_html.as_bytes()).unwrap();
        }

        self.copy_asset_files(document_loader.assets);
        self.write_css_file();
    }

    fn copy_asset_files(&self, static_files: Vec<PathBuf>) {
        for file in static_files {
            let save_path = self.get_save_path(&file, &self.config.output);
            self.create_parent_dirs(&save_path);
            fs::copy(&file, save_path).unwrap();
        }
    }

    fn write_css_file(&self) {
        let css = CSS_TEMPLATE_FILE;
        let save_path = Path::new(&self.config.output).join("default.css");
        let mut file = fs::File::create(save_path).unwrap();
        file.write_all(css.as_bytes()).unwrap();
    }

    fn get_save_path(&self, file: &PathBuf, output_dir: &str) -> PathBuf {
        let file_path = file.strip_prefix(&self.config.source).unwrap();
        let file_path = PathBuf::from(file_path);
        let save_path = Path::new(output_dir).join(file_path);
        save_path
    }

    fn change_file_extension(&self, file: &PathBuf, new_extension: &str) -> PathBuf {
        let file_path = file.to_string_lossy();
        let file_path = file_path.split('.').next().unwrap();
        let file_path = format!("{}.{}", file_path, new_extension);
        PathBuf::from(file_path)
    }

    fn create_parent_dirs(&self, file: &PathBuf) {
        if let Some(parent) = file.parent() {
            fs::create_dir_all(parent).unwrap();
        }
    }
}
