use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use super::{dorcs_file::DorcsFile, navigation_tree::NavigationTree};

pub struct Builder {
    pub tree: NavigationTree,
    pub src_dir: String,
    pub out_dir: String,
}

impl Builder {
    pub fn new(src_dir: &String, out_dir: &String) -> Builder {
        Builder {
            tree: NavigationTree::new(&src_dir),
            src_dir: src_dir.to_string(),
            out_dir: out_dir.to_string(),
        }
    }

    pub fn process_file(&mut self, file: DorcsFile) {
        self.tree.insert(&file);

        // Prepare the output path
        let path = file.path.clone();
        let path = path.strip_prefix(&self.src_dir).unwrap();
        let path = path.with_extension("html");
        let out_path = Path::new(&self.out_dir).join(path);
        let content = file.to_html();
        fs::create_dir_all(&out_path.parent().unwrap()).unwrap();

        let mut file = match File::create(&out_path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to create file: {}", err);
                return;
            }
        };
        file.write_all(content.as_bytes()).unwrap();
    }

    pub fn generate_navigation(&mut self) {
        let root_node = self.tree.get_root_node();
        let children = root_node.borrow_mut().children.sort_by_key(|node| node.borrow().position);

        let save_path = Path::new(&self.out_dir).join("routes.json");
        let save_dir = save_path.parent().unwrap();
        fs::create_dir_all(save_dir).unwrap();
        let mut file = File::create(save_path).unwrap();
        // file.write_all(json!(children).to_string().as_bytes()).unwrap();
    }
}
