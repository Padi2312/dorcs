use std::path::PathBuf;

use handlebars::Handlebars;
use serde_json::Value as JsonValue;

static TEMPLATE_FILE: &'static str = include_str!("templates/default.html");

pub struct TemplProcessor<'a> {
    pub files_list: Vec<PathBuf>,
    reg: Handlebars<'a>,
}

impl<'a> TemplProcessor<'a> {
    pub fn new() -> TemplProcessor<'a> {
        let mut reg = Handlebars::new();
        reg.register_template_string("default", TEMPLATE_FILE)
            .unwrap();
        TemplProcessor {
            files_list: Vec::new(),
            reg,
        }
    }

    pub fn process_templ(&self, title: String, content: String, links: Vec<JsonValue>) -> String {
        let data = serde_json::json!({
            "title":title,
            "content": content,
            "links": links
        });
        let result = self.reg.render("default", &data);
        if result.is_err() {
            panic!("Error rendering template: {:?}", result.err());
        }
        let result = result.unwrap();
        result
    }
}
