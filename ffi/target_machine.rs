use std::libc::*;
use ffi::core::*;

macro_rules! wrap_llvm(($id:ident) => (pub struct $id{priv p:*()}))

pub enum _TargetRef{}
pub enum _TargetDataRef{}
pub enum _TargetMachineRef{}
pub enum _TargetLibraryInfoRef{}
pub enum _StructLayoutRef{}

pub type TargetRef = *_TargetRef;
pub type TargetDataRef = *_TargetDataRef;
pub type TargetMachineRef = *_TargetMachineRef;
pub type TargetLibraryInfoRef = *_TargetLibraryInfoRef;
pub type StructLayoutRef = *_StructLayoutRef;

pub enum ByteOrdering {
    BigEndian,
    LittleEndian
}

pub enum CodeGenOptLevel {
    CodeGenLevelNone,
    CodeGenLevelLess,
    CodeGenLevelDefault,
    CodeGenLevelAggressive,
}

pub enum RelocMode {
    RelocDefault,
    RelocStatic,
    RelocPIC,
    RelocDynamicNoPic,
}

pub enum CodeModel {
    CodeModelDefault,
    CodeModelJITDefault,
    CodeModelSmall,
    CodeModelKernel,
    CodeModelMedium,
    CodeModelLarge,
}

pub enum CodeGenFileType {
    CodeGenAssemblyFile = 0,
    CodeGenObjectFile = 1
}

extern {
    #[fast_ffi]
    pub fn LLVMCreateTargetData(StringRep: *const c_char) -> TargetDataRef;
    #[fast_ffi]
    pub fn LLVMByteOrder(TD: TargetDataRef) -> c_uint;
    #[fast_ffi]
    pub fn LLVMPointerSize(TD: TargetDataRef) -> c_uint;
    #[fast_ffi]
    pub fn LLVMPointerSizeForAS(TD: TargetDataRef, AS: c_uint) -> c_uint;
    #[fast_ffi]
    pub fn LLVMIntPtrType(TD: TargetDataRef) -> TypeRef;
    #[fast_ffi]
    pub fn LLVMIntPtrTypeForAS(TD: TargetDataRef, AS: c_uint) -> TypeRef;
    #[fast_ffi]
    pub fn LLVMSizeOfTypeInBits(TD: TargetDataRef, Ty: TypeRef) -> c_ulonglong;
    #[fast_ffi]
    pub fn LLVMStoreSizeOfType(TD: TargetDataRef, Ty: TypeRef) -> c_ulonglong;
    #[fast_ffi]
    pub fn LLVMABISizeOfType(TD: TargetDataRef, Ty: TypeRef) -> c_ulonglong;
    #[fast_ffi]
    pub fn LLVMABIAlignmentOfType(TD: TargetDataRef, Ty: TypeRef) -> c_uint;
    #[fast_ffi]
    pub fn LLVMCallFrameAlignmentOfType(TD: TargetDataRef, Ty: TypeRef) -> c_uint;
    #[fast_ffi]
    pub fn LLVMPreferredAlignmentOfType(TD: TargetDataRef, Ty: TypeRef) -> c_uint;
    #[fast_ffi]
    pub fn LLVMPreferredAlignmentOfGlobal(TD: TargetDataRef, Global: ValueRef) -> c_uint;
    #[fast_ffi]
    pub fn LLVMElementAtOffset(TD: TargetDataRef,
                               StructTy: TypeRef,
                               Offset: c_ulonglong) -> c_uint;
    #[fast_ffi]
    pub fn LLVMOffsetOfElement(TD: TargetDataRef,
                               StructTy: TypeRef,
                               Element: c_uint) -> c_ulonglong;
    #[fast_ffi]
    pub fn LLVMDisposeTargetData(TD: TargetDataRef);
}
