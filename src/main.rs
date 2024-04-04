mod core;
use core::builder::Builder;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use walkdir::WalkDir;

use crate::core::navigation_tree::NavigationTree;

// #[derive(Clone, Debug)]
// struct NavigationNode {
//     path: String,
//     title: String,
//     children: Vec<Rc<RefCell<NavigationNode>>>,
// }

// impl NavigationNode {
//     fn new(path: &str, title: &str) -> Rc<RefCell<Self>> {
//         Rc::new(RefCell::new(NavigationNode {
//             path: path.to_string(),
//             title: title.to_string(),
//             children: Vec::new(),
//         }))
//     }

//     fn add_child(&mut self, child: Rc<RefCell<NavigationNode>>) {
//         self.children.push(child);
//     }

//     fn find_child(&self, path: &str) -> Option<Rc<RefCell<NavigationNode>>> {
//         self.children
//             .iter()
//             .find(|child| child.borrow().path == path)
//             .cloned()
//     }
// }

// fn build_tree(
//     root: Rc<RefCell<NavigationNode>>,
//     components: Vec<Vec<&Path>>,
// ) -> Rc<RefCell<NavigationNode>> {
//     for path in components {
//         let mut current_node = Rc::clone(&root);
//         for component in path {
//             let component_str = component.to_str().unwrap_or_default();

//             if component_str == "docs" || current_node.borrow().path == component_str {
//                 continue;
//             }

//             let maybe_child = current_node.borrow().find_child(component_str);
//             match maybe_child {
//                 Some(child) => {
//                     current_node = child;
//                 }
//                 None => {
//                     let new_node = NavigationNode::new(component_str, component_str);
//                     current_node.borrow_mut().add_child(Rc::clone(&new_node));
//                     current_node = new_node;
//                 }
//             }
//         }
//     }
//     root
// }

fn main() {
    // let root_node = NavigationNode::new("/", "Root");

    // // Example path components
    // let components = vec![
    //     vec![Path::new("src"), Path::new("lib.rs")],
    //     vec![Path::new("src"), Path::new("main.rs")],
    //     vec![Path::new("tests"), Path::new("lib.rs")],
    //     vec![
    //         Path::new("src"),
    //         Path::new("nested"),
    //         Path::new("deep"),
    //         Path::new("file 1.rs"),
    //     ],
    // ];

    // let root = build_tree(root_node, components);
    // println!("{:#?}", root);

    // let components = vec![
    //     vec![Path::new("src"), Path::new("newwwwww_lib.rs")],
    //     vec![
    //         Path::new("src"),
    //         Path::new("tests"),
    //         Path::new("lib.rs"),
    //     ],
    // ];

    // let root = build_tree(root, components);
    // println!("{:#?}", root);
     let all_paths = WalkDir::new("docs")
         .into_iter()
         .filter_map(|e| e.ok())
         .map(|e| e.path().to_path_buf())
         .collect::<Vec<PathBuf>>();

     let markdown_files = all_paths
         .iter()
         .filter(|p| p.is_file() && p.extension().unwrap() == "md")
         .map(|p| p.to_path_buf())
         .collect::<Vec<PathBuf>>();

     let asset_files = all_paths
         .iter()
         .filter(|p| p.is_file() && p.extension().unwrap() != "md")
         .map(|p| p.to_path_buf())
         .collect::<Vec<PathBuf>>();
    let root_dir = "docs".to_string();
    let mut builder = Builder::new(&root_dir, &"output".to_string());

    for file in markdown_files {
        let file = core::dorcs_file::DorcsFile::new(file);
        builder.process_file(file);
    }

    // println!("{:#?}", &builder.tree.get_root_node());
    builder.generate_navigation();
    // Get paths in root dir

}
