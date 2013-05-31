use ffi::core::*;

pub trait Use {}
pub trait Value {
    fn get_name(&self) -> ~str;
    fn set_name(&self, s: &str);
    fn dump(&self);
    fn is_constant(&self) -> bool;
    fn is_undef(&self) -> bool;
}

pub trait Constant      : Value {}
pub trait Metadata      : Value {}
pub trait BasicBlock    : Value {}
pub trait Instruction   : Value {}

pub trait Global        : Constant {}
pub trait Function      : Global {}

pub trait ConstExpr     : Constant {}

pub trait CallInst      : Instruction {}
pub trait PHINode       : Instruction {}


