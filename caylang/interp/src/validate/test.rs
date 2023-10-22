use crate::validate::*;
use crate::defn_map::{DefnMap, new_defn_map};
use crate::test_helpers::{to_ident,
                          simple_tree_prototype,
                          simple_node_prototype};
use caylang_parser::ast::{Prototype, TreePrototype,
                          NodePrototype, NodeType};



#[test]
fn test_get_tree_prototype() {
    let mut d = new_defn_map();
    _ = d.add_object(
        to_ident("a"),
        Prototype::TreePrototype(simple_tree_prototype("a"))
    );

}
