use crate::id::NodeId;
use crate::node::Node;

/// The main container for nodes.
///
/// An `Arena` stores nodes in a vector and uses `NodeId` to manage relationships.
/// This approach avoids ownership issues and provides good cache locality.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Arena<T> {
    pub(crate) nodes: Vec<Option<Node<T>>>,
    pub(crate) free_indices: Vec<usize>,
    pub(crate) count: usize,
}

pub mod accessors;
pub mod relationships;
pub mod modification;
pub mod iter;
pub mod queries;
pub mod advanced;
pub mod visualize;

impl<T> Arena<T> {
    /// Creates a new, empty arena.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            free_indices: Vec::new(),
            count: 0,
        }
    }

    /// Creates a new arena with the specified capacity.
    pub fn with_capacity(n: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(n),
            free_indices: Vec::new(),
            count: 0,
        }
    }

    /// Returns the total number of active nodes in the arena.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Returns the current capacity of the arena.
    pub fn capacity(&self) -> usize {
        self.nodes.capacity()
    }

    /// Returns true if the arena contains no active nodes.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Clears the arena, removing all nodes and resetting the count.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.free_indices.clear();
        self.count = 0;
    }

    /// Reserves capacity for at least `n` more nodes.
    pub fn reserve(&mut self, n: usize) {
        self.nodes.reserve(n);
    }

    /// Shriks the capacity of the arena as much as possible.
    pub fn shrink_to_fit(&mut self) {
        self.nodes.shrink_to_fit();
        self.free_indices.shrink_to_fit();
    }

    /// Adds a node with the given value to the arena. Returns its NodeId.
    pub fn add_node(&mut self, value: T) -> NodeId {
        let node = Node {
            data: value,
            parent: None,
            children: Vec::new(),
        };

        self.count += 1;
        if let Some(index) = self.free_indices.pop() {
            self.nodes[index] = Some(node);
            NodeId(index)
        } else {
            let index = self.nodes.len();
            self.nodes.push(Some(node));
            NodeId(index)
        }
    }

    /// Creates a node as a root (equivalent to add_node).
    pub fn create_root(&mut self, value: T) -> NodeId {
        self.add_node(value)
    }

    /// Returns true if the arena contains an active node with this ID.
    pub fn contains(&self, id: NodeId) -> bool {
        self.nodes.get(id.0).map_or(false, |n| n.is_some())
    }

    /// Validates if a NodeId is valid and active.
    pub fn validate(&self, id: NodeId) -> bool {
        self.contains(id)
    }

    /// Returns total active node count.
    pub fn node_count(&self) -> usize {
        self.count
    }
}
