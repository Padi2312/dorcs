use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize, Serializer};

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
    pub fn new(path: &str, url: &str, title: &str, position: isize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(NavigationNode {
            path: path.to_string(),
            title: title.to_string(),
            children: Vec::new(),
            position: position,
            // `prepare_url` already adds the leading slash
            url: NavigationNode::prepare_url(url),
        }))
    }

    pub fn set_url(&mut self, url: &str) {
        if url.is_empty() {
            self.url = "".to_string();
            return;
        }
        self.url = NavigationNode::prepare_url(url);
    }

    fn prepare_url(path: &str) -> String {
        let url = format!("pages/{}", path);
        let url = url.replace(".md", "");
        let url = url.replace("\\", "/");
        url
    }

    pub fn add_child(&mut self, child: Rc<RefCell<NavigationNode>>) {
        self.children.push(child);
    }

    pub fn find_child(&self, path: &str) -> Option<Rc<RefCell<NavigationNode>>> {
        self.children
            .iter()
            .find(|child| child.borrow().path == path)
            .cloned()
    }
}
