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

extern {
    #[fast_ffi]
    pub fn LLVMLinkInJIT();
    #[fast_ffi]
    pub fn LLVMLinkInMCJIT();
    #[fast_ffi]
    pub fn LLVMLinkInInterpreter();

    #[fast_ffi]
    pub fn LLVMCreateGenericValueOfInt(Ty: TypeRef,
                                       N: c_ulonglong,
                                       IsSigned: Bool) -> GenericValueRef;
    #[fast_ffi]
    pub fn LLVMCreateGenericValueOfPointer(P: *c_void) -> GenericValueRef;
    #[fast_ffi]
    pub fn LLVMCreateGenericValueOfFloat(Ty: TypeRef, N: c_double) -> GenericValueRef;
    #[fast_ffi]
    pub fn LLVMGenericValueIntWidth(GenValRef: GenericValueRef) -> c_uint;
    #[fast_ffi]
    pub fn LLVMGenericValueToInt(GenVal: GenericValueRef, IsSigned: Bool) -> c_ulonglong;
    #[fast_ffi]
    pub fn LLVMGenericValueToPointer(GenVal: GenericValueRef) -> *c_void;
    #[fast_ffi]
    pub fn LLVMGenericValueToFloat(Ty: TypeRef, GenVal: GenericValueRef) -> c_double;
    #[fast_ffi]
    pub fn LLVMDisposeGenericValue(GenVal: GenericValueRef);
    #[fast_ffi]
    pub fn LLVMCreateExecutionEngineForModule(OutEE: *mut ExecutionEngineRef,
                                              M: ModuleRef,
                                              OutError: *mut *c_char) -> Bool;
    #[fast_ffi]
    pub fn LLVMCreateInterpreterForModule(OutEE: *mut ExecutionEngineRef,
                                          M: ModuleRef,
                                          OutError: *mut *c_char) -> Bool;
    #[fast_ffi]
    pub fn LLVMCreateJITCompilerForModule(OutEE: *mut ExecutionEngineRef,
                                          M: ModuleRef,
                                          OptLevel: c_uint,
                                          OutError: *mut *c_char) -> Bool;
    #[fast_ffi]
    pub fn LLVMInitializeMCJITCompilerOptions(Options: *MCJITCompilerOptions,
                                              SizeOfOptions: size_t);
    #[fast_ffi]
    pub fn LLVMCreateMCJITCompilerForModule(OutEE: *mut ExecutionEngineRef,
                                            M: ModuleRef,
                                            Options: *MCJITCompilerOptions,
                                            SizeOfOptions: size_t,
                                            OutError: *mut *c_char) -> Bool;
    #[fast_ffi]
    pub fn LLVMCreateExecutionEngine(OutEE: *mut ExecutionEngineRef,
                                     MP: ModuleProviderRef,
                                     OutError: *mut *c_char) -> Bool;
    #[fast_ffi]
    pub fn LLVMCreateInterpreter(OutEE: *mut ExecutionEngineRef,
                                 MP: ModuleProviderRef,
                                 OutError: *mut *c_char) -> Bool;
    #[fast_ffi]
    pub fn LLVMCreateJITCompiler(OutEE: *mut ExecutionEngineRef,
                                 MP: ModuleProviderRef,
                                 OptLevel: c_uint,
                                 OutError: *mut *c_char) -> Bool;
    #[fast_ffi]
    pub fn LLVMDisposeExecutionEngine(EE: ExecutionEngineRef);
    #[fast_ffi]
    pub fn LLVMRunStaticConstructors(EE: ExecutionEngineRef);
    #[fast_ffi]
    pub fn LLVMRunStaticDestructors(EE: ExecutionEngineRef);
    #[fast_ffi]
    pub fn LLVMRunFunctionAsMain(EE: ExecutionEngineRef,
                                 F: ValueRef,
                                 ArgC: c_uint,
                                 ArgV: **c_char,
                                 EnvP: **c_char) -> c_int;
    #[fast_ffi]
    pub fn LLVMRunFunction(EE: ExecutionEngineRef,
                           F: ValueRef,
                           NumArgs: c_uint,
                           Args: *GenericValueRef) -> GenericValueRef;
    #[fast_ffi]
    pub fn LLVMFreeMachineCodeForFunction(EE: ExecutionEngineRef, F: ValueRef);
    #[fast_ffi]
    pub fn LLVMAddModule(EE: ExecutionEngineRef, M: ModuleRef);
    #[fast_ffi]
    pub fn LLVMAddModuleProvider(EE: ExecutionEngineRef, MP: ModuleProviderRef);
    #[fast_ffi]
    pub fn LLVMRemoveModule(EE: ExecutionEngineRef,
                            M: ModuleRef,
                            OutMod: *mut ModuleRef,
                            OutError: *mut *c_char) -> Bool;
    #[fast_ffi]
    pub fn LLVMRemoveModuleProvider(EE: ExecutionEngineRef,
                                    MP: ModuleProviderRef,
                                    OutMod: *mut ModuleRef,
                                    OutError: *mut *c_char) -> Bool;
    #[fast_ffi]
    pub fn LLVMFindFunction(EE: ExecutionEngineRef,
                            Name: *const c_char,
                            OutFn: *mut ValueRef) -> Bool;
    #[fast_ffi]
    pub fn LLVMRecompileAndRelinkFunction(EE: ExecutionEngineRef, Fn: ValueRef) -> *c_void;
    #[fast_ffi]
    pub fn LLVMGetExecutionEngineTargetData(EE: ExecutionEngineRef) -> TargetDataRef;
    #[fast_ffi]
    pub fn LLVMAddGlobalMapping(EE: ExecutionEngineRef, Global: ValueRef, Addr: *c_void);
    #[fast_ffi]
    pub fn LLVMGetPointerToGlobal(EE: ExecutionEngineRef, Global: ValueRef) -> *c_void;
}
