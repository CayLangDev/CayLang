use crate::defn_map::{DefnMap, TargetedLookupError};
use crate::from_ast::{Matches};
use caylang_parser::ast::{Ident, NodePrototype, Prototype, StructureList, StructurePair, TreePrototype};
use caylang_io::tree::{NodeIdx, Tree};
use std::iter::zip;
use std::mem::{Discriminant};

#[derive(Debug)]
pub enum ValidationError {
    BadTreeDepth(usize, usize),             // expected a, found b
    IdentifiedPrototypeNotFound(Ident),     // failed on a
    IdentifiedPrototypeNotSupported(Ident), // failed on a
    LayerMatchFailed(Ident, NodeIdx),       // failed to match a to b
    EdgeMatchFailed(NodeIdx),               // failed to match any edge prototype to a
}

pub(super) fn get_tree_prototype<'a>(d: &'a DefnMap, i: &Ident) -> Result<&'a TreePrototype, ValidationError> {
    match d.get_tree_object(i) {
        Ok(p) => Ok(p),
        Err(e) => Err(match e {
            TargetedLookupError::IncorrectTypeObjectFound => ValidationError::IdentifiedPrototypeNotSupported(i.clone()),
            TargetedLookupError::VariableNotFound |
                TargetedLookupError::IgnoreLookup =>
                    ValidationError::IdentifiedPrototypeNotFound(i.clone()),
        })
    }
}

pub(super) fn get_node_prototype<'a>(d: &'a DefnMap, i: &Ident) -> Result<&'a NodePrototype, ValidationError> {
    match d.get_node_object(i) {
        Ok(p) => Ok(p),
        Err(e) => Err(match e {
            TargetedLookupError::IncorrectTypeObjectFound => ValidationError::IdentifiedPrototypeNotSupported(i.clone()),
            TargetedLookupError::VariableNotFound | TargetedLookupError::IgnoreLookup => ValidationError::IdentifiedPrototypeNotFound(i.clone()),
        })
    }
}

pub(super) fn load_validation_prototypes<'a>(
    d: &'a DefnMap,
    l: &StructureList,
) -> Result<Vec<(Ident, &'a NodePrototype)>, ValidationError> {
    let mut prototypes = vec![];
    for StructurePair(_, prototype_idn) in l {
        let prototype = get_node_prototype(d, &prototype_idn)?;
        prototypes.push((prototype_idn.clone(), prototype));
    }
    return Ok(prototypes);
}

// needs to access prototypes by identifiers from a defn_map
pub fn validate_tree(d: &DefnMap, tree: &Tree, ident: &Ident) -> Result<(), ValidationError> {
    let tree_layers: Vec<Vec<NodeIdx>> = tree.proper_layers().collect();
    let prototype = get_tree_prototype(d, ident)?;
    let offset = if prototype.edges.len() > 0 { 1 } else { 0 };

    // There should be as many tree layers as there are prototype layers
    // unless the prototype has an edge, in which case there ought to be one more
    // tree layer.
    if tree_layers.len() != prototype.layers.len() + offset {
        return Err(ValidationError::BadTreeDepth(
            prototype.layers.len() + offset,
            tree_layers.len(),
        ));
    }

    let inner_layers = &tree_layers[0..tree_layers.len() - offset];
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
    for leaf_idx in &tree_layers[tree_layers.len() - 1] {
        let leaf_node = &tree.nodes[*leaf_idx];
        if !edge_prototypes
            .iter()
            .any(|(_, p)| p.matches(&leaf_node.data))
        {
            return Err(ValidationError::EdgeMatchFailed(*leaf_idx));
        }
    }

    return Ok(());
}
