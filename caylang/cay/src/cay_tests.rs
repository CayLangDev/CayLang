#[cfg(test)]
mod tests {
    use caylang_io::filesys::system_test;
    use caylang_io::filesys_builder::FileNode;
    use caylang_io::tree::{NodeData, NodeType, Tree};
    use std::fs::File;
    use std::path::PathBuf;

    #[test]
    fn system_basic_shuffle() {
        let cay_code = r#"
            TreeDirectorySet SmallTreeDir {
                Names: r"test_1",
                Structure: {
                    layers: {
                        A: Directory,
                        B: Directory
                    },
                    edges: {
                        F: File
                    }
                }
            }

            fold "<PATH>": SmallTreeDir {
                A { name as a } => {
                    B { name as b } => {
                        F {name as f} => `{b}/{a}/{f}`
                    }
                }
            }
        "#;

        let in_structure =
            FileNode::dir("a", vec![FileNode::dir("b", vec![FileNode::file("f.txt")])]);
        let out_structure =
            FileNode::dir("b", vec![FileNode::dir("a", vec![FileNode::file("f.txt")])]);

        system_test(cay_code, &in_structure, &out_structure);
    }

    #[test]
    fn system_partial_flatten() {
        let cay_code = r#"
            TreeDirectorySet SmallTreeDir {
                Names: r"test_1",
                Structure: {
                    layers: {
                        A: Directory,
                        B: Directory
                    },
                    edges: {
                        F: File
                    }
                }
            }

            fold "<PATH>": SmallTreeDir {
                A { name as a } => {
                    B { name as b } => {
                        F {name as f} => `{a}/{b}_{f}`
                    }
                }
            }
        "#;

        let in_structure =
            FileNode::dir("a", vec![FileNode::dir("b", vec![FileNode::file("f.txt")])]);
        let out_structure = FileNode::dir("a", vec![FileNode::file_content("b_f.txt", "f.txt")]);

        system_test(cay_code, &in_structure, &out_structure);
    }

    #[test]
    fn system_full_flatten() {
        let cay_code = r#"
            TreeDirectorySet SmallTreeDir {
                Names: r"test_1",
                Structure: {
                    layers: {
                        A: Directory,
                        B: Directory
                    },
                    edges: {
                        F: File
                    }
                }
            }

            fold "<PATH>": SmallTreeDir {
                A { name as a } => {
                    B { name as b } => {
                        F {name as f} => `{a}_{b}_{f}`
                    }
                }
            }
        "#;

        let in_structure =
            FileNode::dir("a", vec![FileNode::dir("b", vec![FileNode::file("f.txt")])]);
        let out_structure = FileNode::file_content("a_b_f.txt", "f.txt");

        system_test(cay_code, &in_structure, &out_structure);
    }

    #[test]
    fn system_shuffle_flatten() {
        let cay_code = r#"
            TreeDirectorySet SmallTreeDir {
                Names: r"test_1",
                Structure: {
                    layers: {
                        A: Directory,
                        B: Directory
                    },
                    edges: {
                        F: File
                    }
                }
            }

            fold "<PATH>": SmallTreeDir {
                A { name as a } => {
                    B { name as b } => {
                        F {name as f} => `{b}_{a}_{f}`
                    }
                }
            }
        "#;

        let in_structure =
            FileNode::dir("a", vec![FileNode::dir("b", vec![FileNode::file("f.txt")])]);
        let out_structure = FileNode::file_content("b_a_f.txt", "f.txt");

        system_test(cay_code, &in_structure, &out_structure);
    }
}
