//! Validate component core module,

use crate::defn_map::{DefnMap, TargetedLookupError};
use crate::from_ast::{Matches};
use caylang_parser::ast::{SuperIdent, Ident, NodePrototype, Prototype, StructureList, StructurePair, TreePrototype};
use caylang_io::tree::{NodeIdx, Tree};
use std::iter::zip;
use debug_print::{debug_println};

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    BadTreeDepth(usize, usize),             // expected a, found b
    IdentifiedPrototypeNotFound(SuperIdent),     // failed on a
    IdentifiedPrototypeNotSupported(SuperIdent), // failed on a
    IdentifiedParamPrototypeInvalid(SuperIdent), // failed on a
    LayerMatchFailed(SuperIdent, NodeIdx),       // failed to match a to b
    EdgeMatchFailed(NodeIdx),               // failed to match any edge prototype to a
}

pub(super) fn get_tree_prototype<'a>(d: &'a DefnMap, i: &SuperIdent) -> Result<TreePrototype, ValidationError> {
    match d.get_tree_object(i) {
        Ok(p) => Ok(p),
        Err(e) => Err(match e {
            TargetedLookupError::IncorrectTypeObjectFound => ValidationError::IdentifiedPrototypeNotSupported(i.clone()),
            TargetedLookupError::VariableNotFound |
                TargetedLookupError::IgnoreLookup =>
                    ValidationError::IdentifiedPrototypeNotFound(i.clone()),
            TargetedLookupError::InvalidParamIdent | TargetedLookupError::BadParameter =>
                ValidationError::IdentifiedParamPrototypeInvalid(i.clone())
        })
    }
}

pub(super) fn get_node_prototype<'a>(d: &'a DefnMap, i: &SuperIdent) -> Result<NodePrototype, ValidationError> {
    match d.get_node_object(i) {
        Ok(p) => Ok(p),
        Err(e) => Err(match e {
            TargetedLookupError::IncorrectTypeObjectFound => ValidationError::IdentifiedPrototypeNotSupported(i.clone()),
            TargetedLookupError::VariableNotFound | TargetedLookupError::IgnoreLookup => ValidationError::IdentifiedPrototypeNotFound(i.clone()),
            TargetedLookupError::InvalidParamIdent | TargetedLookupError::BadParameter =>
                ValidationError::IdentifiedParamPrototypeInvalid(i.clone())
        })
    }
}

pub(super) fn load_validation_prototypes<'a>(
    d: &'a DefnMap,
    l: &StructureList,
) -> Result<Vec<(SuperIdent, NodePrototype)>, ValidationError> {
    let mut prototypes = vec![];
    for StructurePair(_, prototype_idn) in l {
        let prototype = get_node_prototype(d, &prototype_idn)?;
        prototypes.push((prototype_idn.clone(), prototype));
    }
    return Ok(prototypes);
}

/// Validates the given tree's structure matches the prototype ident refers to
/// Will throw an error of type ValidationError indicating different error conditions,
/// i.e. ident doesn't refer to a tree prototype, the tree has the wrong depth
pub fn validate_tree(d: &DefnMap, tree: &Tree, ident: &SuperIdent) -> Result<(), ValidationError> {
    let tree_layers: Vec<Vec<NodeIdx>> = tree.proper_layers().collect();
    let prototype = get_tree_prototype(d, ident)?;
    let offset = if prototype.edges.len() > 0 { 1 } else { 0 };
    debug_println!("offset: {:?}, tree_layers len: {:?}", offset, tree_layers.len());

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
