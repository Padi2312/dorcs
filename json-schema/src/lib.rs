use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct ConfigJsonSchema {
    pub source: Option<String>,
    pub output: Option<String>,
    pub server: Option<ServerSettingsJsonSchema>,
    pub page_settings: Option<PageSettingsJsonSchema>,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct PageSettingsJsonSchema {
    pub page_title: Option<String>,
    pub icon: Option<String>,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct ServerSettingsJsonSchema {
    pub enabled: Option<bool>,
    pub port: Option<u16>,
    pub auto_reload: Option<bool>,
}
