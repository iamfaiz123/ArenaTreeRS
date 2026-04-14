/// A unique identifier for a node in the arena.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NodeId(pub(crate) usize);

impl From<usize> for NodeId {
    fn from(id: usize) -> Self {
        NodeId(id)
    }
}

impl NodeId {
    /// Returns the raw index of the node.
    pub fn index(self) -> usize {
        self.0
    }
}
