#[cfg(test)]
mod tests {
    use crate::arena::Arena;

    #[test]
    fn test_core_api() {
        let mut arena: Arena<i32> = Arena::new();
        let _r = arena.create_root(10);
        assert_eq!(arena.len(), 1);
        assert!(!arena.is_empty());
        arena.clear();
        assert!(arena.is_empty());
    }

    #[test]
    fn test_relationships() {
        let mut arena = Arena::new();
        let r = arena.add_node("root");
        let c1 = arena.insert_node("c1", r).unwrap();
        let _c2 = arena.insert_node("c2", r).unwrap();
        
        assert_eq!(arena.parent(c1), Some(r));
        assert_eq!(arena.children(r).len(), 2);
        assert!(arena.is_root(r));
        assert!(arena.is_leaf(c1));
    }

    #[test]
    fn test_traversals() {
        let mut arena = Arena::new();
        let r = arena.add_node(1);
        let c1 = arena.insert_node(2, r).unwrap();
        let c2 = arena.insert_node(3, r).unwrap();
        let g1 = arena.insert_node(4, c1).unwrap();

        let bfs: Vec<_> = arena.bfs(r).collect();
        assert_eq!(bfs, vec![r, c1, c2, g1]);

        let dfs: Vec<_> = arena.dfs_preorder(r).collect();
        assert_eq!(dfs, vec![r, c1, g1, c2]);
    }
}
