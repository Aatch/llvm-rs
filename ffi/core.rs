use std::libc::*;

pub type Bool = c_int;
pub static True : Bool = 1;
pub static False : Bool = 0;

pub enum _ContextRef {}
pub enum _ModuleRef {}
pub enum _TypeRef {}
pub enum _ValueRef {}
pub enum _BasicBlockRef {}
pub enum _BuilderRef {}
pub enum _ModuleProviderRef {}
pub enum _MemoryBufferRef {}
pub enum _PassManagerRef {}
pub enum _PassRegistryRef {}
pub enum _UseRef {}

pub type ContextRef = *_ContextRef;
pub type ModuleRef = *_ModuleRef;
pub type TypeRef = *_TypeRef;
pub type ValueRef = *_ValueRef;
pub type BasicBlockRef = *_BasicBlockRef;
pub type BuilderRef = *_BuilderRef;
pub type ModuleProviderRef = *_ModuleProviderRef;
pub type MemoryBufferRef = *_MemoryBufferRef;
pub type PassManagerRef = *_PassManagerRef;
pub type PassRegistryRef = *_PassRegistryRef;
pub type UseRef = *_UseRef;

pub enum CallConv {
    CCallConv = 0,
    FastCallConv = 8,
    ColdCallConv = 9,
    X86StdcallCallConv = 64,
    X86FastcallCallConv = 65,
}

pub enum Visibility {
    LLVMDefaultVisibility = 0,
    HiddenVisibility = 1,
    ProtectedVisibility = 2,
}

pub enum Linkage {
    ExternalLinkage = 0,
    AvailableExternallyLinkage = 1,
    LinkOnceAnyLinkage = 2,
    LinkOnceODRLinkage = 3,
    LinkOnceODRAutoHideLinkage = 4,
    WeakAnyLinkage = 5,
    WeakODRLinkage = 6,
    AppendingLinkage = 7,
    InternalLinkage = 8,
    PrivateLinkage = 9,
    DLLImportLinkage = 10,
    DLLExportLinkage = 11,
    ExternalWeakLinkage = 12,
    GhostLinkage = 13,
    CommonLinkage = 14,
    LinkerPrivateLinkage = 15,
    LinkerPrivateWeakLinkage = 16,
}

pub enum Attribute {
    ZExtAttribute = 1,
    SExtAttribute = 2,
    NoReturnAttribute = 4,
    InRegAttribute = 8,
    StructRetAttribute = 16,
    NoUnwindAttribute = 32,
    NoAliasAttribute = 64,
    ByValAttribute = 128,
    NestAttribute = 256,
    ReadNoneAttribute = 512,
    ReadOnlyAttribute = 1024,
    NoInlineAttribute = 2048,
    AlwaysInlineAttribute = 4096,
    OptimizeForSizeAttribute = 8192,
    StackProtectAttribute = 16384,
    StackProtectReqAttribute = 32768,
    // 31 << 16
    AlignmentAttribute = 2031616,
    NoCaptureAttribute = 2097152,
    NoRedZoneAttribute = 4194304,
    NoImplicitFloatAttribute = 8388608,
    NakedAttribute = 16777216,
    InlineHintAttribute = 33554432,
    // 7 << 26
    StackAttribute = 469762048,
    ReturnsTwiceAttribute = 536870912,
    // 1 << 30
    UWTableAttribute = 1073741824,
    NonLazyBindAttribute = 2147483648,
}

// enum for the LLVM IntPredicate type
pub enum IntPredicate {
    IntEQ = 32,
    IntNE = 33,
    IntUGT = 34,
    IntUGE = 35,
    IntULT = 36,
    IntULE = 37,
    IntSGT = 38,
    IntSGE = 39,
    IntSLT = 40,
    IntSLE = 41,
}

// enum for the LLVM RealPredicate type
pub enum RealPredicate {
    RealPredicateFalse = 0,
    RealOEQ = 1,
    RealOGT = 2,
    RealOGE = 3,
    RealOLT = 4,
    RealOLE = 5,
    RealONE = 6,
    RealORD = 7,
    RealUNO = 8,
    RealUEQ = 9,
    RealUGT = 10,
    RealUGE = 11,
    RealULT = 12,
    RealULE = 13,
    RealUNE = 14,
    RealPredicateTrue = 15,
}

pub type TypeKind = u32;
pub static Void: TypeKind      = 0;
pub static Half: TypeKind      = 1;
pub static Float: TypeKind     = 2;
pub static Double: TypeKind    = 3;
pub static X86_FP80: TypeKind  = 4;
pub static FP128: TypeKind     = 5;
pub static PPC_FP128: TypeKind = 6;
pub static Label: TypeKind     = 7;
pub static Integer: TypeKind   = 8;
pub static Function: TypeKind  = 9;
pub static Struct: TypeKind    = 10;
pub static Array: TypeKind     = 11;
pub static Pointer: TypeKind   = 12;
pub static Vector: TypeKind    = 13;
pub static Metadata: TypeKind  = 14;
pub static X86_MMX: TypeKind   = 15;

pub enum AtomicBinOp {
    AtomicXchg = 0,
    AtomicAdd  = 1,
    AtomicSub  = 2,
    AtomicAnd  = 3,
    AtomicNand = 4,
    AtomicOr   = 5,
    AtomicXor  = 6,
    AtomicMax  = 7,
    AtomicMin  = 8,
    AtomicUMax = 9,
    AtomicUMin = 10,
}

pub enum AtomicOrdering {
    NotAtomic = 0,
    Unordered = 1,
    Monotonic = 2,
    // Consume = 3,  // Not specified yet.
    Acquire = 4,
    Release = 5,
    AcquireRelease = 6,
    SequentiallyConsistent = 7
}

pub enum Metadata {
    MD_dbg = 0,
    MD_tbaa = 1,
    MD_prof = 2,
    MD_fpmath = 3,
    MD_range = 4,
    MD_tbaa_struct = 5
}

// Inline Asm Dialect
pub enum AsmDialect {
    AD_ATT   = 0,
    AD_Intel = 1
}

pub enum ThreadLocalMode {
    NotThreadLocal = 0,
    GeneralDynamicTLSModel = 1,
    LocalDynamicTLSModel = 2,
    InitialExecTLSModel = 3,
    LocalExecTLSModel = 4,
}

pub enum OpCode {
    /* Terminator Instructions */
    Ret            = 1,
    Br             = 2,
    Switch         = 3,
    IndirectBr     = 4,
    Invoke         = 5,
    /* removed 6 due to API changes */
    Unreachable    = 7,

    /* Standard Binary Operators */
    Add            = 8,
    FAdd           = 9,
    Sub            = 10,
    FSub           = 11,
    Mul            = 12,
    FMul           = 13,
    UDiv           = 14,
    SDiv           = 15,
    FDiv           = 16,
    URem           = 17,
    SRem           = 18,
    FRem           = 19,

