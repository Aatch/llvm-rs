use ffi::core;
use std::kinds::marker;

pub struct Val<'a, T> {
    priv r: core::LLVMValueRef,
    priv ty: marker::CovariantType<T>,
    priv lifetime: marker::ContravariantLifetime<'a>
}

pub mod traits {
    pub trait Value {}

    pub trait User : Value {}

    pub trait Constant : User {}
    pub trait GlobalValue : Constant {}

    pub trait Instruction : User {}

    pub trait CallInst : Instruction {}
    pub trait IntrinsicInst : Instruction {}

    pub trait DbgInfoIntrinsic : IntrinsicInst {}
    pub trait DbgDeclareInst : DbgInfoIntrinsic {}
    pub trait MemIntrinsic : IntrinsicInst {}

    pub trait CmpInst : Instruction {}
    pub trait TerminatorInst : Instruction {}

    pub trait UnaryInstruction : Instruction {}
    pub trait CastInst : UnaryInstruction {}
}

impl<'a, T:traits::Value> Val<'a, T> {
    pub fn get_value_name<'s>(&'s self) -> &'s str {
        use std::cast::transmute;
        use std::c_str;
        unsafe {
            let name = core::LLVMGetValueName(self.r);
            if name.is_null() { return ""; }

            let c_str = c_str::CString::new(name, false);
            match c_str.as_str() {
                Some(s) => transmute(s),
                None => ""
            }
        }
    }

    pub fn set_value_name(&mut self, name: &str) {
        let c_str = name.to_c_str();
        c_str.with_ref(|c_buffer| {
            unsafe {
                core::LLVMSetValueName(self.r, c_buffer);
            }
        });
    }

    pub fn is_constant(&self) -> bool {
        unsafe {
            core::LLVMIsConstant(self.r) == 1
        }
    }

    pub fn is_undef(&self) -> bool {
        unsafe {
            core::LLVMIsUndef(self.r) == 1
        }
    }
}

pub enum Value {}
pub enum Argument {}
pub enum BasicBlock {}
pub enum InlineAsm {}
pub enum MDNode {}
pub enum MDString {}

pub enum User {}

pub enum Constant {}
pub enum BlockAddress {}
pub enum ConstantAggregateZero {}
pub enum ConstantArray {}
pub enum ConstantExpr {}
pub enum ConstantFP {}
pub enum ConstantInt {}
pub enum ConstantPointerNull {}
pub enum ConstantStruct {}
pub enum ConstantVector {}
pub enum GlobalValue {}
pub enum Function {}
pub enum GlobalAlias {}
pub enum GlobalVariable {}
pub enum UndefValue {}

pub enum Instruction {}

pub enum BinaryOperator {}
pub enum CallInst {}
pub enum IntrinsicInst {}

pub enum DbgInfoIntrinsic {}
pub enum DbgDeclareInst {}
pub enum MemIntrinsic {}
pub enum MemCpyInst {}
pub enum MemMoveInst {}
pub enum MemSetInst {}

pub enum CmpInst {}
pub enum FCmpInst {}
pub enum ICmpInst {}

pub enum ExtractElementInst {}
pub enum GetElementPtrInst {}
pub enum InsertElementInst {}
pub enum InsertValueInst {}
pub enum LandingPadInst {}
pub enum PHINode {}
pub enum SelectInst {}
pub enum ShuffleVectorInst {}
pub enum StoreInst {}
pub enum TerminatorInst {}

pub enum BranchInst {}
pub enum IndirectBrInst {}
pub enum InvokeInst {}
pub enum ReturnInst {}
pub enum SwitchInst {}
pub enum UnreachableInst {}
pub enum ResumeInst {}

pub enum UnaryInstruction {}
pub enum AllocaInst {}
pub enum CastInst {}
pub enum BitCastInst {}
pub enum FPExtInst {}
pub enum FPToSIInst {}
pub enum FPToUIInst {}
pub enum FPTruncInst {}
pub enum IntToPtrInst {}
pub enum PtrToIntInst {}
pub enum SExtInst {}
pub enum SIToFPInst {}
pub enum TruncInst {}
pub enum UIToFPInst {}
pub enum ZExtInst {}
pub enum ExtractValueInst {}
pub enum LoadInst {}
pub enum VAArgInst {}

