use super::*;
use value::{Val,Value,BasicBlock};

use ffi::core::{bb};
use ffi::core::{ValueRef,True,False};

use ffi::core::instruction::*;

use std::cast;

pub trait InstrVal<T:ty::Ty> : Val<T> { }

pub trait InstrImpl {
    pub fn has_metadata(&self) -> bool;
    pub fn get_metadata(&self, kind: ffi::core::Metadata) -> super::value::Metadata;
    pub fn set_metadata(&self, kind: ffi::core::Metadata, md: super::value::Metadata);

    pub fn parent(&self) -> BasicBlock;

    pub fn erase(&self);

    pub fn get_icmp_predicate(&self) -> ffi::core::IntPredicate;
}

pub trait PhiNodeVal<T:ty::Ty> : InstrVal<T> {
    pub fn add_incoming(&self, &[(Value<T>, BasicBlock)]);

    pub fn count_incoming(&self) -> uint;

    pub fn get_incoming_block(&self, idx: uint) -> BasicBlock;
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

impl<T:ty::Ty> Val<T> for Instruction<T> { }
impl<T:ty::Ty> InstrVal<T> for Instruction<T> { }

pub struct CallInst<T> {
    priv r: ffi::core::ValueRef
}

impl<T:ty::Ty> CallInst<T> {
    pub fn set_callconv(&self, cc: uint) {
        unsafe {
            LLVMSetInstructionCallConv(self.r, cc as std::libc::c_uint);
        }
    }

    pub fn get_callconv(&self) -> uint {
        unsafe {
            LLVMGetInstructionCallConv(self.r) as uint
        }
    }


    pub fn add_attribute(&mut self, idx: uint, attr: ffi::core::Attribute) {
        unsafe {
            LLVMAddInstrAttribute(self.r, idx as std::libc::c_uint,
                                  attr as std::libc::c_ulonglong);
        }
    }

    pub fn remove_attribute(&mut self, idx: uint, attr: ffi::core::Attribute) {
        unsafe {
            LLVMRemoveInstrAttribute(self.r, idx as std::libc::c_uint,
                                     attr as std::libc::c_ulonglong);
        }
    }


    pub fn set_alignment(&mut self, idx: uint, align: uint) {
        unsafe {
            LLVMSetInstrParamAlignment(self.r, idx as std::libc::c_uint,
                                       align as std::libc::c_uint);
        }
    }


    pub fn is_tail_call(&self) -> bool {
        unsafe {
            LLVMIsTailCall(self.r) == True
        }
    }

    pub fn set_tail_call(&self, tail_call: bool) {
        unsafe {
            let tc = if tail_call { True } else { False };
            LLVMSetTailCall(self.r, tc);
        }
    }

}

impl<T:ty::Ty> Wrapper<ValueRef> for CallInst<T> {
    pub fn from_ref(R: ValueRef) -> CallInst<T> {
        CallInst {
            r: R
        }
    }

    pub fn to_ref(&self) -> ValueRef {
        self.r
    }
}

impl<T:ty::Ty> Val<T> for CallInst<T> { }
impl<T:ty::Ty> InstrVal<T> for CallInst<T> { }

pub struct SwitchInstr {
    priv r: ffi::core::ValueRef
}

impl SwitchInstr {
    pub fn default_dest(&self) -> BasicBlock {
        unsafe {
            let r = LLVMGetSwitchDefaultDest(self.r);
            Wrapper::from_ref(bb::LLVMBasicBlockAsValue(r))
        }
    }
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

impl Val<ty::Void> for LandingPad { }
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

pub struct PhiNode<T> {
    priv r: ffi::core::ValueRef
}

impl<T:ty::Ty> Val<T> for PhiNode<T> { }
impl<T:ty::Ty> InstrVal<T> for PhiNode<T> { }

impl<T:ty::Ty> Wrapper<ValueRef> for PhiNode<T> {
    pub fn from_ref(R: ValueRef) -> PhiNode<T> {
        PhiNode {
            r: R
        }
    }

    pub fn to_ref(&self) -> ValueRef {
        self.r
    }
}

impl<T:ty::Ty, I:InstrVal<T>> InstrImpl for I {
    pub fn has_metadata(&self) -> bool {
        unsafe {
            LLVMHasMetadata(self.to_ref()) != 0
        }
    }

    pub fn get_metadata(&self, kind: ffi::core::Metadata) -> super::value::Metadata {
        unsafe {
            let r = LLVMGetMetadata(self.to_ref(), kind as std::libc::c_uint);
            Wrapper::from_ref(r)
        }
    }

    pub fn set_metadata(&self, kind: ffi::core::Metadata, md: super::value::Metadata) {
        unsafe {
            LLVMSetMetadata(self.to_ref(), kind as std::libc::c_uint, md.to_ref());
        }
    }


    pub fn parent(&self) -> BasicBlock {
        unsafe {
            let r = LLVMGetInstructionParent(self.to_ref());
            Wrapper::from_ref(bb::LLVMBasicBlockAsValue(r))
        }
    }


    pub fn erase(&self) {
        unsafe {
            LLVMInstructionEraseFromParent(self.to_ref())
        }
    }


    pub fn get_icmp_predicate(&self) -> ffi::core::IntPredicate {
        unsafe {
            let p = LLVMGetICmpPredicate(self.to_ref());
            cast::transmute(p)
        }
    }
}

