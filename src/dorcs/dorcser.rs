use std::{path::PathBuf, time::Instant};

use crate::dorcs::sections::{SectionBuilder, SectionGenerator};

use super::config::Config;

pub struct Dorcser {
    pub config: Config,
}

impl Dorcser {
    pub fn new() -> Dorcser {
        Dorcser {
            config: Config::load(),
        }
    }

    pub fn build(&self) {
        let source_dir = &self.config.source;
        let output_dir = &self.config.output;

        let start_time = Instant::now();

        let section_constructor = SectionGenerator::new(PathBuf::from(source_dir));
        let section = section_constructor.construct();

        let section_handler = SectionBuilder::new(section);
        section_handler.execute(output_dir);

        self.write_page_settings();

        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;

        println!("✅ Build complete in {:.2?}!", elapsed_time);
        println!("📄 Your documentation is available in: {}", output_dir);
    }

    fn write_page_settings(&self) {
        let page_settings = self.config.page_settings.to_json();
        let page_settings_path = format!("{}/page_settings.json", self.config.output);
        std::fs::write(page_settings_path, page_settings).unwrap();
    }
}
