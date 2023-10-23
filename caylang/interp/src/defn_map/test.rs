use caylang_parser::ast::{SuperIdent, Ident, NodePrototype, Prototype, NodeType};
use crate::defn_map::{new_defn_map, DefnMap};
use caylang_parser::test_helpers::{to_ident, to_str_param_ident};

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

fn test_param_dir() {
    let mut d = new_defn_map();    
    let dir = d.get_object(&SuperIdent::ParamIdent(to_str_param_ident("Directory", "r\"test\"")));
    assert!(matches!(&dir, Ok(Prototype::NodePrototype(p))));
    match dir {
        Ok(Prototype::NodePrototype(p)) => {
            assert!(p.regex == r"test".to_string());
            assert!(matches!(p.node_type, NodeType::Dir));
        }
        _ => unreachable!()
    }
}