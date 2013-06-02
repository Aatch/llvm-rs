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

pub extern {
    #[fast_ffi]
    pub fn LLVMInitializeCore(R: PassRegistryRef);
    #[fast_ffi]
    pub fn LLVMShutdown();

    #[fast_ffi]
    pub fn LLVMDisposeMessage(Message: *c_char);

    // Threading
    #[fast_ffi]
    pub fn LLVMStartMultithreaded() -> Bool;
    #[fast_ffi]
    pub fn LLVMStopMultithreaded();
    #[fast_ffi]
    pub fn LLVMIsMultithreaded() -> Bool;
}

pub mod context {
    use super::*;
    use std::libc::*;

    pub extern {
        #[fast_ffi]
        pub fn LLVMContextCreate() -> ContextRef;
        #[fast_ffi]
        pub fn LLVMGetGlobalContext() -> ContextRef;
        #[fast_ffi]
        pub fn LLVMContextDispose(C: ContextRef);
        #[fast_ffi]
        pub fn LLVMGetMDKindIDInContext(C:ContextRef, name:*c_char, SLen:c_uint) -> c_uint;
    }
}

pub mod module {
    use super::*;
    use std::libc::*;

    pub extern {
        #[fast_ffi]
        pub fn LLVMModuleCreateWithName(MID:*c_char) -> ModuleRef;
        #[fast_ffi]
        pub fn LLVMModuleCreateWithNameInContext(MID:*c_char, C:ContextRef) -> ModuleRef;
        #[fast_ffi]
        pub fn LLVMDisposeModule(M:ModuleRef);
        #[fast_ffi]
        pub fn LLVMGetDataLayout(M:ModuleRef) -> *c_char;
        #[fast_ffi]
        pub fn LLVMSetDataLayout(M:ModuleRef,Triple:*c_char);
        #[fast_ffi]
        pub fn LLVMGetTarget(M:ModuleRef) -> *c_char;
        #[fast_ffi]
        pub fn LLVMSetTarget(M:ModuleRef,Triple:*c_char);
        #[fast_ffi]
        pub fn LLVMDumpModule(M:ModuleRef);
        #[fast_ffi]
        pub fn LLVMPrintModuleToFile(M:ModuleRef, Filename:*c_char, ErrMsg:*mut *c_char) -> Bool;
        #[fast_ffi]
        pub fn LLVMSetModuleInlineAsm(M:ModuleRef, Asm:*c_char);
        #[fast_ffi]
        pub fn LLVMGetModuleContext(M:ModuleRef) -> ContextRef;
        #[fast_ffi]
        pub fn LLVMGetTypeByName(M:ModuleRef, Name:*c_char) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMGetNamedMetadataNumOperands(M:ModuleRef, name:*c_char) -> c_uint;
        #[fast_ffi]
        pub fn LLVMGetNamedMetadataOperands(M:ModuleRef, name:*c_char, Dest: *mut ValueRef);
        #[fast_ffi]
        pub fn LLVMAddNamedMetadataOperand(M:ModuleRef, name:*c_char, Val:ValueRef);
        #[fast_ffi]
        pub fn LLVMAddFunction(M:ModuleRef, Name:*c_char, FunctionTy:TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetNamedFunction(M:ModuleRef, Name:*c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetFirstFunction(M:ModuleRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetLastFunction(M:ModuleRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetNextFunction(Fn:ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetPreviousFunction(Fn:ValueRef) -> ValueRef;
    }
}

pub mod types {
    use super::*;
    use std::libc::*;

    pub extern {
        #[fast_ffi]
        pub fn LLVMGetTypeKind(Ty:TypeRef) -> TypeKind;
        #[fast_ffi]
        pub fn LLVMTypeIsSized(Ty:TypeRef) -> Bool;
        #[fast_ffi]
        pub fn LLVMGetTypeContext(Ty:TypeRef) -> ContextRef;

        // Integer Types
        #[fast_ffi]
        pub fn LLVMInt1TypeInContext(C:ContextRef) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMInt8TypeInContext(C:ContextRef) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMInt16TypeInContext(C:ContextRef) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMInt32TypeInContext(C:ContextRef) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMInt64TypeInContext(C:ContextRef) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMIntTypeInContext(C:ContextRef, NumBits:c_uint) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMInt1Type() -> TypeRef;
        #[fast_ffi]
        pub fn LLVMInt8Type() -> TypeRef;
        #[fast_ffi]
        pub fn LLVMInt16Type() -> TypeRef;
        #[fast_ffi]
        pub fn LLVMInt32Type() -> TypeRef;
        #[fast_ffi]
        pub fn LLVMInt64Type() -> TypeRef;
        #[fast_ffi]
        pub fn LLVMIntType(NumBits:c_uint) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMGetIntTypeWidth(IntegerTy:TypeRef) -> c_uint;

        // Floating Point Types
        #[fast_ffi]
        pub fn LLVMHalfTypeInContext(C:ContextRef) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMFloatTypeInContext(C: ContextRef) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMDoubleTypeInContext(C: ContextRef) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMX86FP80TypeInContext(C: ContextRef) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMFP128TypeInContext(C: ContextRef) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMPPCFP128TypeInContext(C: ContextRef) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMHalfType() -> TypeRef;
        #[fast_ffi]
        pub fn LLVMFloatType() -> TypeRef;
        #[fast_ffi]
        pub fn LLVMDoubleType() -> TypeRef;
        #[fast_ffi]
        pub fn LLVMX86FP80Type() -> TypeRef;
        #[fast_ffi]
        pub fn LLVMFP128Type() -> TypeRef;
        #[fast_ffi]
        pub fn LLVMPPCFP128Type() -> TypeRef;

        // Function Types
        #[fast_ffi]
        pub fn LLVMFunctionType(ReturnType:  TypeRef,
                                ParamTypes: *TypeRef,
                                ParamCount:  c_uint,
                                IsVarArg: Bool) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMIsFunctionVarArg(FunctionTy: TypeRef) -> Bool;
        #[fast_ffi]
        pub fn LLVMGetReturnType(FunctionTy: TypeRef) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMCountParamTypes(FunctionTy: TypeRef) -> c_uint;
        #[fast_ffi]
        pub fn LLVMGetParamTypes(FunctionTy: TypeRef, Dest: *mut TypeRef);

        // Struct Types
        #[fast_ffi]
        pub fn LLVMStructTypeInContext(C: ContextRef,
                                       ElementTypes: *TypeRef,
                                       ElementCount: c_uint,
                                       Packed: Bool) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMStructType(ElementTypes: *TypeRef,
                              ElementCount: c_uint,
                              Packed: Bool) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMStructCreateNamed(C: ContextRef, Name: *c_char) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMGetStructName(Ty: TypeRef) -> *c_char;
        #[fast_ffi]
        pub fn LLVMStructSetBody(StructTy: TypeRef,
                                 ElementTypes: *TypeRef,
                                 ElementCount: c_uint,
                                 Packed: Bool);
        #[fast_ffi]
        pub fn LLVMCountStructElementTypes(StructTy: TypeRef) -> c_uint;
        #[fast_ffi]
        pub fn LLVMGetStructElementTypes(StructTy: TypeRef, Dest: *mut TypeRef);
        #[fast_ffi]
        pub fn LLVMIsPackedStruct(StructTy: TypeRef) -> Bool;
        #[fast_ffi]
        pub fn LLVMIsOpaqueStruct(StructTy:TypeRef) -> Bool;

        // Sequential Types
        #[fast_ffi]
        pub fn LLVMArrayType(ElementType: TypeRef, ElementCount: c_uint) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMPointerType(ElementType: TypeRef, AddressSpace: c_uint) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMVectorType(ElementType: TypeRef, ElementCount: c_uint) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMGetElementType(Ty: TypeRef) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMGetArrayLength(ArrayTy: TypeRef) -> c_uint;
        #[fast_ffi]
        pub fn LLVMGetPointerAddressSpace(PointerTy: TypeRef) -> c_uint;
        #[fast_ffi]
        pub fn LLVMGetVectorSize(VectorTy: TypeRef) -> c_uint;

        /* Operations on other types */
        #[fast_ffi]
        pub fn LLVMVoidTypeInContext(C: ContextRef) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMLabelTypeInContext(C: ContextRef) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMVoidType() -> TypeRef;
        #[fast_ffi]
        pub fn LLVMLabelType() -> TypeRef;

    }
}

pub mod value {
    use super::*;
    use std::libc::*;

    pub extern {

        // General APIs
        #[fast_ffi]
        pub fn LLVMTypeOf(Val: ValueRef) -> TypeRef;
        #[fast_ffi]
        pub fn LLVMGetValueName(Val: ValueRef) -> *c_char;
        #[fast_ffi]
        pub fn LLVMSetValueName(Val: ValueRef, Name: *c_char);
        #[fast_ffi]
        pub fn LLVMDumpValue(Val: ValueRef);
        #[fast_ffi]
        pub fn LLVMReplaceAllUsesWith(OldVal: ValueRef, NewVal: ValueRef);
        #[fast_ffi]
        pub fn LLVMIsConstant(Val: ValueRef) -> Bool;
        #[fast_ffi]
        pub fn LLVMIsUndef(Val: ValueRef) -> Bool;

        // Usage
        #[fast_ffi]
        pub fn LLVMGetFirstUse(Val: ValueRef) -> UseRef;
        #[fast_ffi]
        pub fn LLVMGetNextUse(U: UseRef) -> UseRef;
        #[fast_ffi]
        pub fn LLVMGetUser(U: UseRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetUsedValue(U: UseRef) -> ValueRef;

        // User Value
        #[fast_ffi]
        pub fn LLVMGetOperand(Val: ValueRef, Index: c_uint) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMSetOperand(User: ValueRef, Index: c_uint, Val: ValueRef);
        #[fast_ffi]
        pub fn LLVMGetNumOperands(Val: ValueRef) -> c_uint;
    }
}

pub mod constant {
    use super::*;
    use std::libc::*;

    pub extern {
        // Constants
        #[fast_ffi]
        pub fn LLVMConstNull(Ty: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstAllOnes(Ty: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetUndef(Ty: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMIsNull(Val: ValueRef) -> Bool;
        #[fast_ffi]
        pub fn LLVMConstPointerNull(Ty: TypeRef) -> ValueRef;

        // Scalar Constants
        #[fast_ffi]
        pub fn LLVMConstInt(IntTy: TypeRef, N: c_ulonglong, SignExtend: Bool) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstIntOfString(IntTy: TypeRef, Text: *c_char, Radix: u8) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstIntOfStringAndSize(IntTy: TypeRef,
                                           Text: *c_char,
                                           SLen: c_uint,
                                           Radix: u8) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstReal(RealTy: TypeRef, N: f64) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstRealOfString(RealTy: TypeRef, Text: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstRealOfStringAndSize(RealTy: TypeRef,
                                            Text:  *c_char,
                                            SLen:   c_uint) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstIntGetZExtValue(ConstantVal: ValueRef) -> c_ulonglong;
        #[fast_ffi]
        pub fn LLVMConstIntGetSExtValue(ConstantVal: ValueRef) -> c_longlong;

        // Composite Constants
        #[fast_ffi]
        pub fn LLVMConstStringInContext(C: ContextRef,
                                        Str: *c_char,
                                        Length: c_uint,
                                        DontNullTerminate: Bool) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstString(Str: *c_char,
                               Length: c_uint,
                               DontNullTerminate: Bool) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstStructInContext(C: ContextRef,
                                        ConstantVals: *ValueRef,
                                        Count: c_uint,
                                        Packed: Bool) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstStruct(ConstantVals: *ValueRef, Count: c_uint, Packed: Bool) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstArray(ElementTy: TypeRef,
                              ConstantVals: *ValueRef,
                              Length: c_uint) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstNamedStruct(StructTy: TypeRef,
                                    ConstantVals: *ValueRef,
                                    Count: c_uint) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstVector(ScalarConstantVals: *ValueRef, Size: c_uint) -> ValueRef;

        // Constant Expressions
        #[fast_ffi]
        pub fn LLVMAlignOf(Ty: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMSizeOf(Ty: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstNeg(ConstantVal: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstNSWNeg(ConstantVal: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstNUWNeg(ConstantVal: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstFNeg(ConstantVal: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstNot(ConstantVal: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstAdd(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstNSWAdd(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstNUWAdd(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstFAdd(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstSub(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstNSWSub(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstNUWSub(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstFSub(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstMul(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstNSWMul(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstNUWMul(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstFMul(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstUDiv(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstSDiv(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstExactSDiv(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstFDiv(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstURem(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstSRem(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstFRem(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstAnd(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstOr(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstXor(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstShl(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstLShr(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstAShr(LHSConstant: ValueRef, RHSConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstGEP(ConstantVal: ValueRef,
                            ConstantIndices: *ValueRef,
                            NumIndices: c_uint) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstInBoundsGEP(ConstantVal: ValueRef,
                                    ConstantIndices: *ValueRef,
                                    NumIndices: c_uint) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstTrunc(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstSExt(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstZExt(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstFPTrunc(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstFPExt(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstUIToFP(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstSIToFP(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstFPToUI(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstFPToSI(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstPtrToInt(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstIntToPtr(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstBitCast(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstZExtOrBitCast(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstSExtOrBitCast(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstTruncOrBitCast(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstPointerCast(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstIntCast(ConstantVal: ValueRef,
                                ToType: TypeRef,
                                isSigned: Bool) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstFPCast(ConstantVal: ValueRef, ToType: TypeRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstSelect(ConstantCondition: ValueRef,
                               ConstantIfTrue: ValueRef,
                               ConstantIfFalse: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstExtractElement(VectorConstant: ValueRef,
                                       IndexConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstInsertElement(VectorConstant: ValueRef,
                                      ElementValueConstant: ValueRef,
                                      IndexConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstShuffleVector(VectorAConstant: ValueRef,
                                      VectorBConstant: ValueRef,
                                      MaskConstant: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstExtractValue(AggConstant: ValueRef,
                                     IdxList: *c_uint,
                                     NumIdx: c_uint) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstInsertValue(AggConstant: ValueRef,
                                    ElementValueConstant: ValueRef,
                                    IdxList: *c_uint,
                                    NumIdx: c_uint) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMConstInlineAsm(Ty: TypeRef,
                                  AsmString: *c_char,
                                  Constraints: *c_char,
                                  HasSideEffects: Bool,
                                  IsAlignStack: Bool) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBlockAddress(F: ValueRef, BB: BasicBlockRef) -> ValueRef;

    }
}

pub mod global {
    use super::*;
    use std::libc::*;

    pub extern {
        // Global Values
        #[fast_ffi]
        pub fn LLVMGetGlobalParent(Global: ValueRef) -> ModuleRef;
        #[fast_ffi]
        pub fn LLVMIsDeclaration(Global: ValueRef) -> Bool;
        #[fast_ffi]
        pub fn LLVMGetLinkage(Global: ValueRef) -> c_uint;
        #[fast_ffi]
        pub fn LLVMSetLinkage(Global: ValueRef, Link: c_uint);
        #[fast_ffi]
        pub fn LLVMGetSection(Global: ValueRef) -> *c_char;
        #[fast_ffi]
        pub fn LLVMSetSection(Global: ValueRef, Section: *c_char);
        #[fast_ffi]
        pub fn LLVMGetVisibility(Global: ValueRef) -> c_uint;
        #[fast_ffi]
        pub fn LLVMSetVisibility(Global: ValueRef, Viz: c_uint);
        #[fast_ffi]
        pub fn LLVMGetAlignment(Global: ValueRef) -> c_uint;
        #[fast_ffi]
        pub fn LLVMSetAlignment(Global: ValueRef, Bytes: c_uint);

        #[fast_ffi]
        pub fn LLVMAddGlobal(M: ModuleRef, Ty: TypeRef, Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMAddGlobalInAddressSpace(M: ModuleRef,
                                           Ty: TypeRef,
                                           Name: *c_char,
                                           AddressSpace: c_uint) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetNamedGlobal(M: ModuleRef, Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetFirstGlobal(M: ModuleRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetLastGlobal(M: ModuleRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetNextGlobal(GlobalVar: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetPreviousGlobal(GlobalVar: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMDeleteGlobal(GlobalVar: ValueRef);
        #[fast_ffi]
        pub fn LLVMGetInitializer(GlobalVar: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMSetInitializer(GlobalVar: ValueRef, ConstantVal: ValueRef);
        #[fast_ffi]
        pub fn LLVMIsThreadLocal(GlobalVar: ValueRef) -> Bool;
        #[fast_ffi]
        pub fn LLVMSetThreadLocal(GlobalVar: ValueRef, IsThreadLocal: Bool);
        #[fast_ffi]
        pub fn LLVMIsGlobalConstant(GlobalVar: ValueRef) -> Bool;
        #[fast_ffi]
        pub fn LLVMSetGlobalConstant(GlobalVar: ValueRef, IsConstant: Bool);
        pub fn LLVMGetThreadLocalMode(GlobalVar: ValueRef) -> ThreadLocalMode;
        pub fn LLVMSetThreadLocalMode(GlobalVar: ValueRef, Mode: ThreadLocalMode);
        #[fast_ffi]
        pub fn LLVMIsExternallyInitialized(GlobalVar: ValueRef) -> Bool;
        #[fast_ffi]
        pub fn LLVMSetExternallyInitialized(GlobalVar: ValueRef, IsExtInit: Bool);

        /* Operations on aliases */
        #[fast_ffi]
        pub fn LLVMAddAlias(M: ModuleRef,
                            Ty: TypeRef,
                            Aliasee: ValueRef,
                            Name: *c_char) -> ValueRef;
    }
}

pub mod function {
    use super::*;
    use std::libc::*;

    pub extern {
        /* Operations on functions */
        #[fast_ffi]
        pub fn LLVMDeleteFunction(Fn: ValueRef);
        #[fast_ffi]
        pub fn LLVMGetIntrinsicID(Fn: ValueRef) -> c_uint;
        #[fast_ffi]
        pub fn LLVMGetFunctionCallConv(Fn: ValueRef) -> c_uint;
        #[fast_ffi]
        pub fn LLVMSetFunctionCallConv(Fn: ValueRef, CC: c_uint);
        #[fast_ffi]
        pub fn LLVMGetGC(Fn: ValueRef) -> *c_char;
        #[fast_ffi]
        pub fn LLVMSetGC(Fn: ValueRef, Name: *c_char);
        #[fast_ffi]
        pub fn LLVMAddFunctionAttr(Fn: ValueRef, PA: c_ulonglong);
        #[fast_ffi]
        pub fn LLVMGetFunctionAttr(Fn: ValueRef) -> c_ulonglong;
        #[fast_ffi]
        pub fn LLVMRemoveFunctionAttr(Fn: ValueRef, PA: c_ulonglong);

        /* Operations on parameters */
        #[fast_ffi]
        pub fn LLVMCountParams(Fn: ValueRef) -> c_uint;
        #[fast_ffi]
        pub fn LLVMGetParams(Fn: ValueRef, Params: *mut ValueRef);
        #[fast_ffi]
        pub fn LLVMGetParam(Fn: ValueRef, Index: c_uint) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetParamParent(Inst: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetFirstParam(Fn: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetLastParam(Fn: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetNextParam(Arg: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetPreviousParam(Arg: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMAddAttribute(Arg: ValueRef, PA: c_ulonglong);
        #[fast_ffi]
        pub fn LLVMRemoveAttribute(Arg: ValueRef, PA: c_ulonglong);
        #[fast_ffi]
        pub fn LLVMGetAttribute(Arg: ValueRef) -> c_ulonglong;
        #[fast_ffi]
        pub fn LLVMSetParamAlignment(Arg: ValueRef, align: c_uint);

    }
}

pub mod metadata {
    use super::*;
    use std::libc::*;

    pub extern {
        #[fast_ffi]
        pub fn LLVMMDStringInContext(C: ContextRef, Str: *c_char, SLen: c_uint) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMMDString(Str: *c_char, SLen: c_uint) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMMDNodeInContext(C: ContextRef, Vals: *ValueRef, Count: c_uint) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMMDNode(Vals: *ValueRef, Count: c_uint) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetMDString(V: ValueRef, Len: *mut c_uint) -> *c_char;
        #[fast_ffi]
        pub fn LLVMGetMDNodeNumOperands(V: ValueRef) -> c_uint;
        #[fast_ffi]
        pub fn LLVMGetMDNodeOperands(V: ValueRef, Dest: *mut ValueRef);
    }
}

pub mod bb {

    use super::*;
    use std::libc::*;

    pub extern {
        #[fast_ffi]
        pub fn LLVMBasicBlockAsValue(BB: BasicBlockRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMValueIsBasicBlock(Val: ValueRef) -> Bool;
        #[fast_ffi]
        pub fn LLVMValueAsBasicBlock(Val: ValueRef) -> BasicBlockRef;
        #[fast_ffi]
        pub fn LLVMGetBasicBlockParent(BB: BasicBlockRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetBasicBlockTerminator(BB: BasicBlockRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMCountBasicBlocks(Fn: ValueRef) -> c_uint;
        #[fast_ffi]
        pub fn LLVMGetBasicBlocks(Fn: ValueRef, BasicBlocks: *mut BasicBlockRef);
        #[fast_ffi]
        pub fn LLVMGetFirstBasicBlock(Fn: ValueRef) -> BasicBlockRef;
        #[fast_ffi]
        pub fn LLVMGetLastBasicBlock(Fn: ValueRef) -> BasicBlockRef;
        #[fast_ffi]
        pub fn LLVMGetNextBasicBlock(BB: BasicBlockRef) -> BasicBlockRef;
        #[fast_ffi]
        pub fn LLVMGetPreviousBasicBlock(BB: BasicBlockRef) -> BasicBlockRef;
        #[fast_ffi]
        pub fn LLVMGetEntryBasicBlock(Fn: ValueRef) -> BasicBlockRef;
        #[fast_ffi]
        pub fn LLVMAppendBasicBlockInContext(C: ContextRef,
                                             Fn: ValueRef,
                                             Name: *c_char) -> BasicBlockRef;
        #[fast_ffi]
        pub fn LLVMAppendBasicBlock(Fn: ValueRef, Name: *c_char) -> BasicBlockRef;
        #[fast_ffi]
        pub fn LLVMInsertBasicBlockInContext(C: ContextRef,
                                             BB: BasicBlockRef,
                                             Name: *c_char) -> BasicBlockRef;
        #[fast_ffi]
        pub fn LLVMInsertBasicBlock(BB: BasicBlockRef, Name: *c_char) -> BasicBlockRef;
        #[fast_ffi]
        pub fn LLVMDeleteBasicBlock(BB: BasicBlockRef);
        #[fast_ffi]
        pub fn LLVMRemoveBasicBlockFromParent(BB: BasicBlockRef);
        #[fast_ffi]
        pub fn LLVMMoveBasicBlockBefore(BB: BasicBlockRef, MovePos: BasicBlockRef);
        #[fast_ffi]
        pub fn LLVMMoveBasicBlockAfter(BB: BasicBlockRef, MovePos: BasicBlockRef);
        #[fast_ffi]
        pub fn LLVMGetFirstInstruction(BB: BasicBlockRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetLastInstruction(BB: BasicBlockRef) -> ValueRef;
    }
}

pub mod instruction {
    use super::*;
    use std::libc::*;

    pub extern {
        #[fast_ffi]
        pub fn LLVMHasMetadata(Val: ValueRef) -> c_int;
        #[fast_ffi]
        pub fn LLVMGetMetadata(Val: ValueRef, KindID: c_uint) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMSetMetadata(Val: ValueRef, KindId: c_uint, Node: ValueRef);
        #[fast_ffi]
        pub fn LLVMGetInstructionParent(Inst: ValueRef) -> BasicBlockRef;
        #[fast_ffi]
        pub fn LLVMGetNextInstruction(Inst: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetPreviousInstruction(Inst: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMInstructionEraseFromParent(Inst: ValueRef);
        pub fn LLVMGetICmpPredicate(Inst: ValueRef) -> c_uint;
        #[fast_ffi]
        pub fn LLVMGetSwitchDefaultDest(SwitchInstr: ValueRef) -> BasicBlockRef;

        // Call Sites and Invocations
        #[fast_ffi]
        pub fn LLVMSetInstructionCallConv(Instr: ValueRef, CC: c_uint);
        #[fast_ffi]
        pub fn LLVMGetInstructionCallConv(Instr: ValueRef) -> c_uint;
        #[fast_ffi]
        pub fn LLVMAddInstrAttribute(Instr: ValueRef, index: c_uint, Attr: c_uint);
        #[fast_ffi]
        pub fn LLVMRemoveInstrAttribute(Instr: ValueRef, index: c_uint, Attr: c_uint);
        #[fast_ffi]
        pub fn LLVMSetInstrParamAlignment(Instr: ValueRef, index: c_uint, align: c_uint);
        #[fast_ffi]
        pub fn LLVMIsTailCall(CallInst: ValueRef) -> Bool;
        #[fast_ffi]
        pub fn LLVMSetTailCall(CallInst: ValueRef, IsTailCall: Bool);

        // Phi nodes
        #[fast_ffi]
        pub fn LLVMAddIncoming(PhiNode: ValueRef,
                               IncomingValues: *ValueRef,
                               IncomingBlocks: BasicBlockRef,
                               Count: c_uint);
        #[fast_ffi]
        pub fn LLVMCountIncoming(PhiNode: ValueRef) -> c_uint;
        #[fast_ffi]
        pub fn LLVMGetIncomingBlock(PhiNode: ValueRef, Index: c_uint) -> BasicBlockRef;
        #[fast_ffi]
        pub fn LLVMGetIncomingValue(PhiNode: ValueRef, Index: c_uint) -> ValueRef;
    }
}

pub mod ir_builder {
    use super::*;
    use std::libc::*;

    pub extern {

        #[fast_ffi]
        pub fn LLVMCreateBuilderInContext(C: ContextRef) -> BuilderRef;
        #[fast_ffi]
        pub fn LLVMCreateBuilder() -> BuilderRef;
        #[fast_ffi]
        pub fn LLVMPositionBuilder(Builder: BuilderRef, Block: BasicBlockRef, Instr: ValueRef);
        #[fast_ffi]
        pub fn LLVMPositionBuilderBefore(Builder: BuilderRef, Instr: ValueRef);
        #[fast_ffi]
        pub fn LLVMPositionBuilderAtEnd(Builder: BuilderRef, Block: BasicBlockRef);
        #[fast_ffi]
        pub fn LLVMGetInsertBlock(Builder: BuilderRef) -> BasicBlockRef;
        #[fast_ffi]
        pub fn LLVMClearInsertionPosition(Builder: BuilderRef);
        #[fast_ffi]
        pub fn LLVMInsertIntoBuilder(Builder: BuilderRef, Instr: ValueRef);
        #[fast_ffi]
        pub fn LLVMInsertIntoBuilderWithName(Builder: BuilderRef, Instr: ValueRef, Name: *c_char);
        #[fast_ffi]
        pub fn LLVMDisposeBuilder(Builder: BuilderRef);

        /* Metadata */
        #[fast_ffi]
        pub fn LLVMSetCurrentDebugLocation(Builder: BuilderRef, L: ValueRef);
        #[fast_ffi]
        pub fn LLVMGetCurrentDebugLocation(Builder: BuilderRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMSetInstDebugLocation(Builder: BuilderRef, Inst: ValueRef);

        /* Terminators */
        #[fast_ffi]
        pub fn LLVMBuildRetVoid(B: BuilderRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildRet(B: BuilderRef, V: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildAggregateRet(B: BuilderRef, RetVals: *ValueRef, N: c_uint) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildBr(B: BuilderRef, Dest: BasicBlockRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildCondBr(B: BuilderRef,
                               If: ValueRef,
                               Then: BasicBlockRef,
                               Else: BasicBlockRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildSwitch(B: BuilderRef,
                               V: ValueRef,
                               Else: BasicBlockRef,
                               NumCases: c_uint) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildIndirectBr(B: BuilderRef, Addr: ValueRef, NumDests: c_uint) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildInvoke(B: BuilderRef,
                               Fn: ValueRef,
                               Args: *ValueRef,
                               NumArgs: c_uint,
                               Then: BasicBlockRef,
                               Catch: BasicBlockRef,
                               Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildLandingPad(B: BuilderRef,
                                   Ty: TypeRef,
                                   PersFn: ValueRef,
                                   NumClauses: c_uint,
                                   Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildResume(B: BuilderRef, Exn: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildUnreachable(B: BuilderRef) -> ValueRef;

        /* Add a case to the switch instruction */
        #[fast_ffi]
        pub fn LLVMAddCase(Switch: ValueRef, OnVal: ValueRef, Dest: BasicBlockRef);

        /* Add a destination to the indirectbr instruction */
        #[fast_ffi]
        pub fn LLVMAddDestination(IndirectBr: ValueRef, Dest: BasicBlockRef);

        /* Add a catch or filter clause to the landingpad instruction */
        #[fast_ffi]
        pub fn LLVMAddClause(LandingPad: ValueRef, ClauseVal: ValueRef);

        /* Set the 'cleanup' flag in the landingpad instruction */
        #[fast_ffi]
        pub fn LLVMSetCleanup(LandingPad: ValueRef, Val: Bool);

        /* Arithmetic */
        #[fast_ffi]
        pub fn LLVMBuildAdd(B: BuilderRef,
                            LHS: ValueRef,
                            RHS: ValueRef,
                            Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildNSWAdd(B: BuilderRef,
                               LHS: ValueRef,
                               RHS: ValueRef,
                               Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildNUWAdd(B: BuilderRef,
                               LHS: ValueRef,
                               RHS: ValueRef,
                               Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildFAdd(B: BuilderRef,
                             LHS: ValueRef,
                             RHS: ValueRef,
                             Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildSub(B: BuilderRef,
                            LHS: ValueRef,
                            RHS: ValueRef,
                            Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildNSWSub(B: BuilderRef,
                               LHS: ValueRef,
                               RHS: ValueRef,
                               Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildNUWSub(B: BuilderRef,
                               LHS: ValueRef,
                               RHS: ValueRef,
                               Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildFSub(B: BuilderRef,
                             LHS: ValueRef,
                             RHS: ValueRef,
                             Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildMul(B: BuilderRef,
                            LHS: ValueRef,
                            RHS: ValueRef,
                            Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildNSWMul(B: BuilderRef,
                               LHS: ValueRef,
                               RHS: ValueRef,
                               Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildNUWMul(B: BuilderRef,
                               LHS: ValueRef,
                               RHS: ValueRef,
                               Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildFMul(B: BuilderRef,
                             LHS: ValueRef,
                             RHS: ValueRef,
                             Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildUDiv(B: BuilderRef,
                             LHS: ValueRef,
                             RHS: ValueRef,
                             Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildSDiv(B: BuilderRef,
                             LHS: ValueRef,
                             RHS: ValueRef,
                             Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildExactSDiv(B: BuilderRef,
                                  LHS: ValueRef,
                                  RHS: ValueRef,
                                  Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildFDiv(B: BuilderRef,
                             LHS: ValueRef,
                             RHS: ValueRef,
                             Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildURem(B: BuilderRef,
                             LHS: ValueRef,
                             RHS: ValueRef,
                             Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildSRem(B: BuilderRef,
                             LHS: ValueRef,
                             RHS: ValueRef,
                             Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildFRem(B: BuilderRef,
                             LHS: ValueRef,
                             RHS: ValueRef,
                             Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildShl(B: BuilderRef,
                            LHS: ValueRef,
                            RHS: ValueRef,
                            Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildLShr(B: BuilderRef,
                             LHS: ValueRef,
                             RHS: ValueRef,
                             Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildAShr(B: BuilderRef,
                             LHS: ValueRef,
                             RHS: ValueRef,
                             Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildAnd(B: BuilderRef,
                            LHS: ValueRef,
                            RHS: ValueRef,
                            Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildOr(B: BuilderRef,
                           LHS: ValueRef,
                           RHS: ValueRef,
                           Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildXor(B: BuilderRef,
                            LHS: ValueRef,
                            RHS: ValueRef,
                            Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildBinOp(B: BuilderRef,
                              Op: c_uint,
                              LHS: ValueRef,
                              RHS: ValueRef,
                              Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildNeg(B: BuilderRef, V: ValueRef, Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildNSWNeg(B: BuilderRef, V: ValueRef,
                               Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildNUWNeg(B: BuilderRef, V: ValueRef,
                               Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildFNeg(B: BuilderRef, V: ValueRef, Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildNot(B: BuilderRef, V: ValueRef, Name: *c_char) -> ValueRef;

        /* Memory */
        #[fast_ffi]
        pub fn LLVMBuildMalloc(B: BuilderRef, Ty: TypeRef, Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildArrayMalloc(B: BuilderRef,
                                    Ty: TypeRef,
                                    Val: ValueRef,
                                    Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildAlloca(B: BuilderRef, Ty: TypeRef, Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildArrayAlloca(B: BuilderRef,
                                    Ty: TypeRef,
                                    Val: ValueRef,
                                    Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildFree(B: BuilderRef,
                             PointerVal: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildLoad(B: BuilderRef,
                             PointerVal: ValueRef,
                             Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildStore(B: BuilderRef,
                              Val: ValueRef,
                              Ptr: ValueRef) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildGEP(B: BuilderRef,
                            Pointer: ValueRef,
                            Indices: *ValueRef,
                            NumIndices: c_uint,
                            Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildInBoundsGEP(B: BuilderRef,
                                    Pointer: ValueRef,
                                    Indices: *ValueRef,
                                    NumIndices: c_uint,
                                    Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildStructGEP(B: BuilderRef,
                                  Pointer: ValueRef,
                                  Idx: c_uint,
                                  Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildGlobalString(B: BuilderRef,
                                     Str: *c_char,
                                     Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildGlobalStringPtr(B: BuilderRef,
                                        Str: *c_char,
                                        Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMGetVolatile(MemoryAccessInst: ValueRef) -> Bool;
        #[fast_ffi]
        pub fn LLVMSetVolatile(MemoryAccessInst: ValueRef, IsVolatile: Bool);

        /* Casts */
        #[fast_ffi]
        pub fn LLVMBuildTrunc(B: BuilderRef,
                              Val: ValueRef,
                              DestTy: TypeRef,
                              Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildZExt(B: BuilderRef,
                             Val: ValueRef,
                             DestTy: TypeRef,
                             Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildSExt(B: BuilderRef,
                             Val: ValueRef,
                             DestTy: TypeRef,
                             Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildFPToUI(B: BuilderRef,
                               Val: ValueRef,
                               DestTy: TypeRef,
                               Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildFPToSI(B: BuilderRef,
                               Val: ValueRef,
                               DestTy: TypeRef,
                               Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildUIToFP(B: BuilderRef,
                               Val: ValueRef,
                               DestTy: TypeRef,
                               Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildSIToFP(B: BuilderRef,
                               Val: ValueRef,
                               DestTy: TypeRef,
                               Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildFPTrunc(B: BuilderRef,
                                Val: ValueRef,
                                DestTy: TypeRef,
                                Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildFPExt(B: BuilderRef,
                              Val: ValueRef,
                              DestTy: TypeRef,
                              Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildPtrToInt(B: BuilderRef,
                                 Val: ValueRef,
                                 DestTy: TypeRef,
                                 Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildIntToPtr(B: BuilderRef,
                                 Val: ValueRef,
                                 DestTy: TypeRef,
                                 Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildBitCast(B: BuilderRef,
                                Val: ValueRef,
                                DestTy: TypeRef,
                                Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildZExtOrBitCast(B: BuilderRef,
                                      Val: ValueRef,
                                      DestTy: TypeRef,
                                      Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildSExtOrBitCast(B: BuilderRef,
                                      Val: ValueRef,
                                      DestTy: TypeRef,
                                      Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildTruncOrBitCast(B: BuilderRef,
                                       Val: ValueRef,
                                       DestTy: TypeRef,
                                       Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildCast(B: BuilderRef,
                             Op: c_uint,
                             Val: ValueRef,
                             DestTy: TypeRef,
                             Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildPointerCast(B: BuilderRef,
                                    Val: ValueRef,
                                    DestTy: TypeRef,
                                    Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildIntCast(B: BuilderRef,
                                Val: ValueRef, /*Signed cast!*/
                                DestTy: TypeRef,
                                Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildFPCast(B: BuilderRef,
                               Val: ValueRef,
                               DestTy: TypeRef,
                               Name: *c_char) -> ValueRef;

        /* Comparisons */
        #[fast_ffi]
        pub fn LLVMBuildICmp(B: BuilderRef,
                             Op: c_uint,
                             LHS: ValueRef,
                             RHS: ValueRef,
                             Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildFCmp(B: BuilderRef,
                             Op: c_uint,
                             LHS: ValueRef,
                             RHS: ValueRef,
                             Name: *c_char) -> ValueRef;

        /* Miscellaneous instructions */
        #[fast_ffi]
        pub fn LLVMBuildPhi(B: BuilderRef, Ty: TypeRef, Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildCall(B: BuilderRef,
                             Fn: ValueRef,
                             Args: *ValueRef,
                             NumArgs: c_uint,
                             Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildSelect(B: BuilderRef,
                               If: ValueRef,
                               Then: ValueRef,
                               Else: ValueRef,
                               Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildVAArg(B: BuilderRef,
                              List: ValueRef,
                              Ty: TypeRef,
                              Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildExtractElement(B: BuilderRef,
                                       VecVal: ValueRef,
                                       Index: ValueRef,
                                       Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildInsertElement(B: BuilderRef,
                                      VecVal: ValueRef,
                                      EltVal: ValueRef,
                                      Index: ValueRef,
                                      Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildShuffleVector(B: BuilderRef,
                                      V1: ValueRef,
                                      V2: ValueRef,
                                      Mask: ValueRef,
                                      Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildExtractValue(B: BuilderRef,
                                     AggVal: ValueRef,
                                     Index: c_uint,
                                     Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildInsertValue(B: BuilderRef,
                                    AggVal: ValueRef,
                                    EltVal: ValueRef,
                                    Index: c_uint,
                                    Name: *c_char) -> ValueRef;

        #[fast_ffi]
        pub fn LLVMBuildIsNull(B: BuilderRef,
                               Val: ValueRef,
                               Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildIsNotNull(B: BuilderRef,
                                  Val: ValueRef,
                                  Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildPtrDiff(B: BuilderRef,
                                LHS: ValueRef,
                                RHS: ValueRef,
                                Name: *c_char) -> ValueRef;
        #[fast_ffi]
        pub fn LLVMBuildAtomicRMW(B: BuilderRef,
                                  op: c_uint,
                                  PTR: ValueRef,
                                  Val: ValueRef,
                                  ordering: c_uint,
                                  singleThread: Bool) -> ValueRef;
    }
}

pub mod mod_prov {
    use super::*;

    pub extern {
        #[fast_ffi]
        pub fn LLVMCreateModuleProviderForExistingModule(M:ModuleRef) -> ModuleProviderRef;
        #[fast_ffi]
        pub fn LLVMDisposeModuleProvider(M: ModuleProviderRef);
    }
}

pub mod mem_buffer {
    use super::*;
    use std::libc::*;

    pub extern {
        #[fast_ffi]
        pub fn LLVMCreateMemoryBufferWithContentsOfFile(Path: *c_char,
                                                        OutMemBuf: *mut MemoryBufferRef,
                                                        OutMessage: *mut *c_char) -> Bool;
        #[fast_ffi]
        pub fn LLVMCreateMemoryBufferWithSTDIN(OutMemBuf: *mut MemoryBufferRef,
                                               OutMessage: *mut *c_char) -> Bool;
        #[fast_ffi]
        pub fn LLVMCreateMemoryBufferWithMemoryRange(
            InputData: *c_char,
            InputDataLength: size_t,
            BufferName: *c_char,
            RequiresNullTerminator: Bool) -> MemoryBufferRef;
        #[fast_ffi]
        pub fn LLVMCreateMemoryBufferWithMemoryRangeCopy(
            InputData: *c_char,
            InputDataLength: size_t,
            BufferName: *c_char) -> MemoryBufferRef;
        #[fast_ffi]
        pub fn LLVMGetBufferStart(MemBuf: MemoryBufferRef) -> *char;
        #[fast_ffi]
        pub fn LLVMGetBufferSize(MemBuf: MemoryBufferRef) -> size_t;
        #[fast_ffi]
        pub fn LLVMDisposeMemoryBuffer(MemBuf: MemoryBufferRef);
    }
}

pub mod passes {
    use super::*;

    pub extern {
        #[fast_ffi]
        pub fn LLVMGetGlobalPassRegistry() -> PassRegistryRef;
        #[fast_ffi]
        pub fn LLVMCreatePassManager() -> PassManagerRef;
        #[fast_ffi]
        pub fn LLVMCreateFunctionPassManagerForModule(M: ModuleRef) -> PassManagerRef;
        #[fast_ffi]
        pub fn LLVMRunPassManager(PM: PassManagerRef, M: ModuleRef) -> Bool;
        #[fast_ffi]
        pub fn LLVMInitializeFunctionPassManager(FPM: PassManagerRef) -> Bool;
        #[fast_ffi]
        pub fn LLVMRunFunctionPassManager(FPM: PassManagerRef, F: ValueRef) -> Bool;
        #[fast_ffi]
        pub fn LLVMFinalizeFunctionPassManager(FPM: PassManagerRef) -> Bool;
        #[fast_ffi]
        pub fn LLVMDisposePassManager(PM: PassManagerRef);
    }
}
