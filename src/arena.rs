use crate::Node;

#[derive(Debug, Clone)]
pub struct Arena<T, K> {
    root: Option<usize>,
    nodes: Vec<Node<T, K>>,
}

impl<T, K> Arena<T, K>
where
    K: PartialEq,
{
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            root: None,
        }
    }

    pub fn insert(&mut self, data: T, key: K) -> usize {
        let next_index = self.nodes.len();

        self.nodes.push(Node::new(data, key));

        next_index
    }

    pub fn root(&self) -> Option<usize> {
        self.root
    }

    pub fn set_root(&mut self, new_root: Option<usize>) {
        self.root = new_root;
    }

    pub fn nodes(&self) -> &Vec<Node<T, K>> {
        &self.nodes
    }

    pub fn find_inner(&self, key: K) -> Option<&T> {
        for node in self.nodes.iter() {
            if node.key == key {
                return Some(&node.data);
            }
        }

        None
    }

    pub fn get_inner(&self, index: usize) -> Option<&T> {
        let node = self.nodes.get(index);

        if let Some(node) = node {
            Some(&node.data)
        } else {
            None
        }
    }

    pub fn get_inner_mut(&mut self, index: usize) -> Option<&mut T> {
        let node = self.nodes.get_mut(index);

        if let Some(node) = node {
            Some(&mut node.data)
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&Node<T, K>> {
        self.nodes.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Node<T, K>> {
        self.nodes.get_mut(index)
    }

    pub fn find(&self, key: K) -> Option<&Node<T, K>> {
        for node in self.nodes.iter() {
            if node.key == key {
                return Some(node);
            }
        }

        None
    }

    pub fn get_parent(&self, index: usize) -> Option<&Node<T, K>> {
        let node = self.get(index);

        if let Some(node) = node {
            match node.parent() {
                Some(index) => self.get(index),
                None => None,
            }
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.len() == 0
    }

    pub fn count(&self) -> usize {
        self.nodes.len()
    }
}
