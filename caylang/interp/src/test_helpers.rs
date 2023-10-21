use caylang_parser::{Ident, TreePrototype, NodePrototype};

pub(crate) fn to_ident(name: &str) -> Ident {
    return Ident::Variable(name.to_string());
}

pub(crate) fn simple_tree_prototype(name: &str) -> TreePrototype {
    return TreePrototype {name: name.to_string(),
                          layers: vec![],
                          edges: vec![] };
}

pub(crate) fn simple_node_prototype(
name: &str,
node_type: NodeType
) -> NodePrototype {
    return NodePrototype {name: name.to_string(),
                          node_type };
}
