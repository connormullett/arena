pub struct Arena<T, K> {
    nodes: Vec<Node<T, K>>,
}

struct Node<T, K> {
    parent: Option<NodeId>,
    previous_sibling: Option<NodeId>,
    next_sibling: Option<NodeId>,
    first_child: Option<NodeId>,
    last_child: Option<NodeId>,

    pub key: K,
    pub data: T,
}

pub struct NodeId {
    index: usize,
}

impl<T, K> Arena<T, K>
where
    K: PartialEq,
{
    pub fn insert(&mut self, data: T, key: K) -> NodeId {
        // Get the next free index
        let next_index = self.nodes.len();

        // Push the node into the arena
        self.nodes.push(Node {
            parent: None,
            first_child: None,
            last_child: None,
            previous_sibling: None,
            next_sibling: None,
            key,
            data,
        });

        // Return the node identifier
        NodeId { index: next_index }
    }

    pub fn get(&self, key: K) -> Option<&T> {
        for node in self.nodes.iter() {
            if node.key == key {
                return Some(&node.data);
            }
        }

        None
    }
}
