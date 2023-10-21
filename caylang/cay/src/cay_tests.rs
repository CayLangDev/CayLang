#[cfg(test)]
mod tests {
    use caylang_io::filesys::run_system_test;
    use caylang_io::tree::{NodeData, NodeType, Tree};
    use std::path::PathBuf;

    #[test]
    fn basic_system_shuffle() {
        run_system_test("test/cases/simple_test_1");
    }
}