impl traits::Value for Value {}

impl traits::Value for Argument {}
impl traits::Value for BasicBlock {}
impl traits::Value for InlineAsm {}
impl traits::Value for MDNode {}
impl traits::Value for MDString {}

impl traits::Value for User {}

impl traits::Value for Constant {}
impl traits::Value for BlockAddress {}
impl traits::Value for ConstantAggregateZero {}
impl traits::Value for ConstantArray {}
impl traits::Value for ConstantExpr {}
impl traits::Value for ConstantFP {}
impl traits::Value for ConstantInt {}
impl traits::Value for ConstantPointerNull {}
impl traits::Value for ConstantStruct {}
impl traits::Value for ConstantVector {}
impl traits::Value for GlobalValue {}
impl traits::Value for Function {}
impl traits::Value for GlobalAlias {}
impl traits::Value for GlobalVariable {}
impl traits::Value for UndefValue {}

impl traits::Value for Instruction {}

impl traits::Value for BinaryOperator {}
impl traits::Value for CallInst {}
impl traits::Value for IntrinsicInst {}

impl traits::Value for DbgInfoIntrinsic {}
impl traits::Value for DbgDeclareInst {}
impl traits::Value for MemIntrinsic {}
impl traits::Value for MemCpyInst {}
impl traits::Value for MemMoveInst {}
impl traits::Value for MemSetInst {}

impl traits::Value for CmpInst {}
impl traits::Value for FCmpInst {}
impl traits::Value for ICmpInst {}

impl traits::Value for ExtractElementInst {}
impl traits::Value for GetElementPtrInst {}
impl traits::Value for InsertElementInst {}
impl traits::Value for InsertValueInst {}
impl traits::Value for LandingPadInst {}
impl traits::Value for PHINode {}
impl traits::Value for SelectInst {}
impl traits::Value for ShuffleVectorInst {}
impl traits::Value for StoreInst {}
impl traits::Value for TerminatorInst {}

impl traits::Value for BranchInst {}
impl traits::Value for IndirectBrInst {}
impl traits::Value for InvokeInst {}
impl traits::Value for ReturnInst {}
impl traits::Value for SwitchInst {}
impl traits::Value for UnreachableInst {}
impl traits::Value for ResumeInst {}

impl traits::Value for UnaryInstruction {}
impl traits::Value for AllocaInst {}
impl traits::Value for CastInst {}
impl traits::Value for BitCastInst {}
impl traits::Value for FPExtInst {}
impl traits::Value for FPToSIInst {}
impl traits::Value for FPToUIInst {}
impl traits::Value for FPTruncInst {}
impl traits::Value for IntToPtrInst {}
impl traits::Value for PtrToIntInst {}
impl traits::Value for SExtInst {}
impl traits::Value for SIToFPInst {}
impl traits::Value for TruncInst {}
impl traits::Value for UIToFPInst {}
impl traits::Value for ZExtInst {}
impl traits::Value for ExtractValueInst {}
impl traits::Value for LoadInst {}
impl traits::Value for VAArgInst {}

impl traits::User for User {}

impl traits::User for Constant {}
impl traits::User for BlockAddress {}
impl traits::User for ConstantAggregateZero {}
impl traits::User for ConstantArray {}
impl traits::User for ConstantExpr {}
impl traits::User for ConstantFP {}
impl traits::User for ConstantInt {}
impl traits::User for ConstantPointerNull {}
impl traits::User for ConstantStruct {}
impl traits::User for ConstantVector {}
impl traits::User for GlobalValue {}
impl traits::User for Function {}
impl traits::User for GlobalAlias {}
impl traits::User for GlobalVariable {}
impl traits::User for UndefValue {}

impl traits::User for Instruction {}

impl traits::User for BinaryOperator {}
impl traits::User for CallInst {}
impl traits::User for IntrinsicInst {}

impl traits::User for DbgInfoIntrinsic {}
impl traits::User for DbgDeclareInst {}
impl traits::User for MemIntrinsic {}
impl traits::User for MemCpyInst {}
impl traits::User for MemMoveInst {}
impl traits::User for MemSetInst {}

