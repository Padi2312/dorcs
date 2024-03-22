use regex::Regex;

#[derive(Debug, Clone)]
pub struct MetaData {
    pub title: String,
    pub position: Option<i32>,
}

impl MetaData {
    pub fn from_string(content: &String) -> MetaData {
        Self::parse_metadata(content)
    }

    // Parses the metadata from a markdown file.
    fn parse_metadata(raw_content: &String) -> MetaData {
        let re = Regex::new(r"(?s)^---\s*(.*?)\s*---").unwrap();
        let mut title = String::new();
        let mut position: Option<i32> = None;

        if let Some(cap) = re.captures_iter(raw_content.as_str()).next() {
            let meta_str = &cap[1];

            for line in meta_str.lines() {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();

                    // Match the key and update the struct fields accordingly
                    match key {
                        "title" => title = value.to_string(),
                        "position" => position = value.parse::<i32>().ok(),
                        _ => {} // Ignore unknown keys
                    }
                }
            }
        }

        MetaData { title, position }
    }

    pub fn remove_meta_data(content: &String) -> String {
        let re = Regex::new(r"(?s)^---\s*(.*?)\s*---").unwrap();
        let content = re.replace(content, "");
        content.to_string()
    }
}
