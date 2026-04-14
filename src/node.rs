use crate::id::NodeId;

/// A node in the tree arena.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Node<T> {
    pub(crate) data: T,
    pub(crate) parent: Option<NodeId>,
    pub(crate) children: Vec<NodeId>,
}

impl<T> Node<T> {
    /// Returns the data stored in the node.
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Returns a mutable reference to the data stored in the node.
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    /// Returns the parent NodeId, if any.
    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }

    /// Returns a slice of child NodeIds.
    pub fn children(&self) -> &[NodeId] {
        &self.children
    }
}
