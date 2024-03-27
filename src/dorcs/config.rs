use std::path::Path;

use super::page_settings::PageSettings;
use dorcs_json_schema::ConfigJsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub page_title: String,
    pub source: String,
    pub output: String,
    pub page_settings: PageSettings,
}

impl Config {
    pub fn load() -> Config {
        let default_config_path = "dorcs.config.json";
        let mut config: Config = Config::default();
        if Path::new(default_config_path).exists() {
            config = Config::from_file(default_config_path);
        }
        config
    }

    fn default() -> Config {
        Config {
            page_title: "Documentation".to_string(),
            source: "docs".to_string(),
            output: "output".to_string(),
            page_settings: PageSettings::load(),
        }
    }

    fn from_file(file_path: &str) -> Config {
        let content = std::fs::read_to_string(file_path).unwrap();
        let config_schema: ConfigJsonSchema = serde_json::from_str(&content).unwrap();
        let page_title = config_schema
            .page_title
            .unwrap_or("Documentation".to_string());
        let source = config_schema.source.unwrap_or("docs".to_string());
        let output = config_schema.output.unwrap_or("output".to_string());

        Config {
            page_title,
            source,
            output,
            page_settings: PageSettings::load(),
        }
    }
}
