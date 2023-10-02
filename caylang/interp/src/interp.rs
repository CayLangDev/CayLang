use crate::from_ast::{FoldOperation, Rename, Matches};
use crate::defn_map::{DefnMap, new_defn_map};
use caylang_parser::ast::{Expr, TreePrototype};
use caylang_io::filesys::load_full_tree;
use caylang_io::tree::{root_idx, Node, NodeData, NodeIdx, Tree};
use regex::Regex;
use std::collections::VecDeque;
use std::env;
use std::iter::zip;
use std::path::Path;
use std::path::PathBuf;

// use caylang_io::tree::NodeData;


// needs to access prototypes by identifiers from a defn_map
// fn validate(tree: &Tree, prototype: TreePrototype) -> bool {
//     for (l, (_, p_l)) in zip(tree.data_iter(tree.layers()), prototype.layers) {
//
//         if !p_l.matches(l) {
//             return false;
//         }
//     }
//     return true;
//     // don't verify files idgaf'
// }

pub fn to_fold(tree: &Tree, fold_desc: FoldOperation) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut old_paths = vec![];
    let mut new_paths = vec![];
    for l in tree.data_iter(tree.leaves()) {
        for ((_, o), t) in zip(&fold_desc.options, &fold_desc.targets) {
            if o.matches(l) {
                old_paths.push(l.path.clone());
                new_paths.push(new_name(&l.path, t.to_vec()));
            }
        }
    }
    return (old_paths, new_paths);
}

pub fn make_full_path<'a>(i: impl Iterator<Item = &'a Path> + 'a) -> PathBuf {
    let mut b = PathBuf::from("");
    for p in i {
        b.push(p);
    }
    return b;
}

pub fn new_name(path: &PathBuf, target: Rename) -> PathBuf {
    // let comps = path.components().collect();
    let mut target_q = VecDeque::from(target);
    let mut name_comps = vec![];
    for (i, comp) in path.components().enumerate() {
        let c = target_q.front();
        if let Some(j) = c {
            if &i == j {
                name_comps.push(comp);
                target_q.pop_front();
            }
        } else {
            break;
        }
    }
    return make_full_path(name_comps.iter().map(|c| c.as_ref()));
}

pub fn interpret(ast: Expr) {
    let mut defn_map = new_defn_map();
    let operations = defn_map.load_objects(ast);
    defn_map.add_defaults();
    println!("defns {:?}", defn_map);
    println!("operations {:?}", operations);
    return;
}
