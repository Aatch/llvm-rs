use std::libc::*;
use ffi::core::*;

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

externfn!(fn LLVMCreateTargetData(StringRep: *const c_char) -> TargetDataRef)
externfn!(fn LLVMByteOrder(TD: TargetDataRef) -> c_uint)
externfn!(fn LLVMPointerSize(TD: TargetDataRef) -> c_uint)
externfn!(fn LLVMPointerSizeForAS(TD: TargetDataRef, AS: c_uint) -> c_uint)
externfn!(fn LLVMIntPtrType(TD: TargetDataRef) -> TypeRef)
externfn!(fn LLVMIntPtrTypeForAS(TD: TargetDataRef, AS: c_uint) -> TypeRef)
externfn!(fn LLVMSizeOfTypeInBits(TD: TargetDataRef, Ty: TypeRef) -> c_ulonglong)
externfn!(fn LLVMStoreSizeOfType(TD: TargetDataRef, Ty: TypeRef) -> c_ulonglong)
externfn!(fn LLVMABISizeOfType(TD: TargetDataRef, Ty: TypeRef) -> c_ulonglong)
externfn!(fn LLVMABIAlignmentOfType(TD: TargetDataRef, Ty: TypeRef) -> c_uint)
externfn!(fn LLVMCallFrameAlignmentOfType(TD: TargetDataRef, Ty: TypeRef) -> c_uint)
externfn!(fn LLVMPreferredAlignmentOfType(TD: TargetDataRef, Ty: TypeRef) -> c_uint)
externfn!(fn LLVMPreferredAlignmentOfGlobal(TD: TargetDataRef, Global: ValueRef) -> c_uint)
externfn!(fn LLVMElementAtOffset(TD: TargetDataRef,
                           StructTy: TypeRef,
                           Offset: c_ulonglong) -> c_uint)
externfn!(fn LLVMOffsetOfElement(TD: TargetDataRef,
                           StructTy: TypeRef,
                           Element: c_uint) -> c_ulonglong)
externfn!(fn LLVMDisposeTargetData(TD: TargetDataRef))