    /* Logical Operators */
    Shl            = 20,
    LShr           = 21,
    AShr           = 22,
    And            = 23,
    Or             = 24,
    Xor            = 25,

    /* Memory Operators */
    Alloca         = 26,
    Load           = 27,
    Store          = 28,
    GetElementPtr  = 29,

    /* Cast Operators */
    Trunc          = 30,
    ZExt           = 31,
    SExt           = 32,
    FPToUI         = 33,
    FPToSI         = 34,
    UIToFP         = 35,
    SIToFP         = 36,
    FPTrunc        = 37,
    FPExt          = 38,
    PtrToInt       = 39,
    IntToPtr       = 40,
    BitCast        = 41,

    /* Other Operators */
    ICmp           = 42,
    FCmp           = 43,
    PHI            = 44,
    Call           = 45,
    Select         = 46,
    UserOp1        = 47,
    UserOp2        = 48,
    VAArg          = 49,
    ExtractElement = 50,
    InsertElement  = 51,
    ShuffleVector  = 52,
    ExtractValue   = 53,
    InsertValue    = 54,

    /* Atomic operators */
    Fence          = 55,
    AtomicCmpXchg  = 56,
    AtomicRMW      = 57,

    /* Exception Handling Operators */
    Resume         = 58,
    LandingPad     = 59
}

externfn!(fn LLVMInitializeCore(R: PassRegistryRef))
externfn!(fn LLVMShutdown())

externfn!(fn LLVMDisposeMessage(Message: *c_char))

// Threading
externfn!(fn LLVMStartMultithreaded() -> Bool)
externfn!(fn LLVMStopMultithreaded())
externfn!(fn LLVMIsMultithreaded() -> Bool)

#[link_args="-lLLVMIRReader -lLLVMAsmParser -lLLVMSystemZCodeGen -lLLVMSystemZAsmParser \
             -lLLVMSystemZDesc -lLLVMSystemZInfo -lLLVMSystemZAsmPrinter -lLLVMHexagonCodeGen \
             -lLLVMHexagonAsmPrinter -lLLVMHexagonDesc -lLLVMHexagonInfo -lLLVMNVPTXCodeGen \
             -lLLVMNVPTXDesc -lLLVMNVPTXInfo -lLLVMNVPTXAsmPrinter -lLLVMMBlazeDisassembler \
             -lLLVMMBlazeCodeGen -lLLVMMBlazeDesc -lLLVMMBlazeAsmPrinter -lLLVMMBlazeAsmParser \
             -lLLVMMBlazeInfo -lLLVMCppBackendCodeGen -lLLVMCppBackendInfo -lLLVMMSP430CodeGen \
             -lLLVMMSP430Desc -lLLVMMSP430Info -lLLVMMSP430AsmPrinter -lLLVMXCoreDisassembler \
             -lLLVMXCoreCodeGen -lLLVMXCoreDesc -lLLVMXCoreInfo -lLLVMXCoreAsmPrinter \
             -lLLVMMipsDisassembler -lLLVMMipsCodeGen -lLLVMMipsAsmParser -lLLVMMipsDesc \
             -lLLVMMipsInfo -lLLVMMipsAsmPrinter -lLLVMARMDisassembler -lLLVMARMCodeGen \
             -lLLVMARMAsmParser -lLLVMARMDesc -lLLVMARMInfo -lLLVMARMAsmPrinter \
             -lLLVMAArch64Disassembler -lLLVMAArch64CodeGen -lLLVMAArch64AsmParser \
             -lLLVMAArch64Desc -lLLVMAArch64Info -lLLVMAArch64AsmPrinter -lLLVMAArch64Utils \
             -lLLVMPowerPCCodeGen -lLLVMPowerPCDesc -lLLVMPowerPCAsmPrinter \
             -lLLVMPowerPCAsmParser -lLLVMPowerPCInfo -lLLVMSparcCodeGen -lLLVMSparcDesc \
             -lLLVMSparcInfo -lLLVMR600CodeGen -lLLVMR600Desc -lLLVMR600Info -lLLVMR600AsmPrinter \
             -lLLVMTableGen -lLLVMDebugInfo -lLLVMOption -lLLVMX86Disassembler -lLLVMX86AsmParser \
             -lLLVMX86CodeGen -lLLVMSelectionDAG -lLLVMAsmPrinter -lLLVMX86Desc -lLLVMX86Info \
             -lLLVMX86AsmPrinter -lLLVMX86Utils -lLLVMMCDisassembler -lLLVMMCParser \
             -lLLVMInstrumentation -lLLVMArchive -lLLVMBitReader -lLLVMInterpreter -lLLVMipo \
             -lLLVMVectorize -lLLVMLinker -lLLVMBitWriter -lLLVMMCJIT -lLLVMJIT -lLLVMCodeGen \
             -lLLVMObjCARCOpts -lLLVMScalarOpts -lLLVMInstCombine -lLLVMTransformUtils -lLLVMipa \
             -lLLVMAnalysis -lLLVMRuntimeDyld -lLLVMExecutionEngine -lLLVMTarget -lLLVMMC \
             -lLLVMObject -lLLVMCore -lLLVMSupport"] extern {}

pub mod context {
    use super::*;
    use std::libc::*;

    externfn!(fn LLVMContextCreate() -> ContextRef)
    externfn!(fn LLVMGetGlobalContext() -> ContextRef)
    externfn!(fn LLVMContextDispose(C: ContextRef))
    externfn!(fn LLVMGetMDKindIDInContext(C:ContextRef, name:*c_char, SLen:c_uint) -> c_uint)
}

pub mod module {
    use super::*;
    use std::libc::*;

    externfn!(fn LLVMModuleCreateWithName(MID:*c_char) -> ModuleRef)
    externfn!(fn LLVMModuleCreateWithNameInContext(MID:*c_char, C:ContextRef) -> ModuleRef)
    externfn!(fn LLVMDisposeModule(M:ModuleRef))
    externfn!(fn LLVMGetDataLayout(M:ModuleRef) -> *c_char)
    externfn!(fn LLVMSetDataLayout(M:ModuleRef,Triple:*c_char))
    externfn!(fn LLVMGetTarget(M:ModuleRef) -> *c_char)
    externfn!(fn LLVMSetTarget(M:ModuleRef,Triple:*c_char))
    externfn!(fn LLVMPrintModuleToFile(M:ModuleRef, Filename:*c_char, ErrMsg:*mut *c_char) -> Bool)
    externfn!(fn LLVMSetModuleInlineAsm(M:ModuleRef, Asm:*c_char))
    externfn!(fn LLVMGetModuleContext(M:ModuleRef) -> ContextRef)
    externfn!(fn LLVMGetTypeByName(M:ModuleRef, Name:*c_char) -> TypeRef)
    externfn!(fn LLVMGetNamedMetadataNumOperands(M:ModuleRef, name:*c_char) -> c_uint)
    externfn!(fn LLVMGetNamedMetadataOperands(M:ModuleRef, name:*c_char, Dest: *mut ValueRef))
    externfn!(fn LLVMAddNamedMetadataOperand(M:ModuleRef, name:*c_char, Val:ValueRef))
    externfn!(fn LLVMAddFunction(M:ModuleRef, Name:*c_char, FunctionTy:TypeRef) -> ValueRef)
    externfn!(fn LLVMGetNamedFunction(M:ModuleRef, Name:*c_char) -> ValueRef)
    externfn!(fn LLVMGetFirstFunction(M:ModuleRef) -> ValueRef)
    externfn!(fn LLVMGetLastFunction(M:ModuleRef) -> ValueRef)
    externfn!(fn LLVMGetNextFunction(Fn:ValueRef) -> ValueRef)
    externfn!(fn LLVMGetPreviousFunction(Fn:ValueRef) -> ValueRef)
}

