use std::sync::{Arc, RwLock, Weak};

pub trait Key: PartialEq + PartialOrd + Clone {}

pub trait Record: Clone {}

pub type NodePtr<K, T, const FANOUT: usize> = Arc<RwLock<Node<K, T, FANOUT>>>;

pub type NodeWeakPtr<K, T, const FANOUT: usize> = Weak<RwLock<Node<K, T, FANOUT>>>;

pub type RecordPtr<T> = Arc<RwLock<T>>;

#[derive(Debug)]
pub struct Leaf<K: Key, V: Record, const FANOUT: usize> {
    pub size: usize,
    pub keys: Vec<Option<K>>,
    pub records: Vec<Option<RecordPtr<V>>>,
    pub parent: Option<NodeWeakPtr<K, V, FANOUT>>,
    pub prev: Option<NodeWeakPtr<K, V, FANOUT>>,
    pub next: Option<NodeWeakPtr<K, V, FANOUT>>,
}

#[derive(Debug)]
pub struct Interior<K: Key, V: Record, const FANOUT: usize> {
    pub size: usize,
    pub keys: Vec<Option<K>>,
    pub children: Vec<Option<NodePtr<K, V, FANOUT>>>,
}

#[derive(Debug, Default)]
pub enum Node<K: Key, V: Record, const FANOUT: usize> {
    #[default]
    Invalid,
    Leaf(Leaf<K, V, FANOUT>),
    Interior(Interior<K, V, FANOUT>),
}

impl<K: Key, V: Record, const FANOUT: usize> Node<K, V, FANOUT> {
    pub fn new_leaf() -> Leaf<K, V, FANOUT> {
        Leaf {
            size: 0,
            keys: vec![None; FANOUT],
            records: vec![None; FANOUT + 1],
            parent: None,
            prev: None,
            next: None,
        }
    }
    pub fn new_interior() -> Interior<K, V, FANOUT> {
        Interior {
            size: 0,
            keys: vec![None; FANOUT],
            children: vec![None; FANOUT + 1],
        }
    }

    pub(super) fn leaf(&self) -> Option<&Leaf<K, V, FANOUT>> {
        if let Node::Invalid = self {
            panic!("Invalid Node encountered while accessing leaf!")
        }

        if let Node::Leaf(leaf) = self {
            Some(leaf)
        } else {
            None
        }
    }

    pub(super) fn leaf_mut(&mut self) -> Option<&mut Leaf<K, V, FANOUT>> {
        if let Node::Invalid = self {
            panic!("Invalid Node encountered while accessing leaf!")
        }

        if let Node::Leaf(leaf) = self {
            Some(leaf)
        } else {
            None
        }
    }

    pub(super) fn unwrap_leaf(&self) -> &Leaf<K, V, FANOUT> {
        self.leaf().unwrap()
    }

    pub(super) fn unwrap_leaf_mut(&mut self) -> &mut Leaf<K, V, FANOUT> {
        self.leaf_mut().unwrap()
    }

    pub(super) fn interior(&self) -> Option<&Interior<K, V, FANOUT>> {
        if let Node::Invalid = self {
            panic!("Invalid Node encountered while accessing interior!")
        }

        if let Node::Interior(interior) = self {
            Some(interior)
        } else {
            None
        }
    }

    pub(super) fn unwrap_interior(&self) -> &Interior<K, V, FANOUT> {
        self.interior().unwrap()
    }

    pub(super) fn interior_mut(&mut self) -> Option<&mut Interior<K, V, FANOUT>> {
        if let Node::Invalid = self {
            panic!("Invalid Node encountered while accessing interior!")
        }

        if let Node::Interior(interior) = self {
            Some(interior)
        } else {
            None
        }
    }

    pub(super) fn unwrap_interior_mut(&mut self) -> &mut Interior<K, V, FANOUT> {
        self.interior_mut().unwrap()
    }
}