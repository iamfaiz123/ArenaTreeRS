use std::collections::VecDeque;
use crate::id::NodeId;
use crate::arena::Arena;

impl<T> Arena<T> {
    /// Iterates through all active nodes in the arena order.
    pub fn iter(&self) -> impl Iterator<Item = (NodeId, &T)> {
        self.nodes.iter().enumerate().filter_map(|(i, n)| {
            n.as_ref().map(|node| (NodeId(i), node.data()))
        })
    }

    /// Mutable iteration through all active nodes.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (NodeId, &mut T)> {
        self.nodes.iter_mut().enumerate().filter_map(|(i, n)| {
            n.as_mut().map(|node| (NodeId(i), node.data_mut()))
        })
    }

    /// Breath-First Search iterator starting from node_id.
    pub fn bfs(&self, id: NodeId) -> LevelOrderIterator<T> {
        self.level_order(id)
    }

    /// Level-order (BFS) iterator.
    pub fn level_order(&self, id: NodeId) -> LevelOrderIterator<T> {
        let mut queue = VecDeque::new();
        if self.contains(id) {
            queue.push_back(id);
        }
        LevelOrderIterator { arena: self, queue }
    }

    /// DFS Pre-order iterator.
    pub fn dfs_preorder(&self, id: NodeId) -> DfsIterator<T> {
        let mut stack = Vec::new();
        if self.contains(id) {
            stack.push(id);
        }
        DfsIterator {
            arena: self,
            stack,
        }
    }

    /// DFS Post-order iterator (returns a Vec as it's easier to implement).
    pub fn dfs_postorder(&self, id: NodeId) -> Vec<NodeId> {
        let mut res = Vec::new();
        let mut stack = vec![id];
        while let Some(current) = stack.pop() {
            res.push(current);
            for &child in self.children(current) {
                stack.push(child);
            }
        }
        res.reverse();
        res
    }

    /// Iterates through ancestors of a node.
    pub fn ancestors(&self, id: NodeId) -> AncestorsIterator<T> {
        AncestorsIterator {
            arena: self,
            current: self.parent(id),
        }
    }

    /// Iterates through descendants of a node.
    pub fn descendants(&self, id: NodeId) -> DfsIterator<T> {
        self.dfs_preorder(id)
    }

    /// Iterates through children of a node.
    pub fn children_iter(&self, id: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.children(id).iter().copied()
    }
}

pub struct LevelOrderIterator<'a, T> {
    arena: &'a Arena<T>,
    queue: VecDeque<NodeId>,
}

impl<'a, T> Iterator for LevelOrderIterator<'a, T> {
    type Item = NodeId;
    fn next(&mut self) -> Option<Self::Item> {
        let id = self.queue.pop_front()?;
        for &child_id in self.arena.children(id) {
            self.queue.push_back(child_id);
        }
        Some(id)
    }
}

pub struct DfsIterator<'a, T> {
    arena: &'a Arena<T>,
    stack: Vec<NodeId>,
}

impl<'a, T> Iterator for DfsIterator<'a, T> {
    type Item = NodeId;
    fn next(&mut self) -> Option<Self::Item> {
        let id = self.stack.pop()?;
        for &child_id in self.arena.children(id).iter().rev() {
            self.stack.push(child_id);
        }
        Some(id)
    }
}

pub struct AncestorsIterator<'a, T> {
    arena: &'a Arena<T>,
    current: Option<NodeId>,
}

impl<'a, T> Iterator for AncestorsIterator<'a, T> {
    type Item = NodeId;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        if let Some(id) = self.current {
            self.current = self.arena.parent(id);
        }
        result
    }
}
