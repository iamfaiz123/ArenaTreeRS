//! A modular, feature-rich Arena Tree library for Rust.

mod id;
mod node;
mod arena;
mod tests;

pub use id::NodeId;
pub use node::Node;
pub use arena::Arena;

// Re-export common iterators if needed
pub use arena::iter::{LevelOrderIterator, DfsIterator, AncestorsIterator};
