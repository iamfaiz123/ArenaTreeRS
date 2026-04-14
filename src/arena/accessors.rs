use crate::id::NodeId;
use crate::node::Node;
use crate::arena::Arena;

impl<T> Arena<T> {
    /// Returns a reference to the node with the given ID. Panics if not found.
    pub fn get(&self, id: NodeId) -> &Node<T> {
        self.try_get(id).expect("Node ID not found in arena")
    }

    /// Returns an optional reference to the node with the given ID.
    pub fn try_get(&self, id: NodeId) -> Option<&Node<T>> {
        self.nodes.get(id.0)?.as_ref()
    }

    /// Returns a mutable reference to the node with the given ID. Panics if not found.
    pub fn get_mut(&mut self, id: NodeId) -> &mut Node<T> {
        self.try_get_mut(id).expect("Node ID not found in arena")
    }

    /// Returns an optional mutable reference to the node with the given ID.
    pub fn try_get_mut(&mut self, id: NodeId) -> Option<&mut Node<T>> {
        self.nodes.get_mut(id.0)?.as_mut()
    }

    /// Returns a reference to the value stored in the node.
    pub fn value(&self, id: NodeId) -> &T {
        &self.get(id).data
    }

    /// Returns a mutable reference to the value stored in the node.
    pub fn value_mut(&mut self, id: NodeId) -> &mut T {
        &mut self.get_mut(id).data
    }

    /// Sets the value of a node.
    pub fn set_value(&mut self, id: NodeId, value: T) {
        if let Some(node) = self.try_get_mut(id) {
            node.data = value;
        }
    }
}
