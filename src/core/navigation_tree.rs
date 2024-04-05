use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use serde::{Deserialize, Serialize, Serializer};

use super::dorcs_file::DorcsFile;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerializableNavigationNode {
    pub path: String,
    pub title: String,
    pub position: isize,
    pub children: Vec<SerializableNavigationNode>,
    pub url: String,
}

impl SerializableNavigationNode {
    pub fn from_navigation_node(node: &NavigationNode) -> Self {
        SerializableNavigationNode {
            path: node.path.clone(),
            title: node.title.clone(),
            position: node.position,
            children: node
                .children
                .iter()
                .map(|child| SerializableNavigationNode::from_navigation_node(&child.borrow()))
                .collect(),
            url: node.url.clone(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct NavigationNode {
    pub path: String,
    pub title: String,
    pub position: isize,
    pub children: Vec<Rc<RefCell<NavigationNode>>>,
    pub url: String,
}

impl Serialize for NavigationNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("NavigationNode", 5)?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("position", &self.position)?;
        state.serialize_field("url", &self.url)?;
        // Manually handle children serialization to avoid issues with Rc<RefCell<T>>
        let children: Vec<_> = self
            .children
            .iter()
            .map(|child| child.borrow().clone())
            .collect();
        state.serialize_field("children", &children)?;

        state.end()
    }
}

impl NavigationNode {
    fn new(path: &str, url: &str, title: &str, position: isize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(NavigationNode {
            path: path.to_string(),
            title: title.to_string(),
            children: Vec::new(),
            position: position,
            // `prepare_url` already adds the leading slash
            url: NavigationNode::prepare_url(url),
        }))
    }

    fn set_url(&mut self, url: &str) {
        if url.is_empty() {
            self.url = "".to_string();
            return;
        }
        self.url = NavigationNode::prepare_url(url);
    }

    fn prepare_url(path: &str) -> String {
        let url = format!("pages/{}", path);
        let url = url.replace(".md", ".html");
        let url = url.replace("\\", "/");
        url
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
