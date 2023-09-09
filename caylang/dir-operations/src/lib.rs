extern crate tempdir;
extern crate dir_diff;
extern crate fs_extra;

pub mod tree;
pub mod filesys;

#[cfg(test)]
mod tests {
    use crate::filesys::{load_full_tree, write_full_tree, copy_directory, run_test};

    #[test]
    fn flatten_test() {
        run_test("test/templates/test1", |tree| Tree::from_fold_function(&tree, |x| {
            let string = x.data.path.to_str().unwrap();
            PathBuf::from(str::replace(string, "/", "_"))
        }));
    }
}
