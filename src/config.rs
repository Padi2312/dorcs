use std::path::Path;

pub struct Config {
    pub title: String,
    pub source: String,
    pub output: String,
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
            title: "Documentation".to_string(),
            source: "docs".to_string(),
            output: "output".to_string(),
        }
    }

    fn from_file(file_path: &str) -> Config {
        let content = std::fs::read_to_string(file_path).unwrap();
        let data: serde_json::Value = serde_json::from_str(&content).unwrap();
        let title = data["title"]
            .as_str()
            .unwrap_or("Documentation")
            .to_string();
        let source = data["source"].as_str().unwrap_or("docs").to_string();
        let output = data["output"].as_str().unwrap_or("docs_build").to_string();

        Config {
            title,
            source,
            output,
        }
    }
}
