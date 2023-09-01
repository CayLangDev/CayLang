#[cfg(test)]
mod tests {

    use crate::tree;

    #[test]
    fn test_add_child() {
        let tree = tree::Tree::new();
        let root = root_idx();
        tree.add_child(root, tree::Node::new(path, tree::NodeType::Directory));
        assert_eq!()
    }

}