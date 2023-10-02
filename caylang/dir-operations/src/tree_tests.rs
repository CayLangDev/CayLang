#[cfg(test)]
mod tests {

    use crate::tree::{NodeData, NodeType, Tree};
    use std::path::PathBuf;

    #[test]
    fn test_tree_creation() {
        let tree = Tree::new();
        assert_eq!(tree.nodes.len(), 1);
        assert_eq!(tree.path_map.len(), 1);
        assert_eq!(tree.nodes[0].data.node_type, NodeType::Directory);
    }

    #[test]
    fn test_add_child() {
        let mut tree = Tree::new();
        let parent_data = NodeData::new(PathBuf::from("parent"), NodeType::Directory);
        let parent_idx = tree.add_child(0, parent_data.clone());

        let child_data = NodeData::new(PathBuf::from("parent/child1"), NodeType::File);
        let child_idx = tree.add_child(parent_idx, child_data.clone());

        assert_eq!(tree.nodes.len(), 3);
        assert_eq!(tree.path_map.len(), 3);
        assert_eq!(tree.nodes[child_idx].data, child_data);
        assert_eq!(tree.nodes[parent_idx].children, vec![child_idx]);
    }

    #[test]
    fn test_add_node() {
        let mut tree = Tree::new();
        let node_data = NodeData::new(PathBuf::from("child1"), NodeType::Directory);
        tree.add_node(node_data.clone());

        assert_eq!(tree.nodes.len(), 2);
        assert_eq!(tree.path_map.len(), 2);
        assert_eq!(tree.nodes[1].data, node_data);
    }
    #[test]
    fn test_add_node_with_implicit_parents() {
        let mut tree = Tree::new();

        let child_data1 = NodeData::new(PathBuf::from("parent/child1"), NodeType::File);
        let child_idx1 = tree.add_node(child_data1.clone());

        let child_data2 = NodeData::new(PathBuf::from("parent/child2"), NodeType::File);
        let child_idx2 = tree.add_node(child_data2.clone());

        assert_eq!(tree.nodes.len(), 4);
        assert_eq!(tree.path_map.len(), 4);

        let parent_idx = tree.path_map.get(&PathBuf::from("parent")).unwrap();
        let root_idx = tree.path_map.get(&PathBuf::from("")).unwrap();

        assert_eq!(
            tree.nodes[*parent_idx].children,
            vec![child_idx1, child_idx2]
        );
        assert_eq!(tree.nodes[child_idx1].data, child_data1);
        assert_eq!(tree.nodes[child_idx2].data, child_data2);
        assert_eq!(tree.nodes[*root_idx].children, vec![*parent_idx]);
    }

    #[test]
    fn test_get_node_by_path() {
        let mut tree = Tree::new();
        let node_data = NodeData::new(PathBuf::from("parent"), NodeType::Directory);
        let node_idx = tree.add_node(node_data.clone());

        let path = PathBuf::from("parent");
        let result = tree.get_node_by_path(&path);

        assert_eq!(result, Some(&tree.nodes[node_idx]));
    }

    #[test]
    fn test_get_children() {
        let mut tree = Tree::new();
        let parent_data = NodeData::new(PathBuf::from("parent"), NodeType::Directory);
        let parent_idx = tree.add_node(parent_data.clone());

        let child1_data = NodeData::new(PathBuf::from("parent/child1"), NodeType::File);
        let child1_idx = tree.add_child(parent_idx, child1_data.clone());

        let child2_data = NodeData::new(PathBuf::from("parent/child2"), NodeType::File);
        let child2_idx = tree.add_child(parent_idx, child2_data.clone());

        let children = tree.get_children(parent_idx).collect::<Vec<_>>();

        assert_eq!(children, vec![child1_idx, child2_idx]);
    }

    #[test]
    fn test_get_child() {
        let mut tree = Tree::new();
        let parent_data = NodeData::new(PathBuf::from("parent"), NodeType::Directory);
        let parent_idx = tree.add_node(parent_data.clone());

        let child1_data = NodeData::new(PathBuf::from("parent/child1"), NodeType::File);
        let _child1_idx = tree.add_child(parent_idx, child1_data.clone());

        let child2_data = NodeData::new(PathBuf::from("parent/child2"), NodeType::File);
        let child2_idx = tree.add_child(parent_idx, child2_data.clone());

        let result = tree.get_child(parent_idx, "child2".to_string());

        assert_eq!(result, Some(&child2_idx));
    }

    #[test]
    fn test_from_fold_flatten() {
        let mut tree = Tree::new();

        let node_data1 = NodeData::new(PathBuf::from("parent/child1"), NodeType::File);
        let node_data2 = NodeData::new(PathBuf::from("parent/child2"), NodeType::File);

        tree.add_node(node_data1.clone());
        tree.add_node(node_data2.clone());

        let new_tree = Tree::from_fold_function(&tree, |node| {
            let string = node.data.path.to_str().unwrap();
            PathBuf::from(str::replace(string, "/", "_"))
        });

        assert_eq!(new_tree.nodes.len(), 3);
        assert_eq!(new_tree.path_map.len(), 3);

        assert_eq!(new_tree.nodes[0].data.path, PathBuf::from(""));
        assert_eq!(new_tree.nodes[1].data.path, PathBuf::from("parent_child1"));
        assert_eq!(new_tree.nodes[2].data.path, PathBuf::from("parent_child2"));
    }
}
