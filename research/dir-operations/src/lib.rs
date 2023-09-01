extern crate tempdir;

pub mod tree;
pub mod filesys;
pub mod vfs_dir_tests;


#[cfg(test)]
mod tests {
    use file_diff::{diff};
    use std::fs::File;
    use std::io::{self, Write};
    use tempdir::TempDir;

    use jwalk::{WalkDir};
    use std::env;
    use std::path::PathBuf;
    use crate::tree::{Tree, NodeIdx, root_idx, NodeType};
    use crate::filesys::{load_full_tree, write_full_tree, copy_directory};

    #[test]
    fn flatten_test() {
        let test_path = PathBuf::from("test/templates/test1/");
        let mut in_path = test_path.clone();
        in_path.push("in");
        let mut out_path = test_path.clone();
        out_path.push("out");

        let tmp_dir = TempDir::new("test").unwrap();
        let root_path = PathBuf::from(tmp_dir.path());

        copy_directory(&root_path, &in_path);

        let tree: Tree = load_full_tree(root_path.clone());

        let new_tree = Tree::from_fold_function(&tree, |x| {
            let string = x.data.path.to_str().unwrap();
            PathBuf::from(str::replace(string, "/", "_"))
        });

        write_full_tree(&tree, &new_tree);

        diff(out_path.to_str().unwrap(), root_path.to_str().unwrap());
        
        // Assert diff is null
    }
}
