use ffi::core::ir_builder::*;
use ffi::core::{BuilderRef,bb,IntPredicate,RealPredicate};
use instruction::{Instruction,SwitchInstr,LandingPad,PhiNode,CallInst};
use value;
use value::{BasicBlock,Val,Global,Param};

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
    pub fn new(c: &Context) -> IRBuilder {
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

    pub fn unreachable(&self) -> Instruction<ty::Void> {
        unsafe {
            let r = LLVMBuildUnreachable(self.r);
            Wrapper::from_ref(r)
        }
    }

    pub fn add<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildAdd(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn nsw_add<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildNSWAdd(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn nuw_add<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildNUWAdd(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn fadd<V1:Val<ty::Real>,V2:Val<ty::Real>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<ty::Real> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildFAdd(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn sub<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildSub(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn nsw_sub<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildNSWSub(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn nuw_sub<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildNUWSub(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn fsub<V1:Val<ty::Real>,V2:Val<ty::Real>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<ty::Real> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildFSub(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn mul<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildMul(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn nsw_mul<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildNSWMul(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn nuw_mul<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildNUWMul(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn fmul<V1:Val<ty::Real>,V2:Val<ty::Real>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<ty::Real> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildFMul(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn udiv<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildUDiv(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn sdiv<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildSDiv(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn sdiv_exact<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildExactSDiv(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn fdiv<V1:Val<ty::Real>,V2:Val<ty::Real>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<ty::Real> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildFDiv(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn urem<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildURem(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn srem<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildURem(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn frem<V1:Val<ty::Real>,V2:Val<ty::Real>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<ty::Real> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildFRem(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn shl<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildShl(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn lshr<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildLShr(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn ashr<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildAShr(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn or<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildOr(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn xor<T:ty::Ty,V1:Val<T>,V2:Val<T>>(
        &self, lhs: V1, rhs: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildXor(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn neg<T:ty::Ty,V:Val<T>>(&self, val: V, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildNeg(self.r, val.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn nsw_neg<T:ty::Ty,V:Val<T>>(&self, val: V, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildNSWNeg(self.r, val.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn nuw_neg<T:ty::Ty,V:Val<T>>(&self, val: V, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildNUWNeg(self.r, val.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn fneg<V:Val<ty::Real>>(&self, val: V, name: &str) -> Instruction<ty::Real> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildFNeg(self.r, val.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn not<T:ty::Ty,V:Val<T>>(&self, val: V, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildNot(self.r, val.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn malloc<T:ty::Ty>(&self, ty: T, name: &str) -> Instruction<ty::Pointer<T>> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildMalloc(self.r, ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn array_malloc<T:ty::Ty,I:Val<ty::Integer>>(
        self, ty: T, size: I, name: &str) -> Instruction<ty::Pointer<T>> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildArrayMalloc(self.r, ty.to_ref(), size.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn alloca<T:ty::Ty>(&self, ty: T, name: &str) -> Instruction<ty::Pointer<T>> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildAlloca(self.r, ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn array_alloca<T:ty::Ty,I:Val<ty::Integer>>(
        &self, ty: T, size: I, name: &str) -> Instruction<ty::Pointer<T>> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildArrayAlloca(self.r, ty.to_ref(), size.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn free<T:ty::Ty,VP:Val<ty::Pointer<T>>>(&self, v: VP) -> Instruction<ty::Void> {
        unsafe {
            let r = LLVMBuildFree(self.r, v.to_ref());
            Wrapper::from_ref(r)
        }
    }

    pub fn load<T:ty::Ty,V:Val<ty::Pointer<T>>>(&self, v: V, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildLoad(self.r, v.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn store<T:ty::Ty,V:Val<T>,VP:Val<ty::Pointer<T>>>(
        &self, val: V, ptr: VP) -> Instruction<ty::Void> {
        unsafe {
            let r = LLVMBuildStore(self.r, val.to_ref(), ptr.to_ref());
            Wrapper::from_ref(r)
        }
    }

    pub fn gep<T:ty::Ty,VP:Val<ty::Pointer<T>>,VI:Val<ty::Integer>>(
        &self, ptr: VP, idxs: &[VI], name: &str) -> Instruction<ty::Pointer<ty::Type>> {
        unsafe {
            let llis = do idxs.map |i| { i.to_ref() };
            do str::as_c_str(name) |s| {
                do vec::as_imm_buf(llis) |b, len| {
                    let r = LLVMBuildGEP(self.r, ptr.to_ref(),
                                         b, len as std::libc::c_uint, s);
                    Wrapper::from_ref(r)
                }
            }
        }
    }

    pub fn inbounds_gep<T:ty::Ty,VP:Val<ty::Pointer<T>>,VI:Val<ty::Integer>>(
        &self, ptr: VP, idxs: &[VI], name: &str) -> Instruction<ty::Pointer<ty::Type>> {
        unsafe {
            let llis = do idxs.map |i| { i.to_ref() };
            do str::as_c_str(name) |s| {
                do vec::as_imm_buf(llis) |b, len| {
                    let r = LLVMBuildInBoundsGEP(self.r, ptr.to_ref(),
                                         b, len as std::libc::c_uint, s);
                    Wrapper::from_ref(r)
                }
            }
        }
    }

    pub fn struct_gep<VP:Val<ty::Struct>>(&self, ptr: VP, idx: uint, name: &str)
                    -> Instruction<ty::Type> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildStructGEP(self.r, ptr.to_ref(), idx as std::libc::c_uint, s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn global_string(&self, string: &str, name: &str) -> Global<ty::Integer> {
        unsafe {
            do str::as_c_str(string) |d| {
                do str::as_c_str(name) |n| {
                    let r = LLVMBuildGlobalString(self.r, d, n);
                    Wrapper::from_ref(r)
                }
            }
        }
    }

    pub fn global_string(&self, string: &str, name: &str) -> Instruction<ty::Pointer<ty::Integer>> {
        unsafe {
            do str::as_c_str(string) |d| {
                do str::as_c_str(name) |n| {
                    let r = LLVMBuildGlobalStringPtr(self.r, d, n);
                    Wrapper::from_ref(r)
                }
            }
        }
    }

    pub fn trunc<T1:ty::Ty,T2:ty::Ty,V:Val<T1>>(
        &self, val: V, ty: T2, name: &str) -> Instruction<T2> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildTrunc(self.r, val.to_ref(), ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn zext<T1:ty::Ty,T2:ty::Ty,V:Val<T1>>(
        &self, val: V, ty: T2, name: &str) -> Instruction<T2> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildZExt(self.r, val.to_ref(), ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn sext<T1:ty::Ty,T2:ty::Ty,V:Val<T1>>(
        &self, val: V, ty: T2, name: &str) -> Instruction<T2> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildSExt(self.r, val.to_ref(), ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn fp_to_ui<V:Val<ty::Real>>(
        &self, val: V, ty: ty::Integer, name: &str) -> Instruction<ty::Integer> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildFPToUI(self.r, val.to_ref(), ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn fp_to_si<V:Val<ty::Real>>(
        &self, val: V, ty: ty::Integer, name: &str) -> Instruction<ty::Integer> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildFPToSI(self.r, val.to_ref(), ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn ui_to_fp<V:Val<ty::Integer>>(
        &self, val: V, ty: ty::Real, name: &str) -> Instruction<ty::Real> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildUIToFP(self.r, val.to_ref(), ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn si_to_fp<V:Val<ty::Integer>>(
        &self, val: V, ty: ty::Real, name: &str) -> Instruction<ty::Real> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildSIToFP(self.r, val.to_ref(), ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn fptrunc<V:Val<ty::Real>>(
        &self, val:V, ty: ty::Real, name: &str) -> Instruction<ty::Real> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildFPTrunc(self.r, val.to_ref(), ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn fpext<V:Val<ty::Real>>(
        &self, val:V, ty: ty::Real, name: &str) -> Instruction<ty::Real> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildFPExt(self.r, val.to_ref(), ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn ptr_to_int<T:ty::Ty,VP:Val<ty::Pointer<T>>>(
        &self, val: VP, ty: ty::Integer, name: &str) -> Instruction<ty::Integer> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildPtrToInt(self.r, val.to_ref(), ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn int_to_ptr<T:ty::Ty,V:Val<ty::Integer>>(
        &self, val: V, ty: ty::Pointer<T>, name: &str) -> Instruction<ty::Pointer<T>> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildIntToPtr(self.r, val.to_ref(), ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn bitcast<T1:ty::Ty,T2:ty::Ty,V:Val<T1>>(
        &self, val: V, ty: T2, name: &str) -> Instruction<T2> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildBitCast(self.r, val.to_ref(), ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn zext_or_bitcast<T1:ty::Ty,T2:ty::Ty,V:Val<T1>>(
        &self, val: V, ty: T2, name: &str) -> Instruction<T2> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildZExtOrBitCast(self.r, val.to_ref(), ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn sext_or_bitcast<T1:ty::Ty,T2:ty::Ty,V:Val<T1>>(
        &self, val: V, ty: T2, name: &str) -> Instruction<T2> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildSExtOrBitCast(self.r, val.to_ref(), ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn trunc_or_bitcast<T1:ty::Ty,T2:ty::Ty,V:Val<T1>>(
        &self, val: V, ty: T2, name: &str) -> Instruction<T2> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildTruncOrBitCast(self.r, val.to_ref(), ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn pointer_cast<T1:ty::Ty,T2:ty::Ty,V:Val<ty::Pointer<T1>>>(
        &self, val: V, ty: ty::Pointer<T2>, name: &str) -> Instruction<T2> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildPointerCast(self.r, val.to_ref(), ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn int_cast<V:Val<ty::Integer>>(&self, val: V, ty: ty::Integer, name: &str)
        -> Instruction<ty::Integer> {

        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildIntCast(self.r, val.to_ref(), ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn fp_cast<V:Val<ty::Real>>(&self, val: V, ty: ty::Real, name: &str)
        -> Instruction<ty::Real> {

        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildFPCast(self.r, val.to_ref(), ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn icmp<V1:Val<ty::Integer>,V2:Val<ty::Integer>>(
        &self, op: IntPredicate, lhs: V1, rhs: V2, name: &str) -> Instruction<ty::Integer> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildICmp(self.r, op as std::libc::c_uint,
                                      lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn fcmp<V1:Val<ty::Real>,V2:Val<ty::Real>>(
        &self, op: RealPredicate, lhs: V1, rhs: V2, name: &str) -> Instruction<ty::Real> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildFCmp(self.r, op as std::libc::c_uint,
                                      lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn phi<T:ty::Ty>(&self, ty: T, name: &str) -> PhiNode<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildPhi(self.r, ty.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn call(&self, fun: value::Function, args: &[Param<ty::Type>],
                name: &str) -> CallInst<ty::Type> {
        unsafe {
            let llargs = do args.map |a| { a.to_ref() };
            do str::as_c_str(name) |s| {
                do vec::as_imm_buf(llargs) |buf, len| {
                    let r = LLVMBuildCall(self.r, fun.to_ref(),
                                          buf, len as std::libc::c_uint, s);
                    Wrapper::from_ref(r)
                }
            }
        }
    }

    pub fn select<T:ty::Ty,VC:Val<ty::Integer>,VT:Val<T>,VE:Val<T>>(
        self, cond: VC, then: VT, els: VE, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildSelect(self.r, cond.to_ref(), then.to_ref(), els.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn extract_element<T:ty::Ty,V1:Val<ty::Vector<T>>,V2:Val<ty::Integer>>(
        &self, vec: V1, idx: V2, name: &str) -> Instruction<T> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildExtractElement(self.r, vec.to_ref(), idx.to_ref(), s);
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn ptr_diff<T:ty::Ty,V1:Val<ty::Pointer<T>>,V2:Val<ty::Pointer<T>>>(
        &self, lhs: V1, rhs:V2, name: &str) -> Instruction<ty::Integer> {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = LLVMBuildPtrDiff(self.r, lhs.to_ref(), rhs.to_ref(), s);
                Wrapper::from_ref(r)
            }
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
