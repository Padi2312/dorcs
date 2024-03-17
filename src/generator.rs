use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use crate::{config::Config, loader, templ_processor};

pub struct Generator {
    pub config: Config,
}

impl Generator {
    pub fn new(config: Config) -> Generator {
        Generator { config }
    }

    pub fn generate_docs(&self) {
        let handler = templ_processor::TemplProcessor::new();
        let mut document_loader = loader::Loader::new(self.config.source.to_string());
        document_loader.load();

        let links = document_loader.get_links();

        let documents = document_loader.documents;
        for doc in documents {
            let html = doc.to_html();
            let processed_html =
                handler.process_templ(self.config.title.to_string(), html, links.clone());

            let file_path = self.change_file_extension(&PathBuf::from(doc.path), "html");
            let save_path = self.get_save_path(&file_path, &self.config.output);
            self.create_parent_dirs(&save_path);

            let mut file = fs::File::create(save_path).unwrap();
            file.write_all(processed_html.as_bytes()).unwrap();
        }
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
