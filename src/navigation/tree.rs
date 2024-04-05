use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use crate::core::dorcs_file::DorcsFile;

use super::node::NavigationNode;

pub struct NavigationTree {
    pub root_dir: String,
    root_node: Rc<RefCell<NavigationNode>>,
}

impl NavigationTree {
    pub fn new(root_dir: &str) -> NavigationTree {
        let root_node = NavigationNode::new(&root_dir, &root_dir, &root_dir, 0);
        NavigationTree {
            root_dir: root_dir.to_string(),
            root_node,
        }
    }

    pub fn get_root_node(&self) -> Rc<RefCell<NavigationNode>> {
        self.root_node.clone()
    }

    pub fn insert(&mut self, file: &DorcsFile) {
        if self.check_for_section_index_file(&file.path) {
            self.insert_section_index_file(file);
            return;
        }

        let components: Vec<&Path> = file
            .path
            .components()
            .map(|c| Path::new(c.as_os_str()))
            .collect();
        let mut current_node = Rc::clone(&self.root_node);
        for component in components {
            let component_str = component.to_str().unwrap_or_default();

            if component_str == &self.root_dir || current_node.borrow().path == component_str {
                continue;
            }

            let maybe_child = current_node.borrow().find_child(component_str);
            if maybe_child.is_some() {
                current_node = maybe_child.unwrap();
            } else {
                let file_path = &file.path.clone();
                let file_path = file_path.strip_prefix(&self.root_dir);
                let url_path = match file_path {
                    Ok(path) => path.to_str().unwrap_or(""),
                    Err(_) => "",
                };
                let new_node = NavigationNode::new(
                    component_str,
                    url_path,
                    file.meta_data.title.as_str(),
                    file.meta_data.position.unwrap_or(isize::MAX),
                );
                current_node.borrow_mut().add_child(Rc::clone(&new_node));
                current_node = new_node;
            }
        }
        self.sort_tree();
    }

    fn insert_section_index_file(&mut self, file: &DorcsFile) {
        let parent_components: Vec<&Path> = file
            .path
            .components()
            .map(|c| Path::new(c.as_os_str()))
            .collect();

        let first_parent_folder = parent_components[parent_components.len() - 2];
        let mut current_node = Rc::clone(&self.root_node);
        for component in parent_components {
            let component_str = component.to_str().unwrap_or_default();
            if component_str == &self.root_dir {
                continue;
            }

            if current_node.borrow().path == first_parent_folder.to_str().unwrap()
                && component_str == "index.md"
            {
                current_node.borrow_mut().title = file.meta_data.title.clone();
                current_node.borrow_mut().position = file.meta_data.position.unwrap_or(9001);
                let file_path = &file.path.clone();
                let file_path = file_path.strip_prefix(&self.root_dir);
                let url_path = match file_path {
                    Ok(path) => path.to_str().unwrap_or(""),
                    Err(_) => "",
                };
                if !file.content.is_empty() {
                    current_node.borrow_mut().set_url(url_path);
                } else {
                    current_node.borrow_mut().set_url("");
                }
                return;
            }

            let maybe_child = current_node.borrow().find_child(component_str);
            if maybe_child.is_some() {
                current_node = maybe_child.unwrap();
            }
        }
    }

    fn check_for_section_index_file(&self, path: &Path) -> bool {
        let components = path.components().collect::<Vec<_>>();
        if components.len() <= 2 {
            return false;
        }
        let file_name = components.last().unwrap().as_os_str().to_str().unwrap();

        file_name == "index.md"
    }

    fn sort_tree(&mut self) {
        // Recursive function to sort the tree and its children by position
        fn sort_node(node: &Rc<RefCell<NavigationNode>>) {
            let mut node = node.borrow_mut();
            node.children.sort_by_key(|child| child.borrow().position);
            for child in &node.children {
                sort_node(child);
            }
        }

        sort_node(&self.root_node);
    }
}
