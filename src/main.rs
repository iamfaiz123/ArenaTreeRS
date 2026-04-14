use arena_tree::{Arena, NodeId};

fn main() {
    let mut arena = Arena::new();

    // 1. Creation & Relationships
    println!("--- 1. Building Tree ---");
    let root = arena.create_root("Earth");
    let asia = arena.insert_node("Asia", root).unwrap();
    let europe = arena.insert_node("Europe", root).unwrap();
    
    let india = arena.insert_node("India", asia).unwrap();
    let china = arena.insert_node("China", asia).unwrap();
    let uk = arena.insert_node("UK", europe).unwrap();
    let france = arena.insert_node("France", europe).unwrap();

    arena.print_tree(root);

    // 2. Traversal
    println!("\n--- 2. BFS Traversal (Levels) ---");
    for id in arena.bfs(root) {
        println!("Visited: {}", arena.value(id));
    }

    // 3. Structural Queries
    println!("\n--- 3. Structural Queries ---");
    println!("Depth of India: {}", arena.depth(india));
    println!("Height of Asia: {}", arena.height(asia));
    println!("Is Earth ancestor of France? {}", arena.is_ancestor(root, france));
    println!("Subtree size of Europe: {}", arena.subtree_size(europe));

    // 4. Lowest Common Ancestor
    println!("\n--- 4. LCA ---");
    if let Some(lca) = arena.lowest_common_ancestor(india, france) {
        println!("LCA of India and France: {}", arena.value(lca));
    }

    // 5. Path Finding
    println!("\n--- 5. Path Finding ---");
    if let Some(path) = arena.path_between(china, uk) {
        let path_names: Vec<_> = path.iter().map(|&id| *arena.value(id)).collect();
        println!("Path China -> UK: {:?}", path_names);
    }

    // 6. Advanced Operations: Cloning
    println!("\n--- 6. Cloning Subtree (Asia) ---");
    let asia_clone = arena.clone_subtree(asia);
    arena.set_value(asia_clone, "Asia (Clone)");
    println!("New root for cloned Asia: {}", arena.value(asia_clone));
    
    // Attach cloned Asia to Earth for visualization
    arena.add_child(root, asia_clone).unwrap();
    arena.print_tree(root);

    // 7. Modification: Replace Node
    println!("\n--- 7. Replacing 'UK' with 'Spain' ---");
    let spain = arena.add_node("Spain");
    arena.replace_node(uk, spain);
    arena.print_tree(root);

    // 8. Memory Management
    println!("\n--- 8. Memory Status ---");
    println!("Active nodes: {}", arena.node_count());
    println!("Capacity: {}", arena.capacity());
}
