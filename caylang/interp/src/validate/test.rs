use crate::validate::{get_tree_prototype, get_node_prototype, ValidationError};
use crate::defn_map::{DefnMap, new_defn_map};
use crate::test_helpers::{to_ident,
                          simple_tree_prototype,
                          simple_node_prototype};
use caylang_parser::ast::{Prototype, TreePrototype,
                          NodePrototype, NodeType};



#[test]
fn test_get_tree_prototype() {
    let mut d = new_defn_map();
    let p = simple_tree_prototype("a");
    _ = d.add_object(
        to_ident("a"),
        Prototype::TreePrototype(p)
    );

    let p = simple_tree_prototype("a");
    assert_eq!(get_tree_prototype(&d, &to_ident("a")), Ok(&p) as Result<&TreePrototype, ValidationError>);

    let b = to_ident("b");
    let res = get_tree_prototype(&d, &b);
    assert_eq!(res, Err(ValidationError::IdentifiedPrototypeNotFound(b)));

    let i = to_ident("_");
    let res = get_tree_prototype(&d, &i);
    assert_eq!(res, Err(ValidationError::IdentifiedPrototypeNotFound(i)));
}

#[test]
fn test_get_node_prototype() {
    let mut d = new_defn_map();
    let p = simple_node_prototype("a", NodeType::File);
    _ = d.add_object(
        to_ident("a"),
        Prototype::NodePrototype(p)
    );

    let p = simple_node_prototype("a", NodeType::File);
    assert_eq!(get_node_prototype(&d, &to_ident("a")),
               Ok(&p) as Result<&NodePrototype, ValidationError>);

    let b = to_ident("b");
    let res = get_node_prototype(&d, &b);
    assert_eq!(res, Err(ValidationError::IdentifiedPrototypeNotFound(b)));

    let i = to_ident("_");
    let res = get_node_prototype(&d, &i);
    assert_eq!(res, Err(ValidationError::IdentifiedPrototypeNotFound(i)));
}
