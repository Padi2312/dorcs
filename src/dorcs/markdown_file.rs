use markdown::{CompileOptions, Options, ParseOptions};

#[derive(Debug, Clone)]
pub struct MarkdownFile {
    pub meta_data: MetaData,
    pub content: String,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct MetaData {
    pub title: String,
    pub position: Option<i32>,
}

impl MarkdownFile {
    pub fn to_html(&self) -> String {
        let options = &Options {
            compile: CompileOptions {
                allow_dangerous_html: true,
                ..CompileOptions::default()
            },
            ..Options::gfm()
        };
        let html = markdown::to_html_with_options(&self.content, options);
        if html.is_err() {
            panic!("Error rendering markdown: {:?}", html.err());
        }
        html.unwrap()
    }
}
