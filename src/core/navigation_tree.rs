use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use super::dorcs_file::DorcsFile;

#[derive(Clone, Debug)]
pub struct NavigationNode {
    pub path: String,
    pub title: String,
    pub position: i32,
    pub children: Vec<Rc<RefCell<NavigationNode>>>,
}

impl NavigationNode {
    fn new(path: &str, title: &str, position: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(NavigationNode {
            path: path.to_string(),
            title: title.to_string(),
            children: Vec::new(),
            position: position,
        }))
    }

    fn add_child(&mut self, child: Rc<RefCell<NavigationNode>>) {
        self.children.push(child);
    }

    fn find_child(&self, path: &str) -> Option<Rc<RefCell<NavigationNode>>> {
        self.children
            .iter()
            .find(|child| child.borrow().path == path)
            .cloned()
    }
}
pub struct NavigationTree {
    pub root_dir: String,
    root_node: Rc<RefCell<NavigationNode>>,
}

impl NavigationTree {
    pub fn new(root_dir: &String) -> NavigationTree {
        let root_node = NavigationNode::new(&root_dir.as_str(), &root_dir.as_str(), 0);
        NavigationTree {
            root_dir: root_dir.to_string(),
            root_node,
        }
    }

    pub fn get_root_node(&self) -> Rc<RefCell<NavigationNode>> {
        self.root_node.clone()
    }

    pub fn insert(&mut self, file: &DorcsFile) {
        let components: Vec<&Path> = file
            .path
            .components()
            .map(|c| Path::new(c.as_os_str()))
            .collect();
        // for path in components {
        let mut current_node = Rc::clone(&self.root_node);
        for component in components {
            let component_str = component.to_str().unwrap_or_default();

            if component_str == &self.root_dir || current_node.borrow().path == component_str {
                continue;
            }

            let maybe_child = current_node.borrow().find_child(component_str);
            match maybe_child {
                Some(child) => {
                    current_node = child;
                }
                None => {
                    let new_node = NavigationNode::new(
                        component_str,
                        component_str,
                        file.meta_data.position.unwrap_or(i32::MAX),
                    );
                    current_node.borrow_mut().add_child(Rc::clone(&new_node));
                    current_node = new_node;
                }
            }
        }
        // }
        self.sort_tree();
    }

    pub fn sort_tree(&mut self) {
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
