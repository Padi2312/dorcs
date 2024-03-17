use std::path::Path;

pub struct Config {
    pub title: String,
    pub input_dir: String,
    pub output_dir: String,
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
            input_dir: "docs".to_string(),
            output_dir: "docs_build".to_string(),
        }
    }

    fn from_file(file_path: &str) -> Config {
        let content = std::fs::read_to_string(file_path).unwrap();
        let data: serde_json::Value = serde_json::from_str(&content).unwrap();
        let title = data["title"]
            .as_str()
            .unwrap_or("Documentation")
            .to_string();
        let input_dir = data["input_dir"].as_str().unwrap_or("docs").to_string();
        let output_dir = data["output_dir"]
            .as_str()
            .unwrap_or("docs_build")
            .to_string();

        Config {
            title: title,
            input_dir: input_dir,
            output_dir: output_dir,
        }
    }
}
