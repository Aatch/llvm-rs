
use std::libc;

pub type Bool = libc::c_int;

pub static FALSE : Bool = 0;
pub static TRUE : Bool = 1;

pub enum LLVMOpaqueContext {}
pub enum LLVMOpaqueModule {}
pub enum LLVMOpaqueType {}
pub enum LLVMOpaqueValue {}
pub enum LLVMOpaqueBasicBlock {}
pub enum LLVMOpaqueBuilder {}
pub enum LLVMOpaqueModuleProvider {}
pub enum LLVMOpaqueMemoryBuffer {}
pub enum LLVMOpaquePassManager {}
pub enum LLVMOpaquePassRegistry {}
pub enum LLVMOpaqueUse {}

pub type LLVMContextRef = *LLVMOpaqueContext;
pub type LLVMModuleRef = *LLVMOpaqueModule;
pub type LLVMTypeRef = *LLVMOpaqueType;
pub type LLVMValueRef = *LLVMOpaqueValue;
pub type LLVMBasicBlockRef = *LLVMOpaqueBasicBlock;
pub type LLVMBuilderRef = *LLVMOpaqueBuilder;
pub type LLVMModuleProviderRef = *LLVMOpaqueModuleProvider;
pub type LLVMMemoryBufferRef = *LLVMOpaqueMemoryBuffer;
pub type LLVMPassManagerRef = *LLVMOpaquePassManager;
pub type LLVMPassRegistryRef = *LLVMOpaquePassRegistry;
pub type LLVMUseRef = *LLVMOpaqueUse;

pub type LLVMAttribute = libc::c_uint;
pub static ZExtAttr         : LLVMAttribute = 1 << 0;
pub static SExtAttr         : LLVMAttribute = 1 << 1;
pub static NoReturn         : LLVMAttribute = 1 << 2;
pub static InReg            : LLVMAttribute = 1 << 3;
pub static StructRet        : LLVMAttribute = 1 << 4;
pub static NoUnwind         : LLVMAttribute = 1 << 5;
pub static NoAlias          : LLVMAttribute = 1 << 6;
pub static ByVal            : LLVMAttribute = 1 << 7;
pub static Nest             : LLVMAttribute = 1 << 8;
pub static ReadNone         : LLVMAttribute = 1 << 9;
pub static ReadOnly         : LLVMAttribute = 1 << 10;
pub static NoInline         : LLVMAttribute = 1 << 11;
pub static AlwaysInline     : LLVMAttribute = 1 << 12;
pub static OptimizeForSize  : LLVMAttribute = 1 << 13;
pub static StackProtect     : LLVMAttribute = 1 << 14;
pub static StackProtectReq  : LLVMAttribute = 1 << 15;
pub static Alignment        : LLVMAttribute = 31 << 16;
pub static NoCapture        : LLVMAttribute = 1 << 21;
pub static NoRedZone        : LLVMAttribute = 1 << 22;
pub static NoImplicitFloat  : LLVMAttribute = 1 << 23;
pub static Naked            : LLVMAttribute = 1 << 24;
pub static InlineHint       : LLVMAttribute = 1 << 25;
pub static StackAlignment   : LLVMAttribute = 7 << 26;
pub static ReturnsTwice     : LLVMAttribute = 1 << 29;
pub static UWTable          : LLVMAttribute = 1 << 30;
pub static NonLazyBind      : LLVMAttribute = 1 << 31;

#[repr(C)]
pub enum LLVMOpcode {
    Ret = 1,
    Br, Switch, IndirectBr, Invoke, Unreachable,
    Add, FAdd, Sub, FSub, Mul, FMul,
    UDiv, SDiv, FDiv,
    URem, SRem, FRem,
    Shl, LShr, AShr,
    And, Or, Xor,
    Alloca, Load, Store, GetElementPtr,
    Trunc, ZExt, SExt,
    FPTrunc, FPExt,
    PtrToInt, IntToPtr,
    BitCast,
    ICmp, FCmp,
    PHI,
    Call,
    Select,
    UserOp1, UserOp2,
    VAArg,
    ExtractElement, InsertElement, ShuffleVector,
    ExtractValue, InsertValue,
    Fence, AtomicCmpXchg, AtomicRMW,
    Resume, LandingPad
}

#[repr(C)]
pub enum LLVMTypeKind {
    Void,
    Half, Float, Double, X86_FP80, FP128, PPC_FP128,
    Label, Integer, Function, Struct,
    Array, Pointer, Vector,
    Metadata, MMX
}

