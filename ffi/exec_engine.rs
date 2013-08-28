use ffi::core::*;
use ffi::target_machine::*;
use std::libc::*;

pub enum _GenericValueRef {}
pub enum _ExecutionEngineRef {}
pub enum _MCJITMemoryManangerRef {}

pub type GenericValueRef = *_GenericValueRef;
pub type ExecutionEngineRef = *_ExecutionEngineRef;
pub type MCJITMemoryManangerRef = *_MCJITMemoryManangerRef;

struct MCJITCompilerOptions {
    OptLevel: c_uint,
    CodeModel: CodeModel,
    NoFramePointerElim: Bool,
    EnableFastISel: Bool,
    MCJMM: MCJITMemoryManangerRef,
}

externfn!(fn LLVMLinkInJIT())
externfn!(fn LLVMLinkInMCJIT())
externfn!(fn LLVMLinkInInterpreter())

externfn!(fn LLVMCreateGenericValueOfInt(Ty: TypeRef,
                                   N: c_ulonglong,
                                   IsSigned: Bool) -> GenericValueRef)
externfn!(fn LLVMCreateGenericValueOfPointer(P: *c_void) -> GenericValueRef)
externfn!(fn LLVMCreateGenericValueOfFloat(Ty: TypeRef, N: c_double) -> GenericValueRef)
externfn!(fn LLVMGenericValueIntWidth(GenValRef: GenericValueRef) -> c_uint)
externfn!(fn LLVMGenericValueToInt(GenVal: GenericValueRef, IsSigned: Bool) -> c_ulonglong)
externfn!(fn LLVMGenericValueToPointer(GenVal: GenericValueRef) -> *c_void)
externfn!(fn LLVMGenericValueToFloat(Ty: TypeRef, GenVal: GenericValueRef) -> c_double)
externfn!(fn LLVMDisposeGenericValue(GenVal: GenericValueRef))
externfn!(fn LLVMCreateExecutionEngineForModule(OutEE: *mut ExecutionEngineRef,
                                          M: ModuleRef,
                                          OutError: *mut *c_char) -> Bool)
externfn!(fn LLVMCreateInterpreterForModule(OutEE: *mut ExecutionEngineRef,
                                      M: ModuleRef,
                                      OutError: *mut *c_char) -> Bool)
externfn!(fn LLVMCreateJITCompilerForModule(OutEE: *mut ExecutionEngineRef,
                                      M: ModuleRef,
                                      OptLevel: c_uint,
                                      OutError: *mut *c_char) -> Bool)
externfn!(fn LLVMInitializeMCJITCompilerOptions(Options: *MCJITCompilerOptions,
                                          SizeOfOptions: size_t))
externfn!(fn LLVMCreateMCJITCompilerForModule(OutEE: *mut ExecutionEngineRef,
                                        M: ModuleRef,
                                        Options: *MCJITCompilerOptions,
                                        SizeOfOptions: size_t,
                                        OutError: *mut *c_char) -> Bool)
externfn!(fn LLVMCreateExecutionEngine(OutEE: *mut ExecutionEngineRef,
                                 MP: ModuleProviderRef,
                                 OutError: *mut *c_char) -> Bool)
externfn!(fn LLVMCreateInterpreter(OutEE: *mut ExecutionEngineRef,
                             MP: ModuleProviderRef,
                             OutError: *mut *c_char) -> Bool)
externfn!(fn LLVMCreateJITCompiler(OutEE: *mut ExecutionEngineRef,
                             MP: ModuleProviderRef,
                             OptLevel: c_uint,
                             OutError: *mut *c_char) -> Bool)
externfn!(fn LLVMDisposeExecutionEngine(EE: ExecutionEngineRef))
externfn!(fn LLVMRunStaticConstructors(EE: ExecutionEngineRef))
externfn!(fn LLVMRunStaticDestructors(EE: ExecutionEngineRef))
externfn!(fn LLVMRunFunctionAsMain(EE: ExecutionEngineRef,
                             F: ValueRef,
                             ArgC: c_uint,
                             ArgV: **c_char,
                             EnvP: **c_char) -> c_int)
externfn!(fn LLVMRunFunction(EE: ExecutionEngineRef,
                       F: ValueRef,
                       NumArgs: c_uint,
                       Args: *GenericValueRef) -> GenericValueRef)
externfn!(fn LLVMFreeMachineCodeForFunction(EE: ExecutionEngineRef, F: ValueRef))
externfn!(fn LLVMAddModule(EE: ExecutionEngineRef, M: ModuleRef))
externfn!(fn LLVMAddModuleProvider(EE: ExecutionEngineRef, MP: ModuleProviderRef))
externfn!(fn LLVMRemoveModule(EE: ExecutionEngineRef,
                        M: ModuleRef,
                        OutMod: *mut ModuleRef,
                        OutError: *mut *c_char) -> Bool)
externfn!(fn LLVMRemoveModuleProvider(EE: ExecutionEngineRef,
                                MP: ModuleProviderRef,
                                OutMod: *mut ModuleRef,
                                OutError: *mut *c_char) -> Bool)
externfn!(fn LLVMFindFunction(EE: ExecutionEngineRef,
                        Name: *const c_char,
                        OutFn: *mut ValueRef) -> Bool)
externfn!(fn LLVMRecompileAndRelinkFunction(EE: ExecutionEngineRef, Fn: ValueRef) -> *c_void)
externfn!(fn LLVMGetExecutionEngineTargetData(EE: ExecutionEngineRef) -> TargetDataRef)
externfn!(fn LLVMAddGlobalMapping(EE: ExecutionEngineRef, Global: ValueRef, Addr: *c_void))
externfn!(fn LLVMGetPointerToGlobal(EE: ExecutionEngineRef, Global: ValueRef) -> *c_void)
