use crate::id::NodeId;
use crate::arena::Arena;

impl<T: Clone> Arena<T> {
    /// Recursively clones a subtree into a new arena branch.
    pub fn clone_subtree(&mut self, id: NodeId) -> NodeId {
        let data = self.value(id).clone();
        let new_id = self.add_node(data);
        let children: Vec<NodeId> = self.children(id).to_vec();
        for child_id in children {
            let new_child = self.clone_subtree(child_id);
            self.add_child(new_id, new_child).ok();
        }
        new_id
    }

    /// Copies a subtree and attaches it to a new parent.
    pub fn copy_subtree(&mut self, id: NodeId, new_parent: NodeId) -> Result<NodeId, &'static str> {
        let root = self.clone_subtree(id);
        self.add_child(new_parent, root).map(|_| root)
    }
}

impl<T> Arena<T> {
    /// Finds the first node satisfying the predicate.
    pub fn find<F>(&self, predicate: F) -> Option<NodeId>
    where
        F: Fn(&T) -> bool,
    {
        self.iter().find(|(_, data)| predicate(data)).map(|(id, _)| id)
    }

    /// Finds all nodes satisfying the predicate.
    pub fn find_all<F>(&self, predicate: F) -> Vec<NodeId>
    where
        F: Fn(&T) -> bool,
    {
        self.iter().filter(|(_, data)| predicate(data)).map(|(id, _)| id).collect()
    }
}