#[repr(C)]
pub enum LLVMLinkage {
    External, AvailableExternally,
    LinkOnceAny, LinkOnceODR, LinkOnceODRAutoHide,
    WeakAny, WeakODR,
    Appending,
    Internal, Private,
    DLLImport, DLLExport,
    ExternalWeak,
    Ghost,
    Common,
    LinkerPrivate, LinkerPrivateWeak
}

#[repr(C)]
pub enum LLVMVisibility {
    Default,
    Hidden,
    Protected
}

#[repr(C)]
pub enum LLVMCallConv {
    CCall = 0,
    FastCall = 8,
    ColdCall = 9,
    X86StdCall = 64,
    X86Fastcall = 65
}

#[repr(C)]
pub enum LLVMIntPredicate {
    IntEQ = 32,
    IntNE,
    IntUGT,
    IntUGE,
    IntULT,
    IntULE,
    IntSGT,
    IntSGE,
    IntSLT,
    IntSLE
}

#[repr(C)]
pub enum LLVMRealPredicate {
    RealPredicateFalse,
    RealOEQ,
    RealOGT,
    RealOGE,
    RealOLT,
    RealOLE,
    RealONE,
    RealORD,
    RealUNO,
    RealUEQ,
    RealUGT,
    RealUGE,
    RealULT,
    RealULE,
    RealUNE,
    RealPredicateTrue
}

#[repr(C)]
pub enum LLVMLandingPadClauseTy {
    LandingPadCatch,
    LandingPadFilter
}

#[repr(C)]
pub enum LLVMThreadLocalMode {
    NotThreadLocal = 0,
    GeneralDynamicTLS,
    LocalDynamicTLS,
    InitialExecTLS,
    LocalExecTLS
}

#[repr(C)]
pub enum LLVMAtomicOrdering {
    NotAtomic               = 0,
    Unordered               = 1,
    Monotonic               = 2,
    Acquire                 = 4,
    Release                 = 5,
    AcquireRelease          = 6,
    SequentiallyConsistent  = 7
}

#[repr(C)]
pub enum LLVMAtomicRMWBinOp {
    RMWXchh,
    RMWAdd,
    RMWSub,
    RMWAnd,
    RMWNand,
    RMWOr,
    RMWXor,
    RMWMax,
    RMWMin,
    RMWUMax,
    RMWUMin
}

