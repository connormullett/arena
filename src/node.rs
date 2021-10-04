#[derive(Debug, Clone)]
pub struct Node<T, K> {
    parent: Option<usize>,
    previous_sibling: Option<usize>,
    next_sibling: Option<usize>,
    first_child: Option<usize>,
    last_child: Option<usize>,

    pub key: K,
    pub data: T,
}

impl<T, K> Node<T, K> {
    pub fn new(data: T, key: K) -> Self {
        Node {
            parent: None,
            previous_sibling: None,
            next_sibling: None,
            first_child: None,
            last_child: None,
            key,
            data,
        }
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn parent(&self) -> Option<usize> {
        self.parent
    }

    pub fn previous_sibling(&self) -> Option<usize> {
        self.previous_sibling
    }

    pub fn next_sibling(&self) -> Option<usize> {
        self.next_sibling
    }

    pub fn first_child(&self) -> Option<usize> {
        self.first_child
    }

    pub fn last_child(&self) -> Option<usize> {
        self.last_child
    }
}
