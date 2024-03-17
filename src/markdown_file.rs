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
        let html = markdown::to_html(&self.content);
        html
    }
}
