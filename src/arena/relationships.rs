use crate::id::NodeId;
use crate::arena::Arena;

impl<T> Arena<T> {
    /// Returns the parent NodeId of the given node.
    pub fn parent(&self, id: NodeId) -> Option<NodeId> {
        self.try_get(id)?.parent()
    }

    /// Returns a slice of child NodeIds for the given node.
    pub fn children(&self, id: NodeId) -> &[NodeId] {
        self.try_get(id).map(|n| n.children()).unwrap_or(&[])
    }

    /// Returns a mutable reference to the children vector for the given node.
    pub fn children_mut(&mut self, id: NodeId) -> &mut Vec<NodeId> {
        &mut self.get_mut(id).children
    }

    /// Finds the absolute root of the tree containing the given node.
    pub fn root(&self, id: NodeId) -> NodeId {
        let mut current = id;
        while let Some(p_id) = self.parent(current) {
            current = p_id;
        }
        current
    }

    /// Returns true if the node has no parent.
    pub fn is_root(&self, id: NodeId) -> bool {
        self.parent(id).is_none() && self.contains(id)
    }

    /// Returns true if the node has no children.
    pub fn is_leaf(&self, id: NodeId) -> bool {
        self.children(id).is_empty()
    }

    /// Returns all siblings of the node (excluding itself).
    pub fn siblings(&self, id: NodeId) -> Vec<NodeId> {
        if let Some(p_id) = self.parent(id) {
            self.children(p_id)
                .iter()
                .filter(|&&cid| cid != id)
                .copied()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Returns the next sibling in the parent's child list.
    pub fn next_sibling(&self, id: NodeId) -> Option<NodeId> {
        let p_id = self.parent(id)?;
        let children = self.children(p_id);
        let pos = children.iter().position(|&cid| cid == id)?;
        children.get(pos + 1).copied()
    }

    /// Returns the previous sibling in the parent's child list.
    pub fn prev_sibling(&self, id: NodeId) -> Option<NodeId> {
        let p_id = self.parent(id)?;
        let children = self.children(p_id);
        let pos = children.iter().position(|&cid| cid == id)?;
        if pos > 0 {
            children.get(pos - 1).copied()
        } else {
            None
        }
    }

    /// Returns the first child of the node.
    pub fn first_child(&self, id: NodeId) -> Option<NodeId> {
        self.children(id).first().copied()
    }

    /// Returns the last child of the node.
    pub fn last_child(&self, id: NodeId) -> Option<NodeId> {
        self.children(id).last().copied()
    }
}
