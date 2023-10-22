#[cfg(test)]
mod tests {
    use caylang_io::test_helpers::system_testing::system_test;
    use caylang_io::test_helpers::tree_builder::FileNode;
    use caylang_io::tree::{NodeData, NodeType, Tree};
    use std::fs::File;
    use std::path::PathBuf;

    #[test]
    fn basic_system_shuffle() {
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
}
