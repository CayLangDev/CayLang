extern crate dir_diff;
extern crate fs_extra;
extern crate tempdir;

#[cfg(test)]
mod tests {
    use crate::filesys::run_test;
    use crate::tree::Tree;
    use std::path::PathBuf;

    #[test]
    fn flatten_test() {
        run_test("templates/test1", |tree| {
            Tree::from_fold_function(&tree, |x| {
                let string = x.data.path.to_str().unwrap();
                PathBuf::from(str::replace(string, "/", "_"))
            })
        });
    }
}