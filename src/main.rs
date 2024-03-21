mod dorcs;

use dorcs::sections;

fn main() {
    let section_constructor = sections::SectionGenerator::new("docs".into());
    let section = section_constructor.construct();
    println!("Section: {:#?}", section);

    let section_handler = sections::SectionHandler::new(section);
    section_handler.execute("output");
}