impl traits::User for CmpInst {}
impl traits::User for FCmpInst {}
impl traits::User for ICmpInst {}

impl traits::User for ExtractElementInst {}
impl traits::User for GetElementPtrInst {}
impl traits::User for InsertElementInst {}
impl traits::User for InsertValueInst {}
impl traits::User for LandingPadInst {}
impl traits::User for PHINode {}
impl traits::User for SelectInst {}
impl traits::User for ShuffleVectorInst {}
impl traits::User for StoreInst {}
impl traits::User for TerminatorInst {}

impl traits::User for BranchInst {}
impl traits::User for IndirectBrInst {}
impl traits::User for InvokeInst {}
impl traits::User for ReturnInst {}
impl traits::User for SwitchInst {}
impl traits::User for UnreachableInst {}
impl traits::User for ResumeInst {}

impl traits::User for UnaryInstruction {}
impl traits::User for AllocaInst {}
impl traits::User for CastInst {}
impl traits::User for BitCastInst {}
impl traits::User for FPExtInst {}
impl traits::User for FPToSIInst {}
impl traits::User for FPToUIInst {}
impl traits::User for FPTruncInst {}
impl traits::User for IntToPtrInst {}
impl traits::User for PtrToIntInst {}
impl traits::User for SExtInst {}
impl traits::User for SIToFPInst {}
impl traits::User for TruncInst {}
impl traits::User for UIToFPInst {}
impl traits::User for ZExtInst {}
impl traits::User for ExtractValueInst {}
impl traits::User for LoadInst {}
impl traits::User for VAArgInst {}

impl traits::Constant for Constant {}
impl traits::Constant for BlockAddress {}
impl traits::Constant for ConstantAggregateZero {}
impl traits::Constant for ConstantArray {}
impl traits::Constant for ConstantExpr {}
impl traits::Constant for ConstantFP {}
impl traits::Constant for ConstantInt {}
impl traits::Constant for ConstantPointerNull {}
impl traits::Constant for ConstantStruct {}
impl traits::Constant for ConstantVector {}
impl traits::Constant for GlobalValue {}
impl traits::Constant for Function {}
impl traits::Constant for GlobalAlias {}
impl traits::Constant for GlobalVariable {}
impl traits::Constant for UndefValue {}

impl traits::GlobalValue for GlobalValue {}
impl traits::GlobalValue for Function {}
impl traits::GlobalValue for GlobalAlias {}
impl traits::GlobalValue for GlobalVariable {}

impl traits::Instruction for Instruction {}

impl traits::Instruction for BinaryOperator {}
impl traits::Instruction for CallInst {}
impl traits::Instruction for IntrinsicInst {}

impl traits::Instruction for DbgInfoIntrinsic {}
impl traits::Instruction for DbgDeclareInst {}
impl traits::Instruction for MemIntrinsic {}
impl traits::Instruction for MemCpyInst {}
impl traits::Instruction for MemMoveInst {}
impl traits::Instruction for MemSetInst {}

impl traits::Instruction for CmpInst {}
impl traits::Instruction for FCmpInst {}
impl traits::Instruction for ICmpInst {}

impl traits::Instruction for ExtractElementInst {}
impl traits::Instruction for GetElementPtrInst {}
impl traits::Instruction for InsertElementInst {}
impl traits::Instruction for InsertValueInst {}
impl traits::Instruction for LandingPadInst {}
impl traits::Instruction for PHINode {}
impl traits::Instruction for SelectInst {}
impl traits::Instruction for ShuffleVectorInst {}
impl traits::Instruction for StoreInst {}
impl traits::Instruction for TerminatorInst {}

impl traits::Instruction for BranchInst {}
impl traits::Instruction for IndirectBrInst {}
impl traits::Instruction for InvokeInst {}
impl traits::Instruction for ReturnInst {}
impl traits::Instruction for SwitchInst {}
impl traits::Instruction for UnreachableInst {}
impl traits::Instruction for ResumeInst {}

