use crate::id::NodeId;
use crate::arena::Arena;

impl<T> Arena<T> {
    /// Creates a node and attaches it to a parent.
    pub fn insert_node(&mut self, value: T, parent_id: NodeId) -> Result<NodeId, &'static str> {
        if !self.contains(parent_id) {
            return Err("Parent node does not exist");
        }
        let child_id = self.add_node(value);
        self.add_child(parent_id, child_id).map(|_| child_id)
    }

    /// Adds a child to a parent node.
    pub fn add_child(&mut self, parent_id: NodeId, child_id: NodeId) -> Result<(), &'static str> {
        if parent_id == child_id {
            return Err("Cannot add node as its own child");
        }
        if self.is_descendant(parent_id, child_id) {
            return Err("Cycle detected: parent is a descendant of the child");
        }

        self.detach(child_id);

        if let Some(parent) = self.try_get_mut(parent_id) {
            parent.children.push(child_id);
        } else {
            return Err("Parent does not exist");
        }

        if let Some(child) = self.try_get_mut(child_id) {
            child.parent = Some(parent_id);
        }

        Ok(())
    }

    /// Inserts a child at a specific index in the parent's child list.
    pub fn insert_child_at(&mut self, parent_id: NodeId, index: usize, child_id: NodeId) -> Result<(), &'static str> {
        if !self.contains(parent_id) || !self.contains(child_id) {
            return Err("Node not in arena");
        }
        self.detach(child_id);

        let parent = self.get_mut(parent_id);
        if index > parent.children().len() {
            return Err("Index out of bounds");
        }
        parent.children.insert(index, child_id);
        self.get_mut(child_id).parent = Some(parent_id);
        Ok(())
    }

    /// Removes a specific child from a parent.
    pub fn remove_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        if let Some(parent) = self.try_get_mut(parent_id) {
            if parent.children().contains(&child_id) {
                self.detach(child_id);
            }
        }
    }

    /// Detaches a node from its parent, turning it into a root.
    pub fn detach(&mut self, node_id: NodeId) {
        let parent_id = self.parent(node_id);

        if let Some(p_id) = parent_id {
            if let Some(parent) = self.try_get_mut(p_id) {
                parent.children.retain(|&id| id != node_id);
            }
            if let Some(node) = self.try_get_mut(node_id) {
                node.parent = None;
            }
        }
    }

    /// Moves a node to a new parent.
    pub fn move_node(&mut self, id: NodeId, new_parent_id: NodeId) -> Result<(), &'static str> {
        self.add_child(new_parent_id, id)
    }

    /// Swaps the position of two nodes (and their subtrees) in the arena.
    pub fn swap_nodes(&mut self, a_id: NodeId, b_id: NodeId) {
        if a_id == b_id || !self.contains(a_id) || !self.contains(b_id) {
            return;
        }

        let p_a = self.parent(a_id);
        let p_b = self.parent(b_id);

        let idx_a = p_a.and_then(|p| self.children(p).iter().position(|&idx| idx == a_id));
        let idx_b = p_b.and_then(|p| self.children(p).iter().position(|&idx| idx == b_id));

        self.get_mut(a_id).parent = p_b;
        self.get_mut(b_id).parent = p_a;

        if let (Some(p1), Some(i1)) = (p_a, idx_a) {
            self.children_mut(p1)[i1] = b_id;
        }
        if let (Some(p2), Some(i2)) = (p_b, idx_b) {
            self.children_mut(p2)[i2] = a_id;
        }
    }

    /// Replaces an old node with a new node, maintaining the same parent and child connections.
    pub fn replace_node(&mut self, old_id: NodeId, new_id: NodeId) {
        if old_id == new_id || !self.contains(old_id) || !self.contains(new_id) {
            return;
        }

        let parent = self.parent(old_id);
        let children = std::mem::take(&mut self.get_mut(old_id).children);

        if let Some(p_id) = parent {
            if let Some(pos) = self.children(p_id).iter().position(|&id| id == old_id) {
                self.children_mut(p_id)[pos] = new_id;
            }
        }
        self.get_mut(new_id).parent = parent;

        for &child_id in &children {
            self.get_mut(child_id).parent = Some(new_id);
        }
        self.get_mut(new_id).children = children;

        self.get_mut(old_id).parent = None;
    }

    /// Removes a single node from the arena. Children are detached and become roots.
    pub fn remove_node(&mut self, node_id: NodeId) -> Option<T> {
        if !self.contains(node_id) {
            return None;
        }

        self.detach(node_id);

        let children = std::mem::take(&mut self.get_mut(node_id).children);
        for child_id in children {
            if let Some(child) = self.try_get_mut(child_id) {
                child.parent = None;
            }
        }

        let node_data = self.nodes[node_id.0].take().unwrap().data;
        self.free_indices.push(node_id.0);
        self.count -= 1;
        Some(node_data)
    }

    /// Removes a node and its entire subtree recursively.
    pub fn remove_subtree(&mut self, node_id: NodeId) {
        if !self.contains(node_id) {
            return;
        }

        self.detach(node_id);

        let mut stack = vec![node_id];
        while let Some(id) = stack.pop() {
            if let Some(node) = self.nodes[id.0].take() {
                for child_id in node.children {
                    stack.push(child_id);
                }
                self.free_indices.push(id.0);
                self.count -= 1;
            }
        }
    }
}
