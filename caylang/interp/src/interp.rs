use crate::from_ast::{FoldOperation, Rename, Matches};
use crate::defn_map::{new_defn_map, DefnMap};
use caylang_parser::ast::{Expr, Prototype, NodePrototype};

use caylang_io::tree::{Tree};
use caylang_io::filesys::{load_full_tree};

use std::collections::VecDeque;

use std::iter::zip;
use std::path::Path;
use std::path::PathBuf;
use std::path::Component;

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

pub fn to_fold(d: &DefnMap, tree: &Tree, fold_desc: FoldOperation, root_len: usize) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut old_paths = vec![];
    let mut new_paths = vec![];
    for l in tree.data_iter(tree.leaves()) {
        for (i, t) in zip(&fold_desc.options, &fold_desc.targets) {
            let o = d.get_object(i);
            match o {
                Ok(Prototype::NodePrototype(o)) => {
                    if o.matches(l) {
                        old_paths.push(l.path.clone());
                        new_paths.push(new_name(&l.path, t.to_vec(), root_len));
                    }
                }
                _ => {}
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

pub fn new_name(path: &PathBuf, target: Rename, root_len: usize) -> PathBuf {
    // let comps = path.components().collect();
    // let mut target_q = VecDeque::from(target);
    // let mut name_comps = vec![];
    // for (i, comp) in path.components().enumerate() {
    //     let c = target_q.front();
    //     if let Some(j) = c {
    //         if &i == j {
    //             name_comps.push(comp);
    //             target_q.pop_front();
    //         }
    //     } else {
    //         break;
    //     }
    // }
    // return make_full_path(name_comps.iter().map(|c| c.as_ref()));
    // println!("{:?}", path);
    let comps: Vec<Component> = path.components().collect();
    // let m = vec![];
    // for c in comps {
        // m.push(c);
    // }

    let mut name_comps = vec![];
    for i in    0..root_len {
        name_comps.push(comps[i]);
    }

    for i in target {
        name_comps.push(comps[root_len+i]);
    }
    return make_full_path(name_comps.iter().map(|c| c.as_ref()));
}

pub fn interpret(ast: Expr) {
    let mut defn_map = new_defn_map();
    let operations = defn_map.load_objects(ast);
    defn_map.add_defaults();
    println!("defns {:?}", defn_map);
    println!("operations {:?}", operations);
    // now run to_fold
    for op in operations {
        let root: PathBuf = op.from.into();
        let root_len = root.components().count();
        let tree = load_full_tree(root);
        let (old_paths, new_paths) = to_fold(&defn_map, &tree, op.operation, root_len);
        println!("old paths: {:?}", old_paths);
        println!("new paths: {:?}", new_paths);
    }
    return;
}