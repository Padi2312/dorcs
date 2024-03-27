use std::path::Path;

use dorcs_json_schema::PageSettingsJsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PageSettings {
    pub page_title: String,
    pub icon: String,
    pub landing_page: String,
}

impl PageSettings {
    pub fn load() -> PageSettings {
        let default_config_path = "dorcs.config.json";
        let mut page_settings: PageSettings = PageSettings::default();
        if Path::new(default_config_path).exists() {
            page_settings = PageSettings::from_file(default_config_path);
        }
        page_settings
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn from_file(file_path: &str) -> PageSettings {
        let content = std::fs::read_to_string(file_path).unwrap();
        let data: PageSettingsJsonSchema = serde_json::from_str(&content).unwrap();
        let page_title = data.page_title.unwrap_or("Documentation".to_string());
        let icon = data.icon.unwrap_or("".to_string());
        let landing_page = data.landing_page.unwrap_or("index.md".to_string());

        PageSettings {
            page_title,
            icon,
            landing_page,
        }
    }

    fn default() -> PageSettings {
        PageSettings {
            page_title: "Documentation".to_string(),
            icon: "".to_string(),
            landing_page: "index.md".to_string(),
        }
    }
}