#[link(name="LLVMCore")]
extern "C" {
    pub fn LLVMInitializeCore(R: LLVMPassRegistryRef);
    pub fn LLVMShutdown();
    pub fn LLVMCreateMessage(Message: *libc::c_char) -> *libc::c_char;
    pub fn LLVMDisposeMessage(Message: *mut libc::c_char);

    pub fn LLVMContextCreate() -> LLVMContextRef;
    pub fn LLVMGetGlobalContext() -> LLVMContextRef;
    pub fn LLVMContextDispose(C: LLVMContextRef);
    pub fn LLVMGetMDKindIDInContext(C: LLVMContextRef,
        Name: *libc::c_char, SLen: libc::c_uint) -> libc::c_uint;
    pub fn LLVMGetMDKindID(Name: *libc::c_char, SLen: libc::c_uint) -> libc::c_uint;

    pub fn LLVMModuleCreateWithName(ModuleID: *libc::c_char) -> LLVMModuleRef;
    pub fn LLVMModuleCreateWithNameInContext(ModuleID: *libc::c_char, C: LLVMContextRef)
        -> LLVMModuleRef;
    pub fn LLVMDisposeModule(M: LLVMModuleRef);
    pub fn LLVMGetDataLayout(M: LLVMModuleRef) -> *libc::c_char;
    pub fn LLVMSetDataLayout(M: LLVMModuleRef, Triple: *libc::c_char);
    pub fn LLVMGetTarget(M: LLVMModuleRef) -> *libc::c_char;
    pub fn LLVMSetTarget(M: LLVMModuleRef, Triple: *libc::c_char);
    pub fn LLVMDumpModule(M: LLVMModuleRef);
    pub fn LLVMPrintModuleToFile(M: LLVMModuleRef,
        Filename: *libc::c_char, ErrorMessage: *mut *libc::c_char) -> Bool;
    pub fn LLVMSetModuleInlineAsm(M: LLVMModuleRef, Asm: *libc::c_char);
    pub fn LLVMGetModuleContext(M: LLVMModuleRef) -> LLVMContextRef;
    pub fn LLVMGetTypeByName(M: LLVMModuleRef, Name: *libc::c_char) -> LLVMTypeRef;
    pub fn LLVMGetNamedMetadataNumOperands(M: LLVMModuleRef, name: *libc::c_char) -> libc::c_uint;
    pub fn LLVMGetNamedMetadataOperands(M: LLVMModuleRef,
        name: *libc::c_char, Dest: *mut LLVMValueRef);
    pub fn LLVMAddNamedMetadataOperand(M: LLVMModuleRef,
        name: *libc::c_char, Dest: LLVMValueRef);
    pub fn LLVMAddFunction(M: LLVMModuleRef, Name: *libc::c_char) -> LLVMValueRef;
    pub fn LLVMGetFirstFunction(M: LLVMModuleRef) -> LLVMValueRef;
    pub fn LLVMGetLastFunction(M: LLVMModuleRef) -> LLVMValueRef;
    pub fn LLVMGetNextFunction(Fn: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetPreviousFunction(Fn: LLVMValueRef) -> LLVMValueRef;

    pub fn LLVMGetTypeKind(Ty: LLVMTypeRef) -> LLVMTypeKind;
    pub fn LLVMTypeIsSized(Ty: LLVMTypeRef) -> Bool;
    pub fn LLVMGetTypeContext(Ty: LLVMTypeRef) -> LLVMContextRef;

    pub fn LLVMInt1TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMInt8TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMInt16TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMInt32TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMInt64TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMIntTypeInContext(C: LLVMContextRef, NumBits: libc::c_uint) -> LLVMTypeRef;
    pub fn LLVMInt1Type() -> LLVMTypeRef;
    pub fn LLVMInt8Type() -> LLVMTypeRef;
    pub fn LLVMInt16Type() -> LLVMTypeRef;
    pub fn LLVMInt32Type() -> LLVMTypeRef;
    pub fn LLVMInt64Type() -> LLVMTypeRef;
    pub fn LLVMIntType(NumBits: libc::c_uint);
    pub fn LLVMGetIntTypeWidth(IntegerTy: LLVMTypeRef) -> libc::c_uint;

    pub fn LLVMHalfTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMFloatTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMDoubleTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMX86FP80TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMFP128TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMPPCFP128TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;

    pub fn LLVMHalfType() -> LLVMTypeRef;
    pub fn LLVMFloatType() -> LLVMTypeRef;
    pub fn LLVMDoubleType() -> LLVMTypeRef;
    pub fn LLVMX86FP80Type() -> LLVMTypeRef;
    pub fn LLVMFP128Type() -> LLVMTypeRef;
    pub fn LLVMPPCFP128Type() -> LLVMTypeRef;

    pub fn LLVMFunctionType(ReturnType: LLVMTypeRef, ParamTypes: *LLVMTypeRef,
        ParamCount: libc::c_uint, IsVarArg: Bool) -> LLVMTypeRef;
    pub fn LLVMIsFunctionVarArg(FunctionTy: LLVMTypeRef) -> Bool;
    pub fn LLVMGetReturnType(FunctionTy: LLVMTypeRef) -> LLVMTypeRef;
    pub fn LLVMCountParamTypes(FunctionTy: LLVMTypeRef) -> libc::c_uint;
    pub fn LLVMGetParamTypes(FunctionTy: LLVMTypeRef, Dest: *mut LLVMTypeRef);

    pub fn LLVMStructTypeInContext(C: LLVMContextRef,
        ElementTypes: *LLVMTypeRef, ElementCount: libc::c_uint, Packed: Bool) -> LLVMTypeRef;
    pub fn LLVMStructType(ElementTypes: *LLVMTypeRef,
        ElementCount: libc::c_uint, Packed: Bool) -> LLVMTypeRef;
    pub fn LLVMStructCreateNamed(C: LLVMContextRef, Name: *libc::c_char) -> LLVMTypeRef;
    pub fn LLVMGetStructName(Ty: LLVMTypeRef) -> *libc::c_char;
    pub fn LLVMStructSetBody(StructTy: LLVMTypeRef,
        ElementTypes: *LLVMTypeRef, ElementCount: libc::c_uint, Packed: Bool);
    pub fn LLVMCountStructElementTypes(StructTy: LLVMTypeRef) -> libc::c_uint;
    pub fn LLVMGetStructElementTypes(StructTy: LLVMTypeRef, Dest: *mut LLVMTypeRef);
    pub fn LLVMIsPackedStruct(StructTy: LLVMTypeRef) -> Bool;
    pub fn LLVMIsOpaqueStruct(StructTy: LLVMTypeRef) -> Bool;

    pub fn LLVMGetElementType(Ty: LLVMTypeRef) -> LLVMTypeRef;
    pub fn LLVMArrayType(ElementType: LLVMTypeRef, ElementCount: libc::c_uint) -> LLVMTypeRef;
    pub fn LLVMGetArrayLength(ArrayTy: LLVMTypeRef) -> libc::c_uint;
    pub fn LLVMPointerType(ElementType: LLVMTypeRef, AddressSpace: libc::c_uint) -> LLVMTypeRef;
    pub fn LLVMGetPointerAddressSpace(PointerTy: LLVMTypeRef) -> libc::c_uint;
    pub fn LLVMVectorType(ElementType: LLVMTypeRef, ElementCount: libc::c_uint) -> LLVMTypeRef;
    pub fn LLVMGetVectorSize(VectorTy: LLVMTypeRef) -> libc::c_uint;

    pub fn LLVMVoidTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMLabelTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMX86MMXTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMVoidType() -> LLVMTypeRef;
    pub fn LLVMLabelType() -> LLVMTypeRef;
    pub fn LLVMX86MMXType() -> LLVMTypeRef;

    pub fn LLVMIsAArgument(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABasicBlock(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInlineAsm(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMDNode(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMDString(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUser(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstant(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABlockAddress(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantAggregateZero(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantArray(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantExpr(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantFP(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantInt(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantPointerNull(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantStruct(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantVector(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGlobalValue(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFunction(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGlobalAlias(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGlobalVariable(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUndefValue(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInstruction(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABinaryOperator(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACallInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAIntrinsicInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsADbgInfoIntrinsic(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsADbgDeclareInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMemIntrinsic(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMemCpyInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMemMoveInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMemSetInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACmpInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFCmpInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAICmpInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAExtractElementInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGetElementPtrInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInsertElementInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInsertValueInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsALandingPadInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAPHINode(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsASelectInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAShuffleVectorInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAStoreInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsATerminatorInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABranchInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAIndirectBrInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInvokeInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAReturnInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsASwitchInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUnreachableInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAResumeInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUnaryInstruction(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAAllocaInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACastInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABitCastInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFPExtInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFPToSIInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFPToUIInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFPTruncInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAIntToPtrInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAPtrToIntInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsASExtInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsASIToFPInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsATruncInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUIToFPInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAZExtInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAExtractValueInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsALoadInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAVAArgInst(Val: LLVMValueRef) -> LLVMValueRef;

    pub fn LLVMTypeOf(Val: LLVMValueRef) -> LLVMTypeRef;
    pub fn LLVMGetValueName(Val: LLVMValueRef) -> *libc::c_char;
    pub fn LLVMSetValueName(Val: LLVMValueRef, Name: *libc::c_char);
    pub fn LLVMDumpValue(Val: LLVMValueRef);
    pub fn LLVMReplaceAllUsesWith(OldVal: LLVMValueRef, NewVal: LLVMValueRef);
    pub fn LLVMIsConstant(Val: LLVMValueRef) -> Bool;
    pub fn LLVMIsUndef(Val: LLVMValueRef) -> Bool;

    pub fn LLVMGetFirstUse(Val: LLVMValueRef) -> LLVMUseRef;
    pub fn LLVMGetNextUse(U: LLVMUseRef) -> LLVMUseRef;
    pub fn LLVMGetUser(U: LLVMUseRef) -> LLVMValueRef;
    pub fn LLVMGetUsedValue(U: LLVMUseRef) -> LLVMValueRef;

    pub fn LLVMGetOperand(Val: LLVMValueRef, Index: libc::c_uint) -> LLVMValueRef;
    pub fn LLVMSetOperant(User: LLVMValueRef, Index: libc::c_uint, Val: LLVMValueRef);
    pub fn LLVMGetNumOperands(Val: LLVMValueRef) -> libc::c_int;

    pub fn LLVMConstNull(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstAllOnes(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMGetUndef(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMIsNull(Val: LLVMValueRef) -> Bool;
    pub fn LLVMConstPointerNull(Ty: LLVMTypeRef) -> LLVMValueRef;

    pub fn LLVMConstInt(IntTy: LLVMTypeRef, N: libc::c_ulonglong,
        SignExtend: Bool) -> LLVMValueRef;
}
