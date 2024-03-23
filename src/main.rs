mod dorcs;

use std::path::PathBuf;

use crate::dorcs::config::Config;
use dorcs::sections::SectionGenerator;
use dorcs::sections::SectionBuilder;

fn main() {
    let config: Config = Config::load();
    let source_dir = config.source;
    let output_dir = config.output;

    let section_constructor = SectionGenerator::new(PathBuf::from(&source_dir));
    let section = section_constructor.construct();
    println!("Section: {:#?}", section);

    let section_handler = SectionBuilder::new(section);
    section_handler.execute(&output_dir);
}
