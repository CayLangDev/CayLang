pub mod cmd;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}