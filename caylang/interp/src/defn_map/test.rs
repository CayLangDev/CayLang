use caylang_parser::ast::{Ident, NodePrototype, Prototype, NodeType};
use crate::defn_map::*;
use crate::test_helpers::as_ident;

#[test]
fn test_new() {
    let d = new_defn_map();
    assert!(d.data.len() == 0);
}

#[test]
fn test_default_dir() {
    let mut d = new_defn_map();
    d.add_defaults();
    let dir = d.get_object(&to_ident("Directory"));
    assert!(matches!(dir, Ok(Prototype::NodePrototype(p))));
    match dir {
        Ok(Prototype::NodePrototype(p)) => {
            assert!(p.regex == r".*".to_string());
            assert!(matches!(p.node_type, NodeType::Dir));
        }
        _ => unreachable!()
    }
}
