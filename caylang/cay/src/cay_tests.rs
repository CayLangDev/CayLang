#[cfg(test)]
mod tests {
    use caylang_io::filesys::run_system_test;
    use caylang_io::filesys_builder::FileNode;
    use caylang_io::tree::{NodeData, NodeType, Tree};
    use std::fs::File;
    use std::path::PathBuf;

    #[test]
    fn basic_system_shuffle() {
        let in_structure = FileNode::dir("a", vec![FileNode::dir("b", [FileNode::file("f")])]);

        run_system_test("test/cases/simple_test_1");
    }
}