pub mod types {
    use super::*;
    use std::libc::*;

    externfn!(fn LLVMGetTypeKind(Ty:TypeRef) -> TypeKind)
    externfn!(fn LLVMTypeIsSized(Ty:TypeRef) -> Bool)
    externfn!(fn LLVMGetTypeContext(Ty:TypeRef) -> ContextRef)

    // Integer Types
    externfn!(fn LLVMInt1TypeInContext(C:ContextRef) -> TypeRef)
    externfn!(fn LLVMInt8TypeInContext(C:ContextRef) -> TypeRef)
    externfn!(fn LLVMInt16TypeInContext(C:ContextRef) -> TypeRef)
    externfn!(fn LLVMInt32TypeInContext(C:ContextRef) -> TypeRef)
    externfn!(fn LLVMInt64TypeInContext(C:ContextRef) -> TypeRef)
    externfn!(fn LLVMIntTypeInContext(C:ContextRef, NumBits:c_uint) -> TypeRef)
    externfn!(fn LLVMInt1Type() -> TypeRef)
    externfn!(fn LLVMInt8Type() -> TypeRef)
    externfn!(fn LLVMInt16Type() -> TypeRef)
    externfn!(fn LLVMInt32Type() -> TypeRef)
    externfn!(fn LLVMInt64Type() -> TypeRef)
    externfn!(fn LLVMIntType(NumBits:c_uint) -> TypeRef)
    externfn!(fn LLVMGetIntTypeWidth(IntegerTy:TypeRef) -> c_uint)

    // Floating Point Types
    externfn!(fn LLVMHalfTypeInContext(C:ContextRef) -> TypeRef)
    externfn!(fn LLVMFloatTypeInContext(C: ContextRef) -> TypeRef)
    externfn!(fn LLVMDoubleTypeInContext(C: ContextRef) -> TypeRef)
    externfn!(fn LLVMX86FP80TypeInContext(C: ContextRef) -> TypeRef)
    externfn!(fn LLVMFP128TypeInContext(C: ContextRef) -> TypeRef)
    externfn!(fn LLVMPPCFP128TypeInContext(C: ContextRef) -> TypeRef)
    externfn!(fn LLVMHalfType() -> TypeRef)
    externfn!(fn LLVMFloatType() -> TypeRef)
    externfn!(fn LLVMDoubleType() -> TypeRef)
    externfn!(fn LLVMX86FP80Type() -> TypeRef)
    externfn!(fn LLVMFP128Type() -> TypeRef)
    externfn!(fn LLVMPPCFP128Type() -> TypeRef)

    // Function Types
    externfn!(fn LLVMFunctionType(ReturnType:  TypeRef,
                            ParamTypes: *TypeRef,
                            ParamCount:  c_uint,
                            IsVarArg: Bool) -> TypeRef)
    externfn!(fn LLVMIsFunctionVarArg(FunctionTy: TypeRef) -> Bool)
    externfn!(fn LLVMGetReturnType(FunctionTy: TypeRef) -> TypeRef)
    externfn!(fn LLVMCountParamTypes(FunctionTy: TypeRef) -> c_uint)
    externfn!(fn LLVMGetParamTypes(FunctionTy: TypeRef, Dest: *mut TypeRef))

    // Struct Types
    externfn!(fn LLVMStructTypeInContext(C: ContextRef,
                                   ElementTypes: *TypeRef,
                                   ElementCount: c_uint,
                                   Packed: Bool) -> TypeRef)
    externfn!(fn LLVMStructType(ElementTypes: *TypeRef,
                          ElementCount: c_uint,
                          Packed: Bool) -> TypeRef)
    externfn!(fn LLVMStructCreateNamed(C: ContextRef, Name: *c_char) -> TypeRef)
    externfn!(fn LLVMGetStructName(Ty: TypeRef) -> *c_char)
    externfn!(fn LLVMStructSetBody(StructTy: TypeRef,
                             ElementTypes: *TypeRef,
                             ElementCount: c_uint,
                             Packed: Bool))
    externfn!(fn LLVMCountStructElementTypes(StructTy: TypeRef) -> c_uint)
    externfn!(fn LLVMGetStructElementTypes(StructTy: TypeRef, Dest: *mut TypeRef))
    externfn!(fn LLVMIsPackedStruct(StructTy: TypeRef) -> Bool)
    externfn!(fn LLVMIsOpaqueStruct(StructTy:TypeRef) -> Bool)

    // Sequential Types
    externfn!(fn LLVMArrayType(ElementType: TypeRef, ElementCount: c_uint) -> TypeRef)
    externfn!(fn LLVMPointerType(ElementType: TypeRef, AddressSpace: c_uint) -> TypeRef)
    externfn!(fn LLVMVectorType(ElementType: TypeRef, ElementCount: c_uint) -> TypeRef)
    externfn!(fn LLVMGetElementType(Ty: TypeRef) -> TypeRef)
    externfn!(fn LLVMGetArrayLength(ArrayTy: TypeRef) -> c_uint)
    externfn!(fn LLVMGetPointerAddressSpace(PointerTy: TypeRef) -> c_uint)
    externfn!(fn LLVMGetVectorSize(VectorTy: TypeRef) -> c_uint)

    /* Operations on other types */
    externfn!(fn LLVMVoidTypeInContext(C: ContextRef) -> TypeRef)
    externfn!(fn LLVMLabelTypeInContext(C: ContextRef) -> TypeRef)
    externfn!(fn LLVMVoidType() -> TypeRef)
    externfn!(fn LLVMLabelType() -> TypeRef)

}

pub mod value {
    use super::*;
    use std::libc::*;

