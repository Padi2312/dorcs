use dorcs_json_schema::ServerSettingsJsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ServerSettings {
    pub enabled: bool,
    pub port: u16,
    pub auto_reload: bool,
}

impl Default for ServerSettings {
    fn default() -> Self {
        ServerSettings {
            enabled: false,
            port: 8080,
            auto_reload: false,
        }
    }
}

impl ServerSettings {
    pub fn load(json: Option<ServerSettingsJsonSchema>) -> ServerSettings {
        let server_settings: ServerSettings = ServerSettings::default();
        if json.is_none() {
            return server_settings;
        }
        let data: ServerSettingsJsonSchema = json.unwrap();
        let enabled = data.enabled.unwrap_or(false);
        let port = data.port.unwrap_or(8080);
        let auto_reload = data.auto_reload.unwrap_or(false);

        ServerSettings {
            enabled,
            port,
            auto_reload,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