impl traits::Instruction for UnaryInstruction {}
impl traits::Instruction for AllocaInst {}
impl traits::Instruction for CastInst {}
impl traits::Instruction for BitCastInst {}
impl traits::Instruction for FPExtInst {}
impl traits::Instruction for FPToSIInst {}
impl traits::Instruction for FPToUIInst {}
impl traits::Instruction for FPTruncInst {}
impl traits::Instruction for IntToPtrInst {}
impl traits::Instruction for PtrToIntInst {}
impl traits::Instruction for SExtInst {}
impl traits::Instruction for SIToFPInst {}
impl traits::Instruction for TruncInst {}
impl traits::Instruction for UIToFPInst {}
impl traits::Instruction for ZExtInst {}
impl traits::Instruction for ExtractValueInst {}
impl traits::Instruction for LoadInst {}
impl traits::Instruction for VAArgInst {}

impl traits::CallInst for CallInst {}
impl traits::CallInst for IntrinsicInst {}

impl traits::CallInst for DbgInfoIntrinsic {}
impl traits::CallInst for DbgDeclareInst {}
impl traits::CallInst for MemIntrinsic {}
impl traits::CallInst for MemCpyInst {}
impl traits::CallInst for MemMoveInst {}
impl traits::CallInst for MemSetInst {}

impl traits::IntrinsicInst for IntrinsicInst {}

impl traits::IntrinsicInst for DbgInfoIntrinsic {}
impl traits::IntrinsicInst for DbgDeclareInst {}
impl traits::IntrinsicInst for MemIntrinsic {}
impl traits::IntrinsicInst for MemCpyInst {}
impl traits::IntrinsicInst for MemMoveInst {}
impl traits::IntrinsicInst for MemSetInst {}

impl traits::DbgInfoIntrinsic for DbgInfoIntrinsic {}
impl traits::DbgInfoIntrinsic for DbgDeclareInst {}

impl traits::MemIntrinsic for MemIntrinsic {}
impl traits::MemIntrinsic for MemCpyInst {}
impl traits::MemIntrinsic for MemMoveInst {}
impl traits::MemIntrinsic for MemSetInst {}

impl traits::CmpInst for CmpInst {}
impl traits::CmpInst for FCmpInst {}
impl traits::CmpInst for ICmpInst {}

impl traits::TerminatorInst for TerminatorInst {}

impl traits::TerminatorInst for BranchInst {}
impl traits::TerminatorInst for IndirectBrInst {}
impl traits::TerminatorInst for InvokeInst {}
impl traits::TerminatorInst for ReturnInst {}
impl traits::TerminatorInst for SwitchInst {}
impl traits::TerminatorInst for UnreachableInst {}
impl traits::TerminatorInst for ResumeInst {}

impl traits::UnaryInstruction for UnaryInstruction {}
impl traits::UnaryInstruction for AllocaInst {}
impl traits::UnaryInstruction for CastInst {}
impl traits::UnaryInstruction for BitCastInst {}
impl traits::UnaryInstruction for FPExtInst {}
impl traits::UnaryInstruction for FPToSIInst {}
impl traits::UnaryInstruction for FPToUIInst {}
impl traits::UnaryInstruction for FPTruncInst {}
impl traits::UnaryInstruction for IntToPtrInst {}
impl traits::UnaryInstruction for PtrToIntInst {}
impl traits::UnaryInstruction for SExtInst {}
impl traits::UnaryInstruction for SIToFPInst {}
impl traits::UnaryInstruction for TruncInst {}
impl traits::UnaryInstruction for UIToFPInst {}
impl traits::UnaryInstruction for ZExtInst {}
impl traits::UnaryInstruction for ExtractValueInst {}
impl traits::UnaryInstruction for LoadInst {}
impl traits::UnaryInstruction for VAArgInst {}

impl traits::CastInst for CastInst {}
impl traits::CastInst for BitCastInst {}
impl traits::CastInst for FPExtInst {}
impl traits::CastInst for FPToSIInst {}
impl traits::CastInst for FPToUIInst {}
impl traits::CastInst for FPTruncInst {}
impl traits::CastInst for IntToPtrInst {}
impl traits::CastInst for PtrToIntInst {}
impl traits::CastInst for SExtInst {}
impl traits::CastInst for SIToFPInst {}
impl traits::CastInst for TruncInst {}
impl traits::CastInst for UIToFPInst {}
impl traits::CastInst for ZExtInst {}
