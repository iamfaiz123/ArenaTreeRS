use std::fmt;
use crate::id::NodeId;
use crate::arena::Arena;

impl<T: fmt::Display> Arena<T> {
    /// Prints the tree starting at root to stdout.
    pub fn print_tree(&self, root: NodeId) {
        self.print_node(root, 0);
    }

    fn print_node(&self, id: NodeId, indent: usize) {
        if let Some(node) = self.try_get(id) {
            println!("{}{}", "  ".repeat(indent), node.data());
            for &child_id in node.children() {
                self.print_node(child_id, indent + 1);
            }
        }
    }
}

impl<T: fmt::Debug> Arena<T> {
    /// Prints debug representation of the tree.
    pub fn debug_tree(&self, root: NodeId) {
        self.debug_node(root, 0);
    }

    fn debug_node(&self, id: NodeId, indent: usize) {
        if let Some(node) = self.try_get(id) {
            println!("{}{:?} (ID: {:?})", "  ".repeat(indent), node.data(), id);
            for &child_id in node.children() {
                self.debug_node(child_id, indent + 1);
            }
        }
    }
}
