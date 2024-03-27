use std::path::Path;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, JsonSchema)]
pub struct PageSettingsJsonSchema {
    pub icon: Option<String>,
    pub landing_page: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PageSettings {
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
        let icon = data.icon.unwrap_or("".to_string());
        let landing_page = data.landing_page.unwrap_or("index.md".to_string());

        PageSettings { icon, landing_page }
    }

    fn default() -> PageSettings {
        PageSettings {
            icon: "".to_string(),
            landing_page: "index.md".to_string(),
        }
    }
}
