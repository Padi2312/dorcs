mod generator;
mod templ_processor;
mod markdown_file;
mod config;
mod loader;

fn main() {
    let generator = generator::Generator::new("docs".to_string(), "docs_gen".to_string());
    generator.generate_docs();
}
