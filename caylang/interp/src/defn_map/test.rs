use caylang_parser::ast::{SuperIdent, Ident, StructurePair, TreePrototype, NodePrototype, Prototype, NodeType};
use crate::defn_map::{new_defn_map, DefnMap};
use caylang_parser::test_helpers::{to_ident, to_regex_param_ident, to_int_param_ident};

#[test]
fn test_new() {
    let d = new_defn_map();
    assert!(d.data.len() == 0);
}

#[test]
fn test_default_dir() {
    let mut d = new_defn_map();
    d.add_defaults();
    let dir = d.get_object(&SuperIdent::Ident(to_ident("Directory")));
    assert!(matches!(&dir, Ok(Prototype::NodePrototype(p))));
    match dir {
        Ok(Prototype::NodePrototype(p)) => {
            assert!(p.regex == r".*".to_string());
            assert!(matches!(p.node_type, NodeType::Dir));
        }
        _ => unreachable!()
    }
}

#[test]
fn test_default_file() {
    let mut d = new_defn_map();
    d.add_defaults();
    let file = d.get_object(&SuperIdent::Ident(to_ident("File")));
    assert!(matches!(&file, Ok(Prototype::NodePrototype(p))));
    match file {
        Ok(Prototype::NodePrototype(p)) => {
            assert!(p.regex == r".*".to_string());
            assert!(matches!(p.node_type, NodeType::File));
        }
        _ => unreachable!()
    }
}

#[test]
fn test_add_file() {
//
}

#[test]
fn test_param_dir() {
    let mut d = new_defn_map();    
    let dir = d.get_object(&SuperIdent::ParamIdent(to_regex_param_ident("Directory", r"test")));
    assert!(matches!(&dir, Ok(Prototype::NodePrototype(p))));
    match dir {
        Ok(Prototype::NodePrototype(p)) => {
            assert!(p.regex == r"test".to_string());
            assert!(matches!(p.node_type, NodeType::Dir));
        }
        _ => unreachable!()
    }
}

#[test]
fn test_param_tree_dir() {
    let mut d = new_defn_map();
    
    let star = TreePrototype {
        regex: r".*".to_string(),
        layers: vec![StructurePair(Ident::Ignored, SuperIdent::Ident(to_ident("Directory"))),
                     StructurePair(Ident::Ignored, SuperIdent::Ident(to_ident("Directory")))],
        edges: vec![StructurePair(Ident::Ignored, SuperIdent::Ident(to_ident("File")))]
    };

    let dir = d.get_object(&SuperIdent::ParamIdent(to_int_param_ident("Star", 3)));
    assert!(matches!(&dir, Ok(Prototype::TreePrototype(p))));
    match dir {
        Ok(Prototype::TreePrototype(t)) => {
            assert_eq!(&star, &t)
        }
        _ => unreachable!()
    }
}
