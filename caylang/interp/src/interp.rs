use crate::defn_map::{new_defn_map, DefnMap};
use crate::from_ast::{FoldOperation, Rename, RenamePart, Matches, top_level_ident};
use caylang_parser::ast::{Expr, NodePrototype, TreePrototype, Prototype, StructureList, Ident, StructurePair};

use caylang_io::filesys::{load_full_tree, write_full_tree};
use caylang_io::tree::{Tree, NodeIdx};

use std::collections::VecDeque;

use std::iter::zip;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ValidationError {
    BadTreeDepth(usize, usize), // expected a, found b
    IdentifiedPrototypeNotFound(Ident), // failed on a
    IdentifiedPrototypeNotSupported(Ident), // failed on a
    LayerMatchFailed(Ident, NodeIdx), // failed to match a to b
    EdgeMatchFailed(NodeIdx) // failed to match any edge prototype to a
}

fn get_tree_prototype<'a>(
    d: &'a DefnMap,
    i: &Ident
) -> Result<&'a TreePrototype, ValidationError> {
    let Ok(prototype) = d.get_object(&i) else {
            return
                Err(
                    ValidationError::IdentifiedPrototypeNotFound(
                        i.clone()
                    )
                );
        };

    let Prototype::TreePrototype(prototype) = prototype else {
        return
            Err(
                ValidationError::IdentifiedPrototypeNotSupported(
                    i.clone()
                )
            );
    };
    return Ok(prototype);
}

fn get_node_prototype<'a>(
    d: &'a DefnMap,
    i: &Ident
) -> Result<&'a NodePrototype, ValidationError> {
    let Ok(prototype) = d.get_object(&i) else {
            return
                Err(
                    ValidationError::IdentifiedPrototypeNotFound(
                        i.clone()
                    )
                );
        };

    let Prototype::NodePrototype(prototype) = prototype else {
        return
            Err(
                ValidationError::IdentifiedPrototypeNotSupported(
                    i.clone()
                )
            );
    };
    return Ok(prototype);
}

fn load_validation_prototypes<'a>(
d: &'a DefnMap,
l: &StructureList
) -> Result<Vec<(Ident, &'a NodePrototype)>, ValidationError> {
    let mut prototypes = vec![];
    for StructurePair(_, prototype_idn) in l {
        let prototype = get_node_prototype(d, &prototype_idn)?;
        prototypes.push((prototype_idn.clone(), prototype));
    }
    return Ok(prototypes);
}

// needs to access prototypes by identifiers from a defn_map
fn validate_tree(
    d: &DefnMap,
    tree: &Tree,
    ident: &Ident
) -> Result<(), ValidationError> {
    let tree_layers: Vec<Vec<NodeIdx>> = tree.proper_layers().collect();
    let prototype = get_tree_prototype(d, ident)?;
    let offset = if prototype.edges.len() > 0 { 1 } else { 0 };

    // There should be as many tree layers as there are prototype layers
    // unless the prototype has an edge, in which case there ought to be one more
    // tree layer.
    if tree_layers.len() != prototype.layers.len() + offset {
        return Err(ValidationError::BadTreeDepth(prototype.layers.len() + offset,
                                                 tree_layers.len()));
    }

    let inner_layers = &tree_layers[0..tree_layers.len()-offset];
    let layer_prototypes = load_validation_prototypes(&d, &prototype.layers)?;

    for (layer, (l_ident, l_prototype)) in zip(inner_layers, layer_prototypes) {
        for node_idx in layer {
            let node = &tree.nodes[*node_idx];
            if !l_prototype.matches(&node.data) {
                return Err(ValidationError::LayerMatchFailed(l_ident, *node_idx));
            }
        }
    }

    if offset == 0 {
        return Ok(());
    }

    let edge_prototypes = load_validation_prototypes(&d, &prototype.edges)?;
    // return false if there's any leaf node that doesn't match some
    // edge prototype.
    for leaf_idx in &tree_layers[tree_layers.len()-1] {
        let leaf_node = &tree.nodes[*leaf_idx];
        if !edge_prototypes.iter().any(|(_, p)| p.matches(&leaf_node.data)) {
            return Err(ValidationError::EdgeMatchFailed(*leaf_idx));
        }
    }

    return Ok(());
}

pub fn to_fold(
    d: &DefnMap,
    tree: &Tree,
    fold_desc: FoldOperation
) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut old_paths = vec![];
    let mut new_paths = vec![];
    for l in tree.data_iter(tree.leaves()) {

        let prototypes = fold_desc.options.iter().map(|i| {
            let Ok(Prototype::NodePrototype(o)) = d.get_object(i)
            else {panic!("Edge Prototype doesn't exist")};
            o
        });

        let out = zip(prototypes,fold_desc.targets.iter()).find(|(p, t)| p.matches(l));

        match out {
            Some((_, t)) => {
                old_paths.push(l.path.clone());
                new_paths.push(new_name(&l.path, t));
            },
            _ => {
                panic!("Edge Prototypes didn't match!");
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

pub fn interpret(ast: Expr) {
    let mut defn_map = new_defn_map();
    let operations = defn_map.load_objects(ast);
    defn_map.add_defaults();
    println!("defns {:?}", defn_map);
    println!("operations {:?}", operations);
    // now run to_fold
    for op in operations {
        let root: PathBuf = op.from.into();
        let tree = load_full_tree(&root);
        let tlp_ident = op.structure.top_level;
        validate_tree(&defn_map, &tree, &tlp_ident).unwrap();

        // tree.print();
        let (old_paths, new_paths) = to_fold(&defn_map, &tree, op.operation);
        println!("old paths: {:?}", old_paths);
        println!("new paths: {:?}", new_paths);
        let new_tree = Tree::from_fold(&tree, old_paths, new_paths);
        write_full_tree(&root, &root, &tree, &new_tree);
    }
    return;
}