    // General APIs
    externfn!(fn LLVMTypeOf(Val: ValueRef) -> TypeRef)
    externfn!(fn LLVMGetValueName(Val: ValueRef) -> *c_char)
    externfn!(fn LLVMSetValueName(Val: ValueRef, Name: *c_char))
    externfn!(fn LLVMDumpValue(Val: ValueRef))
    externfn!(fn LLVMReplaceAllUsesWith(OldVal: ValueRef, NewVal: ValueRef))
    externfn!(fn LLVMIsConstant(Val: ValueRef) -> Bool)
    externfn!(fn LLVMIsUndef(Val: ValueRef) -> Bool)

    // Usage
    externfn!(fn LLVMGetFirstUse(Val: ValueRef) -> UseRef)
    externfn!(fn LLVMGetNextUse(U: UseRef) -> UseRef)
    externfn!(fn LLVMGetUser(U: UseRef) -> ValueRef)
    externfn!(fn LLVMGetUsedValue(U: UseRef) -> ValueRef)

    // User Value
    externfn!(fn LLVMGetOperand(Val: ValueRef, Index: c_uint) -> ValueRef)
    externfn!(fn LLVMSetOperand(User: ValueRef, Index: c_uint, Val: ValueRef))
    externfn!(fn LLVMGetNumOperands(Val: ValueRef) -> c_uint)
}

pub mod constant {
    use super::*;
    use std::libc::*;

