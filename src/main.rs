mod core;

use core::dorcs::Dorcs;

fn main() {
    Dorcs::new().init();
    // let root_dir = "docs".to_string();
    // let mut builder = Builder::new(&root_dir, "output");

    // let file_handler = FileHandler::new("docs");
    // let markdown_files = file_handler.get_markdown_files();
    // let asset_files = file_handler.get_asset_files();
    // for file in markdown_files {
    //     let file = core::dorcs_file::DorcsFile::new(file);
    //     builder.process_file(file);
    // }

    // for file in asset_files {
    //     builder.process_asset_file(&file);
    // }

    // builder.generate_navigation();
}
