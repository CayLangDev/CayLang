use caylang_parser::ast::{Ident, TreePrototype, NodePrototype, NodeType};

pub(crate) fn to_ident(name: &str) -> Ident {
    return Ident::Variable(name.to_string());
}

pub(crate) fn simple_tree_prototype(name: &str) -> TreePrototype {
    return TreePrototype {regex: name.to_string(),
                          layers: vec![],
                          edges: vec![] };
}

pub(crate) fn simple_node_prototype(
name: &str,
node_type: NodeType
) -> NodePrototype {
    return NodePrototype {regex: name.to_string(),
                          node_type };
}
