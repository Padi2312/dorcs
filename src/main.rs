mod config;
mod generator;
mod loader;
mod markdown_file;
mod templ_processor;

use config::Config;
use generator::Generator;

fn main() {
    let config: Config = Config::load();
    let generator = Generator::new(config);
    generator.generate_docs();
}
