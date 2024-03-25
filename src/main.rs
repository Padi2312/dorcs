mod dorcs;

use std::path::PathBuf;

use crate::dorcs::config::Config;
use dorcs::sections::SectionBuilder;
use dorcs::sections::SectionGenerator;
use std::time::Instant;

fn main() {
    let config: Config = Config::load();
    let source_dir = config.source;
    let output_dir = config.output;

    let start_time = Instant::now();

    let section_constructor = SectionGenerator::new(PathBuf::from(&source_dir));
    let section = section_constructor.construct();

    let section_handler = SectionBuilder::new(section);
    section_handler.execute(&output_dir);

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    println!("✅ Build complete in {:.2?}!", elapsed_time);
    println!("📄 Your documentation is available in: {}", output_dir);
}
