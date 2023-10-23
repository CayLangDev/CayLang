use crate::ast::{Ident, ParamIdent, SuperIdent, Literal, 
    TreePrototype, NodePrototype, NodeType, StructureList, StructurePair};

pub fn to_ident(name: &str) -> Ident {
    return Ident::Variable(name.to_string());
}

pub fn to_int_param_ident(name: &str, num: i32) -> ParamIdent {
    return ParamIdent {name: name.to_string(), param: Literal::Integer(num)};
}

pub fn to_str_param_ident(name: &str, string: &str) -> ParamIdent {
    return ParamIdent {name: name.to_string(), param: Literal::String(string.to_string())};
}

pub fn simple_tree_prototype(name: &str) -> TreePrototype {
    return TreePrototype {regex: name.to_string(),
                          layers: vec![],
                          edges: vec![] };
}

pub fn simple_node_prototype(
name: &str,
node_type: NodeType
) -> NodePrototype {
    return NodePrototype {regex: name.to_string(),
                          node_type };
}

pub fn ign_label_slist(idens: &[&str]) -> StructureList {
    idens.iter().map(|s| StructurePair(Ident::Ignored, SuperIdent::Ident(Ident::Variable(s.to_string())))).collect()
}

pub fn compl_tree_prototype(
name: &str,
layers: &[&str],
edges: &[&str],
) -> TreePrototype {
    return TreePrototype {regex: name.to_string(),
                          layers: ign_label_slist(layers),
                          edges: ign_label_slist(edges) };
}