    // Constants
    externfn!(fn LLVMConstNull(Ty: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstAllOnes(Ty: TypeRef) -> ValueRef)
    externfn!(fn LLVMGetUndef(Ty: TypeRef) -> ValueRef)
    externfn!(fn LLVMIsNull(Val: ValueRef) -> Bool)
    externfn!(fn LLVMConstPointerNull(Ty: TypeRef) -> ValueRef)

    // Scalar Constants
    externfn!(fn LLVMConstInt(IntTy: TypeRef, N: c_ulonglong, SignExtend: Bool) -> ValueRef)
    externfn!(fn LLVMConstIntOfString(IntTy: TypeRef, Text: *c_char, Radix: u8) -> ValueRef)
    externfn!(fn LLVMConstIntOfStringAndSize(IntTy: TypeRef,
                                       Text: *c_char,
                                       SLen: c_uint,
                                       Radix: u8) -> ValueRef)
    externfn!(fn LLVMConstReal(RealTy: TypeRef, N: f64) -> ValueRef)
    externfn!(fn LLVMConstRealOfString(RealTy: TypeRef, Text: *c_char) -> ValueRef)
    externfn!(fn LLVMConstRealOfStringAndSize(RealTy: TypeRef,
                                        Text:  *c_char,
                                        SLen:   c_uint) -> ValueRef)
    externfn!(fn LLVMConstIntGetZExtValue(ConstantVal: ValueRef) -> c_ulonglong)
    externfn!(fn LLVMConstIntGetSExtValue(ConstantVal: ValueRef) -> c_longlong)

    // Composite Constants
    externfn!(fn LLVMConstStringInContext(C: ContextRef,
                                    Str: *c_char,
                                    Length: c_uint,
                                    DontNullTerminate: Bool) -> ValueRef)
    externfn!(fn LLVMConstString(Str: *c_char,
                           Length: c_uint,
                           DontNullTerminate: Bool) -> ValueRef)
    externfn!(fn LLVMConstStructInContext(C: ContextRef,
                                    ConstantVals: *ValueRef,
                                    Count: c_uint,
                                    Packed: Bool) -> ValueRef)
    externfn!(fn LLVMConstStruct(ConstantVals: *ValueRef, Count: c_uint, Packed: Bool) -> ValueRef)
    externfn!(fn LLVMConstArray(ElementTy: TypeRef,
                          ConstantVals: *ValueRef,
                          Length: c_uint) -> ValueRef)
    externfn!(fn LLVMConstNamedStruct(StructTy: TypeRef,
                                ConstantVals: *ValueRef,
                                Count: c_uint) -> ValueRef)
    externfn!(fn LLVMConstVector(ScalarConstantVals: *ValueRef, Size: c_uint) -> ValueRef)

    // Constant Expressions
    externfn!(fn LLVMAlignOf(Ty: TypeRef) -> ValueRef)
    externfn!(fn LLVMSizeOf(Ty: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstNeg(ConstantVal: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstNSWNeg(ConstantVal: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstNUWNeg(ConstantVal: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstFNeg(ConstantVal: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstNot(ConstantVal: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstAdd(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstNSWAdd(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstNUWAdd(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstFAdd(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstSub(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstNSWSub(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstNUWSub(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstFSub(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstMul(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstNSWMul(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstNUWMul(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstFMul(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstUDiv(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstSDiv(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstExactSDiv(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstFDiv(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstURem(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstSRem(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstFRem(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstAnd(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstOr(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstXor(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstShl(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstLShr(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstAShr(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstGEP(ConstantVal: ValueRef,
                        ConstantIndices: *ValueRef,
                        NumIndices: c_uint) -> ValueRef)
    externfn!(fn LLVMConstInBoundsGEP(ConstantVal: ValueRef,
                                ConstantIndices: *ValueRef,
                                NumIndices: c_uint) -> ValueRef)
    externfn!(fn LLVMConstTrunc(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstSExt(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstZExt(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstFPTrunc(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstFPExt(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstUIToFP(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstSIToFP(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstFPToUI(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstFPToSI(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstPtrToInt(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstIntToPtr(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstBitCast(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstZExtOrBitCast(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstSExtOrBitCast(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstTruncOrBitCast(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstPointerCast(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstIntCast(ConstantVal: ValueRef,
                            ToType: TypeRef,
                            isSigned: Bool) -> ValueRef)
    externfn!(fn LLVMConstFPCast(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef)
    externfn!(fn LLVMConstSelect(ConstantCondition: ValueRef,
                           ConstantIfTrue: ValueRef,
                           ConstantIfFalse: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstExtractElement(VectorConstant: ValueRef,
                                   IndexConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstInsertElement(VectorConstant: ValueRef,
                                  ElementValueConstant: ValueRef,
                                  IndexConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstShuffleVector(VectorAConstant: ValueRef,
                                  VectorBConstant: ValueRef,
                                  MaskConstant: ValueRef) -> ValueRef)
    externfn!(fn LLVMConstExtractValue(AggConstant: ValueRef,
                                 IdxList: *c_uint,
                                 NumIdx: c_uint) -> ValueRef)
    externfn!(fn LLVMConstInsertValue(AggConstant: ValueRef,
                                ElementValueConstant: ValueRef,
                                IdxList: *c_uint,
                                NumIdx: c_uint) -> ValueRef)
    externfn!(fn LLVMConstInlineAsm(Ty: TypeRef,
                              AsmString: *c_char,
                              Constraints: *c_char,
                              HasSideEffects: Bool,
                              IsAlignStack: Bool) -> ValueRef)
    externfn!(fn LLVMBlockAddress(F: ValueRef, BB: BasicBlockRef) -> ValueRef)

}

pub mod global {
    use super::*;
    use std::libc::*;

    // Global Values
    externfn!(fn LLVMGetGlobalParent(Global: ValueRef) -> ModuleRef)
    externfn!(fn LLVMIsDeclaration(Global: ValueRef) -> Bool)
    externfn!(fn LLVMGetLinkage(Global: ValueRef) -> c_uint)
    externfn!(fn LLVMSetLinkage(Global: ValueRef, Link: c_uint))
    externfn!(fn LLVMGetSection(Global: ValueRef) -> *c_char)
    externfn!(fn LLVMSetSection(Global: ValueRef, Section: *c_char))
    externfn!(fn LLVMGetVisibility(Global: ValueRef) -> c_uint)
    externfn!(fn LLVMSetVisibility(Global: ValueRef, Viz: c_uint))
    externfn!(fn LLVMGetAlignment(Global: ValueRef) -> c_uint)
    externfn!(fn LLVMSetAlignment(Global: ValueRef, Bytes: c_uint))

    externfn!(fn LLVMAddGlobal(M: ModuleRef, Ty: TypeRef, Name: *c_char) -> ValueRef)
    externfn!(fn LLVMAddGlobalInAddressSpace(M: ModuleRef,
                                       Ty: TypeRef,
                                       Name: *c_char,
                                       AddressSpace: c_uint) -> ValueRef)
    externfn!(fn LLVMGetNamedGlobal(M: ModuleRef, Name: *c_char) -> ValueRef)
    externfn!(fn LLVMGetFirstGlobal(M: ModuleRef) -> ValueRef)
    externfn!(fn LLVMGetLastGlobal(M: ModuleRef) -> ValueRef)
    externfn!(fn LLVMGetNextGlobal(GlobalVar: ValueRef) -> ValueRef)
    externfn!(fn LLVMGetPreviousGlobal(GlobalVar: ValueRef) -> ValueRef)
    externfn!(fn LLVMDeleteGlobal(GlobalVar: ValueRef))
    externfn!(fn LLVMGetInitializer(GlobalVar: ValueRef) -> ValueRef)
    externfn!(fn LLVMSetInitializer(GlobalVar: ValueRef, ConstantVal: ValueRef))
    externfn!(fn LLVMIsThreadLocal(GlobalVar: ValueRef) -> Bool)
    externfn!(fn LLVMSetThreadLocal(GlobalVar: ValueRef, IsThreadLocal: Bool))
    externfn!(fn LLVMIsGlobalConstant(GlobalVar: ValueRef) -> Bool)
    externfn!(fn LLVMSetGlobalConstant(GlobalVar: ValueRef, IsConstant: Bool))
    externfn!(fn LLVMGetThreadLocalMode(GlobalVar: ValueRef) -> ThreadLocalMode)
    externfn!(fn LLVMSetThreadLocalMode(GlobalVar: ValueRef, Mode: ThreadLocalMode))
    externfn!(fn LLVMIsExternallyInitialized(GlobalVar: ValueRef) -> Bool)
    externfn!(fn LLVMSetExternallyInitialized(GlobalVar: ValueRef, IsExtInit: Bool))

    /* Operations on aliases */
    externfn!(fn LLVMAddAlias(M: ModuleRef,
                        Ty: TypeRef,
                        Aliasee: ValueRef,
                        Name: *c_char) -> ValueRef)
}

pub mod function {
    use super::*;
    use std::libc::*;

    /* Operations on functions */
    externfn!(fn LLVMDeleteFunction(Fn: ValueRef))
    externfn!(fn LLVMGetIntrinsicID(Fn: ValueRef) -> c_uint)
    externfn!(fn LLVMGetFunctionCallConv(Fn: ValueRef) -> c_uint)
    externfn!(fn LLVMSetFunctionCallConv(Fn: ValueRef, CC: c_uint))
    externfn!(fn LLVMGetGC(Fn: ValueRef) -> *c_char)
    externfn!(fn LLVMSetGC(Fn: ValueRef, Name: *c_char))
    externfn!(fn LLVMAddFunctionAttr(Fn: ValueRef, PA: c_ulonglong))
    externfn!(fn LLVMGetFunctionAttr(Fn: ValueRef) -> c_ulonglong)
    externfn!(fn LLVMRemoveFunctionAttr(Fn: ValueRef, PA: c_ulonglong))

    /* Operations on parameters */
    externfn!(fn LLVMCountParams(Fn: ValueRef) -> c_uint)
    externfn!(fn LLVMGetParams(Fn: ValueRef, Params: *mut ValueRef))
    externfn!(fn LLVMGetParam(Fn: ValueRef, Index: c_uint) -> ValueRef)
    externfn!(fn LLVMGetParamParent(Inst: ValueRef) -> ValueRef)
    externfn!(fn LLVMGetFirstParam(Fn: ValueRef) -> ValueRef)
    externfn!(fn LLVMGetLastParam(Fn: ValueRef) -> ValueRef)
    externfn!(fn LLVMGetNextParam(Arg: ValueRef) -> ValueRef)
    externfn!(fn LLVMGetPreviousParam(Arg: ValueRef) -> ValueRef)
    externfn!(fn LLVMAddAttribute(Arg: ValueRef, PA: c_ulonglong))
    externfn!(fn LLVMRemoveAttribute(Arg: ValueRef, PA: c_ulonglong))
    externfn!(fn LLVMGetAttribute(Arg: ValueRef) -> c_ulonglong)
    externfn!(fn LLVMSetParamAlignment(Arg: ValueRef, align: c_uint))
}

pub mod metadata {
    use super::*;
    use std::libc::*;

    externfn!(fn LLVMMDStringInContext(C: ContextRef, Str: *c_char, SLen: c_uint) -> ValueRef)
    externfn!(fn LLVMMDString(Str: *c_char, SLen: c_uint) -> ValueRef)
    externfn!(fn LLVMMDNodeInContext(C: ContextRef, Vals: *ValueRef, Count: c_uint) -> ValueRef)
    externfn!(fn LLVMMDNode(Vals: *ValueRef, Count: c_uint) -> ValueRef)
    externfn!(fn LLVMGetMDString(V: ValueRef, Len: *mut c_uint) -> *c_char)
    externfn!(fn LLVMGetMDNodeNumOperands(V: ValueRef) -> c_uint)
    externfn!(fn LLVMGetMDNodeOperands(V: ValueRef, Dest: *mut ValueRef))
}

pub mod bb {

    use super::*;
    use std::libc::*;

    externfn!(fn LLVMBasicBlockAsValue(BB: BasicBlockRef) -> ValueRef)
    externfn!(fn LLVMValueIsBasicBlock(Val: ValueRef) -> Bool)
    externfn!(fn LLVMValueAsBasicBlock(Val: ValueRef) -> BasicBlockRef)
    externfn!(fn LLVMGetBasicBlockParent(BB: BasicBlockRef) -> ValueRef)
    externfn!(fn LLVMGetBasicBlockTerminator(BB: BasicBlockRef) -> ValueRef)
    externfn!(fn LLVMCountBasicBlocks(Fn: ValueRef) -> c_uint)
    externfn!(fn LLVMGetBasicBlocks(Fn: ValueRef, BasicBlocks: *mut BasicBlockRef))
    externfn!(fn LLVMGetFirstBasicBlock(Fn: ValueRef) -> BasicBlockRef)
    externfn!(fn LLVMGetLastBasicBlock(Fn: ValueRef) -> BasicBlockRef)
    externfn!(fn LLVMGetNextBasicBlock(BB: BasicBlockRef) -> BasicBlockRef)
    externfn!(fn LLVMGetPreviousBasicBlock(BB: BasicBlockRef) -> BasicBlockRef)
    externfn!(fn LLVMGetEntryBasicBlock(Fn: ValueRef) -> BasicBlockRef)
    externfn!(fn LLVMAppendBasicBlockInContext(C: ContextRef,
                                         Fn: ValueRef,
                                         Name: *c_char) -> BasicBlockRef)
    externfn!(fn LLVMAppendBasicBlock(Fn: ValueRef, Name: *c_char) -> BasicBlockRef)
    externfn!(fn LLVMInsertBasicBlockInContext(C: ContextRef,
                                         BB: BasicBlockRef,
                                         Name: *c_char) -> BasicBlockRef)
    externfn!(fn LLVMInsertBasicBlock(BB: BasicBlockRef, Name: *c_char) -> BasicBlockRef)
    externfn!(fn LLVMDeleteBasicBlock(BB: BasicBlockRef))
    externfn!(fn LLVMRemoveBasicBlockFromParent(BB: BasicBlockRef))
    externfn!(fn LLVMMoveBasicBlockBefore(BB: BasicBlockRef, MovePos: BasicBlockRef))
    externfn!(fn LLVMMoveBasicBlockAfter(BB: BasicBlockRef, MovePos: BasicBlockRef))
    externfn!(fn LLVMGetFirstInstruction(BB: BasicBlockRef) -> ValueRef)
    externfn!(fn LLVMGetLastInstruction(BB: BasicBlockRef) -> ValueRef)
}

pub mod instruction {
    use super::*;
    use std::libc::*;

    externfn!(fn LLVMHasMetadata(Val: ValueRef) -> c_int)
    externfn!(fn LLVMGetMetadata(Val: ValueRef, KindID: c_uint) -> ValueRef)
    externfn!(fn LLVMSetMetadata(Val: ValueRef, KindId: c_uint, Node: ValueRef))
    externfn!(fn LLVMGetInstructionParent(Inst: ValueRef) -> BasicBlockRef)
    externfn!(fn LLVMGetNextInstruction(Inst: ValueRef) -> ValueRef)
    externfn!(fn LLVMGetPreviousInstruction(Inst: ValueRef) -> ValueRef)
    externfn!(fn LLVMInstructionEraseFromParent(Inst: ValueRef))
    externfn!(fn LLVMGetICmpPredicate(Inst: ValueRef) -> c_uint)
    externfn!(fn LLVMGetSwitchDefaultDest(SwitchInstr: ValueRef) -> BasicBlockRef)

    // Call Sites and Invocations
    externfn!(fn LLVMSetInstructionCallConv(Instr: ValueRef, CC: c_uint))
    externfn!(fn LLVMGetInstructionCallConv(Instr: ValueRef) -> c_uint)
    externfn!(fn LLVMAddInstrAttribute(Instr: ValueRef, index: c_uint, Attr: c_ulonglong))
    externfn!(fn LLVMRemoveInstrAttribute(Instr: ValueRef, index: c_uint, Attr: c_ulonglong))
    externfn!(fn LLVMSetInstrParamAlignment(Instr: ValueRef, index: c_uint, align: c_uint))
    externfn!(fn LLVMIsTailCall(CallInst: ValueRef) -> Bool)
    externfn!(fn LLVMSetTailCall(CallInst: ValueRef, IsTailCall: Bool))

    // Phi nodes
    externfn!(fn LLVMAddIncoming(PhiNode: ValueRef,
                           IncomingValues: *ValueRef,
                           IncomingBlocks: *BasicBlockRef,
                           Count: c_uint))
    externfn!(fn LLVMCountIncoming(PhiNode: ValueRef) -> c_uint)
    externfn!(fn LLVMGetIncomingBlock(PhiNode: ValueRef, Index: c_uint) -> BasicBlockRef)
    externfn!(fn LLVMGetIncomingValue(PhiNode: ValueRef, Index: c_uint) -> ValueRef)
}

pub mod ir_builder {
    use super::*;
    use std::libc::*;

    externfn!(fn LLVMCreateBuilderInContext(C: ContextRef) -> BuilderRef)
    externfn!(fn LLVMCreateBuilder() -> BuilderRef)
    externfn!(fn LLVMPositionBuilder(Builder: BuilderRef, Block: BasicBlockRef, Instr: ValueRef))
    externfn!(fn LLVMPositionBuilderBefore(Builder: BuilderRef, Instr: ValueRef))
    externfn!(fn LLVMPositionBuilderAtEnd(Builder: BuilderRef, Block: BasicBlockRef))
    externfn!(fn LLVMGetInsertBlock(Builder: BuilderRef) -> BasicBlockRef)
    externfn!(fn LLVMClearInsertionPosition(Builder: BuilderRef))
    externfn!(fn LLVMInsertIntoBuilder(Builder: BuilderRef, Instr: ValueRef))
    externfn!(fn LLVMInsertIntoBuilderWithName(Builder: BuilderRef, Instr: ValueRef, Name: *c_char))
    externfn!(fn LLVMDisposeBuilder(Builder: BuilderRef))

    /* Metadata */
    externfn!(fn LLVMSetCurrentDebugLocation(Builder: BuilderRef, L: ValueRef))
    externfn!(fn LLVMGetCurrentDebugLocation(Builder: BuilderRef) -> ValueRef)
    externfn!(fn LLVMSetInstDebugLocation(Builder: BuilderRef, Inst: ValueRef))

    /* Terminators */
    externfn!(fn LLVMBuildRetVoid(B: BuilderRef) -> ValueRef)
    externfn!(fn LLVMBuildRet(B: BuilderRef, V: ValueRef) -> ValueRef)
    externfn!(fn LLVMBuildAggregateRet(B: BuilderRef, RetVals: *ValueRef, N: c_uint) -> ValueRef)
    externfn!(fn LLVMBuildBr(B: BuilderRef, Dest: BasicBlockRef) -> ValueRef)
    externfn!(fn LLVMBuildCondBr(B: BuilderRef,
                           If: ValueRef,
                           Then: BasicBlockRef,
                           Else: BasicBlockRef) -> ValueRef)
    externfn!(fn LLVMBuildSwitch(B: BuilderRef,
                           V: ValueRef,
                           Else: BasicBlockRef,
                           NumCases: c_uint) -> ValueRef)
    externfn!(fn LLVMBuildIndirectBr(B: BuilderRef, Addr: ValueRef, NumDests: c_uint) -> ValueRef)
    externfn!(fn LLVMBuildInvoke(B: BuilderRef,
                           Fn: ValueRef,
                           Args: *ValueRef,
                           NumArgs: c_uint,
                           Then: BasicBlockRef,
                           Catch: BasicBlockRef,
                           Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildLandingPad(B: BuilderRef,
                               Ty: TypeRef,
                               PersFn: ValueRef,
                               NumClauses: c_uint,
                               Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildResume(B: BuilderRef, Exn: ValueRef) -> ValueRef)
    externfn!(fn LLVMBuildUnreachable(B: BuilderRef) -> ValueRef)

    /* Add a case to the switch instruction */
    externfn!(fn LLVMAddCase(switch: ValueRef, OnVal: ValueRef, Dest: BasicBlockRef))

    /* Add a destination to the indirectbr instruction */
    externfn!(fn LLVMAddDestination(indirectBr: ValueRef, Dest: BasicBlockRef))

    /* Add a catch or filter clause to the landingpad instruction */
    externfn!(fn LLVMAddClause(landingPad: ValueRef, ClauseVal: ValueRef))

    /* Set the 'cleanup' flag in the landingpad instruction */
    externfn!(fn LLVMSetCleanup(landingPad: ValueRef, Val: Bool))

    /* Arithmetic */
    externfn!(fn LLVMBuildAdd(B: BuilderRef,
                        LHS: ValueRef,
                        RHS: ValueRef,
                        Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildNSWAdd(B: BuilderRef,
                           LHS: ValueRef,
                           RHS: ValueRef,
                           Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildNUWAdd(B: BuilderRef,
                           LHS: ValueRef,
                           RHS: ValueRef,
                           Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildFAdd(B: BuilderRef,
                         LHS: ValueRef,
                         RHS: ValueRef,
                         Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildSub(B: BuilderRef,
                        LHS: ValueRef,
                        RHS: ValueRef,
                        Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildNSWSub(B: BuilderRef,
                           LHS: ValueRef,
                           RHS: ValueRef,
                           Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildNUWSub(B: BuilderRef,
                           LHS: ValueRef,
                           RHS: ValueRef,
                           Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildFSub(B: BuilderRef,
                         LHS: ValueRef,
                         RHS: ValueRef,
                         Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildMul(B: BuilderRef,
                        LHS: ValueRef,
                        RHS: ValueRef,
                        Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildNSWMul(B: BuilderRef,
                           LHS: ValueRef,
                           RHS: ValueRef,
                           Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildNUWMul(B: BuilderRef,
                           LHS: ValueRef,
                           RHS: ValueRef,
                           Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildFMul(B: BuilderRef,
                         LHS: ValueRef,
                         RHS: ValueRef,
                         Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildUDiv(B: BuilderRef,
                         LHS: ValueRef,
                         RHS: ValueRef,
                         Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildSDiv(B: BuilderRef,
                         LHS: ValueRef,
                         RHS: ValueRef,
                         Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildExactSDiv(B: BuilderRef,
                              LHS: ValueRef,
                              RHS: ValueRef,
                              Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildFDiv(B: BuilderRef,
                         LHS: ValueRef,
                         RHS: ValueRef,
                         Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildURem(B: BuilderRef,
                         LHS: ValueRef,
                         RHS: ValueRef,
                         Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildSRem(B: BuilderRef,
                         LHS: ValueRef,
                         RHS: ValueRef,
                         Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildFRem(B: BuilderRef,
                         LHS: ValueRef,
                         RHS: ValueRef,
                         Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildShl(B: BuilderRef,
                        LHS: ValueRef,
                        RHS: ValueRef,
                        Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildLShr(B: BuilderRef,
                         LHS: ValueRef,
                         RHS: ValueRef,
                         Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildAShr(B: BuilderRef,
                         LHS: ValueRef,
                         RHS: ValueRef,
                         Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildAnd(B: BuilderRef,
                        LHS: ValueRef,
                        RHS: ValueRef,
                        Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildOr(B: BuilderRef,
                       LHS: ValueRef,
                       RHS: ValueRef,
                       Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildXor(B: BuilderRef,
                        LHS: ValueRef,
                        RHS: ValueRef,
                        Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildBinOp(B: BuilderRef,
                          Op: c_uint,
                          LHS: ValueRef,
                          RHS: ValueRef,
                          Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildNeg(B: BuilderRef, V: ValueRef, Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildNSWNeg(B: BuilderRef, V: ValueRef,
                           Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildNUWNeg(B: BuilderRef, V: ValueRef,
                           Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildFNeg(B: BuilderRef, V: ValueRef, Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildNot(B: BuilderRef, V: ValueRef, Name: *c_char) -> ValueRef)

    /* Memory */
    externfn!(fn LLVMBuildMalloc(B: BuilderRef, Ty: TypeRef, Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildArrayMalloc(B: BuilderRef,
                                Ty: TypeRef,
                                Val: ValueRef,
                                Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildAlloca(B: BuilderRef, Ty: TypeRef, Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildArrayAlloca(B: BuilderRef,
                                Ty: TypeRef,
                                Val: ValueRef,
                                Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildFree(B: BuilderRef,
                         PointerVal: ValueRef) -> ValueRef)
    externfn!(fn LLVMBuildLoad(B: BuilderRef,
                         PointerVal: ValueRef,
                         Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildStore(B: BuilderRef,
                          Val: ValueRef,
                          Ptr: ValueRef) -> ValueRef)
    externfn!(fn LLVMBuildGEP(B: BuilderRef,
                        pointer: ValueRef,
                        Indices: *ValueRef,
                        NumIndices: c_uint,
                        Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildInBoundsGEP(B: BuilderRef,
                                pointer: ValueRef,
                                Indices: *ValueRef,
                                NumIndices: c_uint,
                                Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildStructGEP(B: BuilderRef,
                              pointer: ValueRef,
                              Idx: c_uint,
                              Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildGlobalString(B: BuilderRef,
                                 Str: *c_char,
                                 Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildGlobalStringPtr(B: BuilderRef,
                                    Str: *c_char,
                                    Name: *c_char) -> ValueRef)
    externfn!(fn LLVMGetVolatile(MemoryAccessInst: ValueRef) -> Bool)
    externfn!(fn LLVMSetVolatile(MemoryAccessInst: ValueRef, IsVolatile: Bool))

    /* Casts */
    externfn!(fn LLVMBuildTrunc(B: BuilderRef,
                          Val: ValueRef,
                          DestTy: TypeRef,
                          Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildZExt(B: BuilderRef,
                         Val: ValueRef,
                         DestTy: TypeRef,
                         Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildSExt(B: BuilderRef,
                         Val: ValueRef,
                         DestTy: TypeRef,
                         Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildFPToUI(B: BuilderRef,
                           Val: ValueRef,
                           DestTy: TypeRef,
                           Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildFPToSI(B: BuilderRef,
                           Val: ValueRef,
                           DestTy: TypeRef,
                           Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildUIToFP(B: BuilderRef,
                           Val: ValueRef,
                           DestTy: TypeRef,
                           Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildSIToFP(B: BuilderRef,
                           Val: ValueRef,
                           DestTy: TypeRef,
                           Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildFPTrunc(B: BuilderRef,
                            Val: ValueRef,
                            DestTy: TypeRef,
                            Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildFPExt(B: BuilderRef,
                          Val: ValueRef,
                          DestTy: TypeRef,
                          Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildPtrToInt(B: BuilderRef,
                             Val: ValueRef,
                             DestTy: TypeRef,
                             Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildIntToPtr(B: BuilderRef,
                             Val: ValueRef,
                             DestTy: TypeRef,
                             Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildBitCast(B: BuilderRef,
                            Val: ValueRef,
                            DestTy: TypeRef,
                            Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildZExtOrBitCast(B: BuilderRef,
                                  Val: ValueRef,
                                  DestTy: TypeRef,
                                  Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildSExtOrBitCast(B: BuilderRef,
                                  Val: ValueRef,
                                  DestTy: TypeRef,
                                  Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildTruncOrBitCast(B: BuilderRef,
                                   Val: ValueRef,
                                   DestTy: TypeRef,
                                   Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildCast(B: BuilderRef,
                         Op: c_uint,
                         Val: ValueRef,
                         DestTy: TypeRef,
                         Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildPointerCast(B: BuilderRef,
                                Val: ValueRef,
                                DestTy: TypeRef,
                                Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildIntCast(B: BuilderRef,
                            Val: ValueRef, /*Signed cast!*/
                            DestTy: TypeRef,
                            Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildFPCast(B: BuilderRef,
                           Val: ValueRef,
                           DestTy: TypeRef,
                           Name: *c_char) -> ValueRef)

    /* Comparisons */
    externfn!(fn LLVMBuildICmp(B: BuilderRef,
                         Op: c_uint,
                         LHS: ValueRef,
                         RHS: ValueRef,
                         Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildFCmp(B: BuilderRef,
                         Op: c_uint,
                         LHS: ValueRef,
                         RHS: ValueRef,
                         Name: *c_char) -> ValueRef)

    /* Miscellaneous instructions */
    externfn!(fn LLVMBuildPhi(B: BuilderRef, Ty: TypeRef, Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildCall(B: BuilderRef,
                         Fn: ValueRef,
                         Args: *ValueRef,
                         NumArgs: c_uint,
                         Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildSelect(B: BuilderRef,
                           If: ValueRef,
                           Then: ValueRef,
                           Else: ValueRef,
                           Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildVAArg(B: BuilderRef,
                          List: ValueRef,
                          Ty: TypeRef,
                          Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildExtractElement(B: BuilderRef,
                                   VecVal: ValueRef,
                                   Index: ValueRef,
                                   Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildInsertElement(B: BuilderRef,
                                  VecVal: ValueRef,
                                  EltVal: ValueRef,
                                  Index: ValueRef,
                                  Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildShuffleVector(B: BuilderRef,
                                  V1: ValueRef,
                                  V2: ValueRef,
                                  Mask: ValueRef,
                                  Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildExtractValue(B: BuilderRef,
                                 AggVal: ValueRef,
                                 Index: c_uint,
                                 Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildInsertValue(B: BuilderRef,
                                AggVal: ValueRef,
                                EltVal: ValueRef,
                                Index: c_uint,
                                Name: *c_char) -> ValueRef)

    externfn!(fn LLVMBuildIsNull(B: BuilderRef,
                           Val: ValueRef,
                           Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildIsNotNull(B: BuilderRef,
                              Val: ValueRef,
                              Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildPtrDiff(B: BuilderRef,
                            LHS: ValueRef,
                            RHS: ValueRef,
                            Name: *c_char) -> ValueRef)
    externfn!(fn LLVMBuildAtomicRMW(B: BuilderRef,
                              op: c_uint,
                              PTR: ValueRef,
                              Val: ValueRef,
                              ordering: c_uint,
                              singleThread: Bool) -> ValueRef)
}

pub mod mod_prov {
    use super::*;

    externfn!(fn LLVMCreateModuleProviderForExistingModule(M:ModuleRef) -> ModuleProviderRef)
    externfn!(fn LLVMDisposeModuleProvider(M: ModuleProviderRef))
}

pub mod mem_buffer {
    use super::*;
    use std::libc::*;

    externfn!(fn LLVMCreateMemoryBufferWithContentsOfFile(Path: *c_char,
                                                    OutMemBuf: *mut MemoryBufferRef,
                                                    OutMessage: *mut *c_char) -> Bool)
    externfn!(fn LLVMCreateMemoryBufferWithSTDIN(OutMemBuf: *mut MemoryBufferRef,
                                           OutMessage: *mut *c_char) -> Bool)
    externfn!(fn LLVMCreateMemoryBufferWithMemoryRange(
        InputData: *c_char,
        InputDataLength: size_t,
        BufferName: *c_char,
        RequiresNullTerminator: Bool) -> MemoryBufferRef)
    externfn!(fn LLVMCreateMemoryBufferWithMemoryRangeCopy(
        InputData: *c_char,
        InputDataLength: size_t,
        BufferName: *c_char) -> MemoryBufferRef)
    externfn!(fn LLVMGetBufferStart(MemBuf: MemoryBufferRef) -> *char)
    externfn!(fn LLVMGetBufferSize(MemBuf: MemoryBufferRef) -> size_t)
    externfn!(fn LLVMDisposeMemoryBuffer(MemBuf: MemoryBufferRef))
}

pub mod passes {
    use super::*;

    externfn!(fn LLVMGetGlobalPassRegistry() -> PassRegistryRef)
    externfn!(fn LLVMCreatePassManager() -> PassManagerRef)
    externfn!(fn LLVMCreateFunctionPassManagerForModule(M: ModuleRef) -> PassManagerRef)
    externfn!(fn LLVMRunPassManager(PM: PassManagerRef, M: ModuleRef) -> Bool)
    externfn!(fn LLVMInitializeFunctionPassManager(FPM: PassManagerRef) -> Bool)
    externfn!(fn LLVMRunFunctionPassManager(FPM: PassManagerRef, F: ValueRef) -> Bool)
    externfn!(fn LLVMFinalizeFunctionPassManager(FPM: PassManagerRef) -> Bool)
    externfn!(fn LLVMDisposePassManager(PM: PassManagerRef))
}
