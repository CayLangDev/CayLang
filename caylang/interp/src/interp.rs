use crate::defn_map::{new_defn_map, DefnMap};
use crate::from_ast::{FoldOperation, Matches, Rename, RenamePart};
use crate::validate::{validate_tree};
use caylang_parser::ast::{Expr, Prototype};

use caylang_io::filesys::{load_full_tree, write_full_tree};
use caylang_io::tree::{Tree};


use std::iter::zip;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use debug_print::{debug_println};

/// Calculate the old paths and new paths of each leaf in the given tree based on the given FoldOperation
/// Uses given DefnMap to retrieve the leaf node prototypes described in the fold operation
/// Checks each leaf node of the tree matches some leaf node prototype and its new path based on that leaf node's rename described in FoldOperation
pub fn to_fold(d: &DefnMap, tree: &Tree, fold_desc: FoldOperation) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut old_paths = vec![];
    let mut new_paths = vec![];
    for l in tree.data_iter(tree.leaves()) {
        let prototypes = fold_desc.options.iter().map(|i| {
            let Ok(Prototype::NodePrototype(o)) = d.get_object(i)
            else {panic!("Edge Prototype doesn't exist")};
            o
        });

        let out = zip(prototypes, fold_desc.targets.iter()).find(|(p, t)| p.matches(l));

        match out {
            Some((_, t)) => {
                old_paths.push(l.path.clone());
                new_paths.push(new_name(&l.path, t));
            }
            _ => {
                panic!("Edge Prototypes didn't match!");
            }
        }
    }
    return (old_paths, new_paths);
}

/// Makes a full path from path components
pub fn make_full_path<'a>(i: impl Iterator<Item = &'a Path> + 'a) -> PathBuf {
    let mut b = PathBuf::from("");
    for p in i {
        b.push(p);
    }
    return b;
}

/// Transforms a path based on a rename description
pub fn new_name(path: &PathBuf, target: &Rename) -> PathBuf {
    let comps: Vec<Component> = path.components().collect();

    let mut name_comps = vec![];

    for part in &target.parts {
        let mut s = "".to_string();
        for subpart in part {
            match subpart {
                RenamePart::Text(t) => s += &t,
                RenamePart::Idx(i) => s += comps[*i].as_os_str().to_str().unwrap(),
            }
        }
        name_comps.push(s);
    }
    return make_full_path(name_comps.iter().map(|c| c.as_ref()));
}

/// Interpret the given AST
pub fn interpret(ast: Expr) {
    let mut defn_map = new_defn_map();
    let operations = defn_map.load_objects(ast);
    defn_map.add_defaults();
    for op in operations {
        let root: PathBuf = op.from.into();
        let tree = load_full_tree(&root);
        let tlp_ident = op.structure.top_level;
        #[cfg(debug_assertions)] tree.print(); // print tree if debugging
        validate_tree(&defn_map, &tree, &tlp_ident).unwrap();

        // tree.print();
        let (old_paths, new_paths) = to_fold(&defn_map, &tree, op.operation);
        debug_println!("old paths: {:?}", old_paths);
        debug_println!("new paths: {:?}", new_paths);
        let new_tree = Tree::from_fold(&tree, old_paths, new_paths);
        write_full_tree(&root, &root, &tree, &new_tree);
    }
    return;
}
