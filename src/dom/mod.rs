use std::cell::RefCell;
use std::rc::{Rc, Weak};

use NodeData::*;

#[derive(Debug)]
pub struct Node {
    pub(crate) data: NodeData,
    pub(crate) document: Option<Weak<RefCell<Self>>>,
    pub(crate) parent: Option<Weak<RefCell<Self>>>,
    pub(crate) children: Vec<Rc<RefCell<Self>>>,
}

#[derive(PartialEq, Debug)]
pub enum NodeData {
    Document,
    Doctype {
        name: String,
        public_identifier: String,
        private_identifier: String,
    },
    Element {
        tag_name: String,
    },
    Text(String),
    Comment(String),
}

impl Node {
    pub fn create_document() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            data: Document,
            document: None,
            parent: None,
            children: Vec::new(),
        }))
    }

    pub fn create_element(
        tag_name: String,
        document: &Rc<RefCell<Self>>,
        parent: &Rc<RefCell<Self>>,
    ) -> Rc<RefCell<Self>> {
        let elem = Rc::new(RefCell::new(Self {
            data: Element { tag_name },
            document: Some(Rc::downgrade(document)),
            parent: Some(Rc::downgrade(parent)),
            children: Vec::new(),
        }));
        RefCell::borrow_mut(parent).children.push(elem.clone());
        elem
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        (&self.data, &self.children) == (&other.data, &other.children)
    }
}
