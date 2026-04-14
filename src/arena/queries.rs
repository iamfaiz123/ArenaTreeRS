use crate::id::NodeId;
use crate::arena::Arena;

impl<T> Arena<T> {
    /// Returns the depth of the node (0 for root).
    pub fn depth(&self, id: NodeId) -> usize {
        self.ancestors(id).count()
    }

    /// Returns the height of the subtree starting at this node.
    pub fn height(&self, id: NodeId) -> usize {
        let children = self.children(id);
        if children.is_empty() {
            0
        } else {
            1 + children.iter().map(|&cid| self.height(cid)).max().unwrap_or(0)
        }
    }

    /// Returns the number of nodes in the subtree (including self).
    pub fn subtree_size(&self, id: NodeId) -> usize {
        1 + self.children(id).iter().map(|&cid| self.subtree_size(cid)).sum::<usize>()
    }

    /// Returns true if `a` is an ancestor of `b`.
    pub fn is_ancestor(&self, a: NodeId, b: NodeId) -> bool {
        self.ancestors(b).any(|id| id == a)
    }

    /// Returns true if `a` is a descendant of `b`.
    pub fn is_descendant(&self, a: NodeId, b: NodeId) -> bool {
        self.is_ancestor(b, a)
    }

    /// Returns the path from the node to the root.
    pub fn path_to_root(&self, id: NodeId) -> Vec<NodeId> {
        let mut path = vec![id];
        path.extend(self.ancestors(id));
        path
    }

    /// Returns the path between two nodes.
    pub fn path_between(&self, a: NodeId, b: NodeId) -> Option<Vec<NodeId>> {
        let lca = self.lowest_common_ancestor(a, b)?;
        let mut path_a = Vec::new();
        let mut curr = a;
        while curr != lca {
            path_a.push(curr);
            curr = self.parent(curr).unwrap();
        }
        path_a.push(lca);
        
        let mut path_b = Vec::new();
        let mut curr = b;
        while curr != lca {
            path_b.push(curr);
            curr = self.parent(curr).unwrap();
        }
        path_b.reverse();
        path_a.extend(path_b);
        Some(path_a)
    }

    /// Returns the Lowest Common Ancestor (LCA) of two nodes.
    pub fn lowest_common_ancestor(&self, a: NodeId, b: NodeId) -> Option<NodeId> {
        let ancestors_a: Vec<NodeId> = self.path_to_root(a).into_iter().rev().collect();
        let ancestors_b: Vec<NodeId> = self.path_to_root(b).into_iter().rev().collect();
        
        let mut last_common = None;
        for (a_id, b_id) in ancestors_a.iter().zip(ancestors_b.iter()) {
            if a_id == b_id {
                last_common = Some(*a_id);
            } else {
                break;
            }
        }
        last_common
    }
}
