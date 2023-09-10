#[cfg(test)]
mod tests {

    use crate::tree::{root_idx, NodeData, NodeType, Tree};
    use std::path::PathBuf;

    #[test]
    fn check_same_files() {
        let mut dataset_1 = Tree::new();

        dataset_1.add_child(
            root_idx(),
            NodeData::new(PathBuf::from("1"), NodeType::Directory),
        );
        dataset_1.add_child(
            root_idx(),
            NodeData::new(PathBuf::from("2"), NodeType::Directory),
        );
        dataset_1.add_child(
            root_idx(),
            NodeData::new(PathBuf::from("3"), NodeType::Directory),
        );

        dataset_1.add_child(
            1,
            NodeData::new(PathBuf::from("file_a.txt"), NodeType::File),
        );
        dataset_1.add_child(
            1,
            NodeData::new(PathBuf::from("file_b.txt"), NodeType::File),
        );
        dataset_1.add_child(
            1,
            NodeData::new(PathBuf::from("file_c.txt"), NodeType::File),
        );
        dataset_1.add_child(
            2,
            NodeData::new(PathBuf::from("file_d.txt"), NodeType::File),
        );
        dataset_1.add_child(
            2,
            NodeData::new(PathBuf::from("file_e.txt"), NodeType::File),
        );
        dataset_1.add_child(
            2,
            NodeData::new(PathBuf::from("file_f.txt"), NodeType::File),
        );
        dataset_1.add_child(
            3,
            NodeData::new(PathBuf::from("file_g.txt"), NodeType::File),
        );
        dataset_1.add_child(
            3,
            NodeData::new(PathBuf::from("file_h.txt"), NodeType::File),
        );
        dataset_1.add_child(
            3,
            NodeData::new(PathBuf::from("file_i.txt"), NodeType::File),
        );

        let mut dataset_2 = Tree::new();

        dataset_2.add_child(
            root_idx(),
            NodeData::new(PathBuf::from("1"), NodeType::Directory),
        );
        dataset_2.add_child(
            root_idx(),
            NodeData::new(PathBuf::from("1"), NodeType::Directory),
        ); // 2
        dataset_2.add_child(
            root_idx(),
            NodeData::new(PathBuf::from("2"), NodeType::Directory),
        ); // 3
        dataset_2.add_child(
            root_idx(),
            NodeData::new(PathBuf::from("3"), NodeType::Directory),
        ); // 4
        dataset_2.add_child(
            root_idx(),
            NodeData::new(PathBuf::from("3"), NodeType::Directory),
        ); // 5

        dataset_2.add_child(
            1,
            NodeData::new(PathBuf::from("file_a.txt"), NodeType::File),
        );
        dataset_2.add_child(
            1,
            NodeData::new(PathBuf::from("file_b.txt"), NodeType::File),
        );
        dataset_2.add_child(
            2,
            NodeData::new(PathBuf::from("file_c.txt"), NodeType::File),
        );
        dataset_2.add_child(
            2,
            NodeData::new(PathBuf::from("file_d.txt"), NodeType::File),
        );
        dataset_2.add_child(
            3,
            NodeData::new(PathBuf::from("file_e.txt"), NodeType::File),
        );
        dataset_2.add_child(
            3,
            NodeData::new(PathBuf::from("file_f.txt"), NodeType::File),
        );
        dataset_2.add_child(
            3,
            NodeData::new(PathBuf::from("file_g.txt"), NodeType::File),
        );
        dataset_2.add_child(
            4,
            NodeData::new(PathBuf::from("file_h.txt"), NodeType::File),
        );
        dataset_2.add_child(
            5,
            NodeData::new(PathBuf::from("file_i.txt"), NodeType::File),
        );

        assert_eq!(dataset_1.get_file_names(), dataset_2.get_file_names());
    }

    // #[test]
    // fn test_get_children() {
    //     let mut dataset = Tree::new();

    //     dataset.add_child(
    //         root_idx(),
    //         NodeData::new(PathBuf::from("1"), NodeType::Directory),
    //     );
    //     dataset.add_child(
    //         root_idx(),
    //         NodeData::new(PathBuf::from("2"), NodeType::Directory),
    //     );
    //     dataset.add_child(
    //         root_idx(),
    //         NodeData::new(PathBuf::from("3"), NodeType::Directory),
    //     );

    //     dataset.add_child(
    //         1,
    //         NodeData::new(PathBuf::from("file_a.txt"), NodeType::File),
    //     );
    //     dataset.add_child(
    //         1,
    //         NodeData::new(PathBuf::from("file_b.txt"), NodeType::File),
    //     );
    //     dataset.add_child(
    //         1,
    //         NodeData::new(PathBuf::from("file_c.txt"), NodeType::File),
    //     );
    //     dataset.add_child(
    //         2,
    //         NodeData::new(PathBuf::from("file_d.txt"), NodeType::File),
    //     );
    //     dataset.add_child(
    //         2,
    //         NodeData::new(PathBuf::from("file_e.txt"), NodeType::File),
    //     );
    //     dataset.add_child(
    //         2,
    //         NodeData::new(PathBuf::from("file_f.txt"), NodeType::File),
    //     );
    //     dataset.add_child(
    //         3,
    //         NodeData::new(PathBuf::from("file_g.txt"), NodeType::File),
    //     );
    //     dataset.add_child(
    //         3,
    //         NodeData::new(PathBuf::from("file_h.txt"), NodeType::File),
    //     );
    //     dataset.add_child(
    //         3,
    //         NodeData::new(PathBuf::from("file_i.txt"), NodeType::File),
    //     );

    //     let children = dataset.get_children(2);
    //     let mut i = 7;
    //     for child in children {
    //         assert_eq!(child, i);
    //         i += 1;
    //     }

    //     match dataset.get_child(1, "file_b.txt".to_string()) {
    //         Some(&value) => println!("Child index: {}", value),
    //         None => println!("Child not found"),
    //     }

    //     assert_eq!(dataset.get_child(1, "file_b.txt".to_string()).unwrap(), &5);
    //     assert_eq!(dataset.get_child(3, "file_h.txt".to_string()).unwrap(), &11);
    // }
}
