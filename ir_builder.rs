use ffi::core::ir_builder::*;
use ffi::core::{BuilderRef,bb};
use instruction::{Instruction,SwitchInstr,LandingPad};
use value;
use value::{BasicBlock,Val};

use super::*;

use std::str;
use std::vec;

pub struct IRBuilder {
    priv r: BuilderRef
}

impl Wrapper<BuilderRef> for IRBuilder {
    pub fn from_ref(R: BuilderRef) -> IRBuilder {
        IRBuilder {
            r: R
        }
    }

    pub fn to_ref(&self) -> BuilderRef {
        self.r
    }
}

impl IRBuilder {
    pub fn new(c: Context) -> IRBuilder {
        unsafe {
            let r = LLVMCreateBuilderInContext(c.to_ref());
            Wrapper::from_ref(r)
        }
    }

    pub fn position<T:ty::Ty>(&mut self, block: BasicBlock, instr: Instruction<T>) {
        unsafe {
            LLVMPositionBuilder(self.r,
                                bb::LLVMValueAsBasicBlock(block.to_ref()),
                                instr.to_ref());
        }
    }

    pub fn position_before<T:ty::Ty>(&mut self, instr: Instruction<T>) {
        unsafe {
            LLVMPositionBuilderBefore(self.r, instr.to_ref());
        }
    }

    pub fn position_at_end(&mut self, block: BasicBlock) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.r, bb::LLVMValueAsBasicBlock(block.to_ref()));
        }
    }

    pub fn get_insert_block(&self) -> BasicBlock {
        unsafe {
            let r = LLVMGetInsertBlock(self.r);
            Wrapper::from_ref(
                bb::LLVMBasicBlockAsValue(r))
        }
    }

    pub fn clear_insertion_position(&self) {
        unsafe {
            LLVMClearInsertionPosition(self.r);
        }
    }

    pub fn insert<T:ty::Ty>(&mut self, instr: Instruction<T>) {
        unsafe {
            LLVMInsertIntoBuilder(self.r, instr.to_ref());
        }
    }

    pub fn insert_with_name<T:ty::Ty>(&mut self, instr: Instruction<T>, name: &str) {
        unsafe {
            do str::as_c_str(name) |s| {
                LLVMInsertIntoBuilderWithName(self.r, instr.to_ref(), s);
            }
        }
    }

    pub fn set_current_debug_location(&mut self, loc: value::Metadata) {
        unsafe {
            LLVMSetCurrentDebugLocation(self.r, loc.to_ref());
        }
    }

    pub fn get_current_debug_location(&self) -> value::Metadata {
        unsafe {
            let r = LLVMGetCurrentDebugLocation(self.r);
            Wrapper::from_ref(r)
        }
    }

    pub fn set_inst_debug_location<T:ty::Ty>(&mut self, inst: Instruction<T>) {
        unsafe {
            LLVMSetInstDebugLocation(self.r, inst.to_ref());
        }
    }

    pub fn ret_void(&mut self) -> Instruction<ty::Void> {
        unsafe {
            let r = LLVMBuildRetVoid(self.r);
            Wrapper::from_ref(r)
        }
    }

    pub fn ret<T:ty::Ty,V:value::Val<T>>(&mut self, val: V) -> Instruction<ty::Void> {
        unsafe {
            let r = LLVMBuildRet(self.r, val.to_ref());
            Wrapper::from_ref(r)
        }
    }

    pub fn aggregate_ret(&mut self, vals: &[value::Value<ty::Type>]) -> Instruction<ty::Void> {
        unsafe {
            let llvs = do vals.map |v| { v.to_ref() };
            let r = do vec::as_imm_buf(llvs) |b, len| {
                LLVMBuildAggregateRet(self.r, b, len as std::libc::c_uint)
            };

            Wrapper::from_ref(r)
        }
    }

    pub fn br(&mut self, dest: BasicBlock) -> Instruction<ty::Void> {
        unsafe {
            let r = LLVMBuildBr(self.r,
                                bb::LLVMValueAsBasicBlock(dest.to_ref()));
            Wrapper::from_ref(r)
        }
    }

    pub fn cond_br<V:Val<ty::Integer>>(
        &mut self,
        cond: V,
        then: BasicBlock,
        els: BasicBlock) -> Instruction<ty::Void> {
        unsafe {
            let r = LLVMBuildCondBr(
                self.r,cond.to_ref(),
                bb::LLVMValueAsBasicBlock(then.to_ref()),
                bb::LLVMValueAsBasicBlock(els.to_ref()));
            Wrapper::from_ref(r)
        }
    }

    pub fn switch<V:Val<ty::Integer>>(
        &mut self,
        val: V,
        els: BasicBlock,
        num_cases: uint) -> SwitchInstr {
        unsafe {
            let r = LLVMBuildSwitch(self.r,
                                    val.to_ref(),
                                    bb::LLVMValueAsBasicBlock(els.to_ref()),
                                    num_cases as std::libc::c_uint);
            Wrapper::from_ref(r)
        }
    }

    pub fn indirect_br<T:ty::Ty,V:Val<ty::Pointer<T>>>(&self, addr: V, num_dests: uint) -> Instruction<ty::Void> {
        unsafe {
            let r = LLVMBuildIndirectBr(self.r, addr.to_ref(), num_dests as std::libc::c_uint);
            Wrapper::from_ref(r)
        }
    }

    pub fn invoke(&self,
                  fun: value::Function,
                  args: &[value::Param<ty::Type>],
                  then: BasicBlock,
                  catch: BasicBlock,
                  name: &str) -> Instruction<ty::Type> {
        unsafe {
            let llargs = do args.map |a| { a.to_ref() };
            let then = bb::LLVMValueAsBasicBlock(then.to_ref());
            let catch = bb::LLVMValueAsBasicBlock(catch.to_ref());

            let r = do vec::as_imm_buf(llargs) |b,len| {
                do str::as_c_str(name) |s| {
                    LLVMBuildInvoke(self.r, fun.to_ref(), b, len as std::libc::c_uint,
                                    then, catch, s)
                }
            };

            Wrapper::from_ref(r)
        }
    }

    pub fn landingpad<T:ty::Ty>(&self,
                                ty: T,
                                pers_fn: value::Function,
                                num_clauses: uint,
                                name: &str) -> LandingPad {
        unsafe {
            let r = do str::as_c_str(name) |s| {
                LLVMBuildLandingPad(self.r,
                                    ty.to_ref(),
                                    pers_fn.to_ref(),
                                    num_clauses as std::libc::c_uint, s)
            };

            Wrapper::from_ref(r)
        }
    }

    pub fn resume<T:ty::Ty,V:Val<T>>(&self, exn: V) -> Instruction<ty::Void> {
        unsafe {
            let r = LLVMBuildResume(self.r, exn.to_ref());
            Wrapper::from_ref(r)
        }
    }
}

impl Drop for IRBuilder {
    pub fn finalize(&self) {
        unsafe {
            LLVMDisposeBuilder(self.r);
        }
    }
}
