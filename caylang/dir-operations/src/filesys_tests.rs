extern crate dir_diff;
extern crate fs_extra;
extern crate tempdir;

#[cfg(test)]
mod tests {
    use crate::filesys::run_test;
    use crate::tree::Tree;
    use std::path::PathBuf;

    #[test]
    fn basic_flatten() {
        run_test("templates/test1", |tree| {
            Tree::from_fold_function(&tree, |x| {
                let string = x.data.path.to_str().unwrap();
                PathBuf::from(str::replace(string, "/", "_"))
            })
        });
    }

    #[test]
    fn basic_shuffle() {
        run_test("templates/basic_shuffle", |tree| {
            Tree::from_fold_function(&tree, |x| {
                let mut components: Vec<&str> = x.data.path.to_str().unwrap().split('/').collect();
                components.swap(0, 1);
                PathBuf::from(components.join("/"))
            })
        });
    }
}
