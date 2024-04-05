use dorcs_json_schema::PageSettingsJsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PageSettings {
    pub page_title: String,
    pub icon: String,
}

impl PageSettings {
    pub fn load(json: Option<PageSettingsJsonSchema>) -> PageSettings {
        let page_settings: PageSettings = PageSettings::default();
        if json.is_none() {
            return page_settings;
        }
        let data: PageSettingsJsonSchema = json.unwrap();
        let page_title = data.page_title.unwrap_or("Documentation".to_string());
        let icon = if let Some(icon) = data.icon {
            // Check if path is relative or absolute due to structure of the output
            // we need to make sure the path is absolute to render the icon correctly
            // if icon.starts_with("./") {
            icon.trim_start_matches(".").to_string()
        } else {
            "".to_string()
        };

        PageSettings { page_title, icon }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn default() -> PageSettings {
        PageSettings {
            page_title: "Documentation".to_string(),
            icon: "".to_string(),
        }
    }
}
