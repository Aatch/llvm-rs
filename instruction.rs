use super::*;
use value::{Val,Metadata,Value};

use ffi::core::{ValueRef};

use ffi::core::instruction::*;

pub trait InstrVal<T:ty::Ty> : Val<T> { }

pub trait InstrImp {
    pub fn has_metadata(&self) -> bool;
    pub fn get_metadata(&self, kind: ffi::core::Metadata) -> Metadata;
    pub fn set_metadata(&self, kind: ffi::core::Metadata, md: Metadata);

    //pub fn parent(&self) -> BasicBlock;

    pub fn erase(&self);

    pub fn get_icmp_predicate(&self) -> ffi::core::IntPredicate;

    //pub fn get_switch_default(&self) -> BasicBlock;
}

pub trait CallInstVal<T:ty::Ty> : InstrVal<T> {
    pub fn set_callconv(&self, cc: uint);
    pub fn get_callconv(&self) -> uint;

    pub fn add_attribute(&mut self, idx: uint, attr: ffi::core::Attribute);
    pub fn remove_attribute(&mut self, idx: uint, attr: ffi::core::Attribute);

    pub fn set_alignment(&mut self, idx: uint, align: uint);

    pub fn is_tail_call(&self) -> bool;
    pub fn set_tail_call(&self, tail_call: bool);
}

pub trait PhiNodeVal<T:ty::Ty> : InstrVal<T> {
    //pub fn add_incoming(&self, &[(value::Value<T>, BasicBlock)]);

    pub fn count_incoming(&self) -> uint;

    //pub fn get_incoming_block(&self, idx: uint) -> BasicBlock;
    pub fn get_incoming_value(&self, idx: uint) -> Value<T>;
}

pub struct Instruction<T> {
    priv r: ffi::core::ValueRef
}

impl<T:ty::Ty> Wrapper<ValueRef> for Instruction<T> {
    pub fn from_ref(R: ValueRef) -> Instruction<T> {
        Instruction {
            r: R
        }
    }

    pub fn to_ref(&self) -> ValueRef {
        self.r
    }
}

impl<T:ty::Ty> InstrVal<T> for Instruction<T> { }

pub struct SwitchInstr {
    priv r: ffi::core::ValueRef
}

impl InstrVal<ty::Void> for SwitchInstr { }

impl Wrapper<ValueRef> for SwitchInstr {
    pub fn from_ref(R: ValueRef) -> SwitchInstr {
        SwitchInstr {
            r: R
        }
    }

    pub fn to_ref(&self) -> ValueRef {
        self.r
    }
}

pub struct LandingPad {
    priv r: ffi::core::ValueRef
}

impl InstrVal<ty::Void> for LandingPad { }

impl Wrapper<ValueRef> for LandingPad {
    pub fn from_ref(R: ValueRef) -> LandingPad {
        LandingPad {
            r: R
        }
    }

    pub fn to_ref(&self) -> ValueRef {
        self.r
    }
}
