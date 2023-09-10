extern crate dir_diff;
extern crate fs_extra;
extern crate tempdir;

pub mod filesys;
pub mod tree;

#[cfg(test)]
mod tests {
    use crate::filesys::run_test;
    use crate::tree::Tree;
    use std::path::PathBuf;

    #[test]
    fn flatten_test() {
        // TODO consider modifying internal path storage to be relative to tree root rather than call location. Would avoid this magic depth number
        run_test("templates/test1", |tree| {
            Tree::from_fold_function(&tree, |x| {
                let string = x.data.path.to_str().unwrap();
                let tmp_string = str::replace(string, "/", "_");
                PathBuf::from(&str::replacen(&tmp_string, "_", "/", 4))
            })
        });
    }
}
