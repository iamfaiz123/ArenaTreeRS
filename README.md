# ArenaTreeRs

A high-performance, feature-rich Arena Tree library for Rust.

`ArenaTreeRs` is a memory-efficient tree data structure where nodes are stored in a contiguous vector (the "arena"). Relationships are maintained using type-safe indices (`NodeId`), providing extreme performance and avoiding the common "borrow checker" complexities of pointer-based trees in Rust.

---

## 🚀 Why ArenaTreeRs?

### 1. Cache Locality
Because nodes are stored next to each other in a `Vec<Node<T>>`, tree traversals are much more cache-friendly than pointer-based trees. This significantly reduces CPU cache misses.

### 2. Zero-Overhead Safety
By using `NodeId` indices instead of `Rc<RefCell<Node<T>>>`, we avoid:
- **Reference counting overhead** (atomic operations in `Arc`).
- **Runtime borrow checking** (RefCell panics or overhead).
- **Circular dependencies** (No memory leaks from circular `Rc` paths).

### 3. Smart Memory Reuse
The arena maintains a **Free List**. When a node is deleted, its index is reserved for the next node creation. This prevents frequent reallocations and keeps the arena compact.

---

## 🛠️ Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
arena_tree_rs = "0.1.0"
```

---

## 🔧 Core API

### Node Creation
- `add_node(value)`: Adds a node to the arena.
- `insert_node(value, parent)`: Adds a node and attaches it to a parent.
- `create_root(value)`: Explicit root creation.

### Relationship Management
- `add_child(parent, child)`: Link two existing nodes.
- `insert_child_at(parent, index, child)`: Precise positioning.
- `move_node(id, new_parent)`: Re-parent a subtree.
- `detach(id)`: Turn a subtree into a standalone tree.

### Traversal & Search
- `bfs(id)` / `level_order(id)`: Breadth-first search.
- `dfs_preorder(id)` / `dfs_postorder(id)`: Depth-first search.
- `ancestors(id)` / `descendants(id)`: Directional iterators.
- `find(predicate)` / `find_all(predicate)`: Search the arena.

### Advanced Queries
- `lowest_common_ancestor(a, b)`: Find the nearest shared parent.
- `path_between(a, b)`: Get an ordered list of nodes between two points.
- `depth(id)` / `height(id)`: Calculate tree metrics.

---

## 💡 Examples

### 1. Basic Tree Building
```rust
use arena_tree_rs::{Arena, NodeId};

fn main() {
    let mut arena = Arena::new();
    let root = arena.create_root("Earth");
    
    let asia = arena.insert_node("Asia", root).unwrap();
    let india = arena.insert_node("India", asia).unwrap();
    let china = arena.insert_node("China", asia).unwrap();
    
    // Quick relationship check
    assert!(arena.is_ancestor(root, india));
    assert_eq!(arena.depth(india), 2);
}
```

### 2. Path Finding & LCA
```rust
let sea = arena.insert_node("SEA", asia).unwrap();
let th = arena.insert_node("Thailand", sea).unwrap();
let fr = arena.insert_node("France", root).unwrap();

// Find the LCA of Thailand and France
if let Some(lca) = arena.lowest_common_ancestor(th, fr) {
    println!("Shared ancestor: {}", arena.value(lca)); // "Earth"
}

// Get the path from China to Thailand
if let Some(path) = arena.path_between(china, th) {
    for id in path {
        println!("Path step: {}", arena.value(id));
    }
}
```

### 3. Structural Modification
```rust
let uk = arena.new_node("UK");
let spain = arena.new_node("Spain");

// Swap nodes if needed
arena.swap_nodes(india, china);

// Replace a node while keeping its children
arena.replace_node(uk, spain);
```

---

## 📊 Performance at a Glance

| Operation | Complexity | Note |
|-----------|------------|------|
| `add_node` | O(1) | Amortized vector push / free-list reuse |
| `detach` | O(s) | `s` = number of siblings of the node |
| `children` | O(1) | Returns a slice slice `&[NodeId]` |
| `bfs` / `dfs` | O(n) | Where `n` is number of nodes in subtree |
| `LCA` | O(d) | Where `d` is the depth of the tree |

---

## 🧪 Testing

The library is strictly tested for edge cases including cycle detection and slot reuse.

```bash
cargo test
```
