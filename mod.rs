#[link(
    name = "rust-llvm",
    vers = "0.1"
    )];
#[crate_type="lib"];

use ffi::core::*;

pub mod ffi {
    pub mod core;
    pub mod exec_engine;
    pub mod target_machine;
}

pub struct Context {
    priv r: ContextRef
}

pub struct Module {
    priv r: ModuleRef
}

