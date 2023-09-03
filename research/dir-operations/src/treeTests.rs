#[cfg(test)]
mod tests {

    use crate::tree::{Tree, Node, NodeIdx, NodeData, NodeType, root_idx};
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::path::Path;

    #[test]
    fn check_same_files() {
        
        let mut dataset_1 = Tree::new();

        dataset_1.add_child(root_idx(), NodeData::new(PathBuf::from("1"), NodeType::Directory));
        dataset_1.add_child(root_idx(), NodeData::new(PathBuf::from("2"), NodeType::Directory));
        dataset_1.add_child(root_idx(), NodeData::new(PathBuf::from("3"), NodeType::Directory));

        dataset_1.add_child(1, NodeData::new(PathBuf::from("file_a.txt"), NodeType::File));
        dataset_1.add_child(1, NodeData::new(PathBuf::from("file_b.txt"), NodeType::File));
        dataset_1.add_child(1, NodeData::new(PathBuf::from("file_c.txt"), NodeType::File));
        dataset_1.add_child(2, NodeData::new(PathBuf::from("file_d.txt"), NodeType::File));
        dataset_1.add_child(2, NodeData::new(PathBuf::from("file_e.txt"), NodeType::File));
        dataset_1.add_child(2, NodeData::new(PathBuf::from("file_f.txt"), NodeType::File));
        dataset_1.add_child(3, NodeData::new(PathBuf::from("file_g.txt"), NodeType::File));
        dataset_1.add_child(3, NodeData::new(PathBuf::from("file_h.txt"), NodeType::File));
        dataset_1.add_child(3, NodeData::new(PathBuf::from("file_i.txt"), NodeType::File));

        let mut dataset_2 = Tree::new();

        dataset_2.add_child(root_idx(), NodeData::new(PathBuf::from("1"), NodeType::Directory));
        dataset_2.add_child(root_idx(), NodeData::new(PathBuf::from("1"), NodeType::Directory)); // 2
        dataset_2.add_child(root_idx(), NodeData::new(PathBuf::from("2"), NodeType::Directory)); // 3
        dataset_2.add_child(root_idx(), NodeData::new(PathBuf::from("3"), NodeType::Directory)); // 4
        dataset_2.add_child(root_idx(), NodeData::new(PathBuf::from("3"), NodeType::Directory)); // 5

        dataset_2.add_child(1, NodeData::new(PathBuf::from("file_a.txt"), NodeType::File));
        dataset_2.add_child(1, NodeData::new(PathBuf::from("file_b.txt"), NodeType::File));
        dataset_2.add_child(2, NodeData::new(PathBuf::from("file_c.txt"), NodeType::File));
        dataset_2.add_child(2, NodeData::new(PathBuf::from("file_d.txt"), NodeType::File));
        dataset_2.add_child(3, NodeData::new(PathBuf::from("file_e.txt"), NodeType::File));
        dataset_2.add_child(3, NodeData::new(PathBuf::from("file_f.txt"), NodeType::File));
        dataset_2.add_child(3, NodeData::new(PathBuf::from("file_g.txt"), NodeType::File));
        dataset_2.add_child(4, NodeData::new(PathBuf::from("file_h.txt"), NodeType::File));
        dataset_2.add_child(5, NodeData::new(PathBuf::from("file_i.txt"), NodeType::File));

        assert_eq!(dataset_1.get_file_names(), dataset_2.get_file_names());
    }
}