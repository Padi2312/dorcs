use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct ConfigJsonSchema {
    pub page_title: Option<String>,
    pub source: Option<String>,
    pub output: Option<String>,
    pub page_settings: Option<PageSettingsJsonSchema>,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct PageSettingsJsonSchema {
    pub icon: Option<String>,
    pub landing_page: Option<String>,
}
