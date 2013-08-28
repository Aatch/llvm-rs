use super::*;
use ffi::core::{value,constant,global,function,metadata,bb};
use ffi::core::{ValueRef,True,False,Linkage,Visibility,ThreadLocalMode,Attribute};

use std::str;
use std::vec;
use std::cast;

pub trait Val<T:ty::Ty> : Wrapper<ValueRef> { }
pub trait ConstVal<T:ty::Ty> : Val<T> { }
pub trait GlobalVal<T:ty::Ty> : ConstVal<T> { }

pub trait ValImpl<T:ty::Ty> {
    fn type_of(&self) -> T;
    fn get_name(&self) -> ~str;
    fn set_name(&mut self, name: &str);
    fn dump(&self);
    fn is_constant(&self) -> bool;
    fn is_undef(&self) -> bool;
    fn to_value(&self) -> Value<T>;
}

pub trait ConstImpl<T:ty::Ty> {
    fn is_null(&self) -> bool;
}

pub trait GlobalImpl<T:ty::Ty> {
    fn is_decl(&self) -> bool;

    fn get_linkage(&self) -> Linkage;
    fn set_linkage(&mut self, link: Linkage);

    fn get_section(&self) -> ~str;
    fn set_section(&mut self, section: &str);

    fn get_visibility(&self) -> Visibility;
    fn set_visibility(&mut self, vis: Visibility);

    fn get_alignment(&self) -> uint;
    fn set_alignment(&mut self, align: uint);
}

pub trait ConstInt : ConstVal<ty::Integer> {
    fn new(ty: ty::Integer, val: u64, sext: bool) -> Self;
    fn zext_val(&self) -> u64;
    fn sext_val(&self) -> u64;
}

pub trait ConstReal : ConstVal<ty::Real> {
    fn new(ty: ty::Real, val: f64) -> Self;
}

pub trait ConstArray<T:ty::Ty> : ConstVal<ty::Array<T>> {
    fn new(ty: T, els: ~[Constant<T>]) -> Self;
    fn new_from_str(c: &Context, data: &str, null_term: bool) -> Self;
}

pub trait ConstVector<T:ty::Ty> : ConstVal<ty::Vector<T>> {
    fn new(els: ~[Constant<T>]) -> Self;
}

pub trait ConstStruct : ConstVal<ty::Struct> {
    fn new(c: Context, members: ~[Constant<ty::Type>], packed: bool) -> Self;
    fn named(ty: ty::Struct, members: ~[Constant<ty::Type>]) -> Self;
}

pub trait GlobalVar<T:ty::Ty> : GlobalVal<T> {
    fn delete(&self);

    fn get_initializer(&self) -> Constant<T>;
    fn set_initializer(&mut self, val: Constant<T>);

    fn is_thread_local(&self) -> bool;
    fn set_thread_local(&mut self, thread_local: bool);

    fn is_global_constant(&self) -> bool;
    fn set_global_constant(&mut self, global_constant: bool);

    fn get_thread_local_mode(&self) -> ThreadLocalMode;
    fn set_thread_local_mode(&mut self, mode: ThreadLocalMode);

    fn is_externally_initialized(&self) -> bool;
    fn set_externally_initialized(&self, extern_init: bool);
}

pub trait FunctionVal : GlobalVal<ty::Function> {
    fn delete(&self);

    fn intrinsic_id(&self) -> uint;

    fn get_callconv(&self) -> uint;
    fn set_callconv(&mut self, cc: uint);

    fn get_gc(&self) -> ~str;
    fn set_gc(&mut self, &str);

    fn add_attr(&mut self, attr: Attribute);
    fn get_attr(&self) -> u64;
    fn remove_attr(&mut self, attr: Attribute);

    fn params(&self) -> ~[Param<ty::Type>];
    fn get_param(&self, idx: uint) -> Param<ty::Type>;

    fn add_basic_block(&mut self, c: &Context, name: &str) -> BasicBlock;
}

pub trait ParamVal<T:ty::Ty> : Val<T> {
    fn add_attr(&mut self, attr: Attribute);
    fn remove_attr(&mut self, attr: Attribute);
    fn get_attribute(&self) -> u64;
    fn set_alignment(&mut self, align: uint);
}

pub trait MDVal : Val<ty::Metadata> {
    fn operands(&self) -> ~[Value<ty::Type>];
    fn get_string(&self) -> ~str;
}

pub trait BBVal : Val<ty::Label> {
    fn parent(&self) -> Function;
    fn terminator(&self) -> instruction::Instruction<ty::Type>;

    fn insert(&self, c: Context, name: &str) -> BasicBlock;
    fn delete(&self);

    fn move_before(&self, before: BasicBlock);
    fn move_after(&self, after: BasicBlock);
}

impl<T:ty::Ty,U:Val<T>> ValImpl<T> for U {
    fn type_of(&self) -> T {
        unsafe {
            let r = value::LLVMTypeOf(self.to_ref());
            Wrapper::from_ref(r)
        }
    }

    fn get_name(&self) -> ~str {
        unsafe {
            let buf = value::LLVMGetValueName(self.to_ref());
            str::raw::from_c_str(buf)
        }
    }

    fn set_name(&mut self, name: &str) {
        unsafe {
            do name.with_c_str |s| {
                value::LLVMSetValueName(self.to_ref(), s);
            }
        }
    }

    fn dump(&self) {
        unsafe {
            value::LLVMDumpValue(self.to_ref());
        }
    }

    fn is_constant(&self) -> bool {
        unsafe {
            value::LLVMIsConstant(self.to_ref()) == True
        }
    }

    fn is_undef(&self) -> bool {
        unsafe {
            value::LLVMIsUndef(self.to_ref()) == True
        }
    }

    fn to_value(&self) -> Value<T> {
        unsafe {
            cast::transmute(self)
        }
    }
}

impl<T:ty::Ty,U:ConstVal<T>> ConstImpl<T> for U {
    fn is_null(&self) -> bool {
        unsafe {
            constant::LLVMIsNull(self.to_ref()) == True
        }
    }
}

impl<T:ty::Ty,U:GlobalVal<T>> GlobalImpl<T> for U {
    fn is_decl(&self) -> bool {
        unsafe {
            global::LLVMIsDeclaration(self.to_ref()) == True
        }
    }

    fn get_linkage(&self) -> Linkage {
        unsafe {
            let l = global::LLVMGetLinkage(self.to_ref());
            cast::transmute(l)
        }
    }

    fn set_linkage(&mut self, link: Linkage) {
        unsafe {
            let l = link as std::libc::c_uint;
            global::LLVMSetLinkage(self.to_ref(), l);
        }
    }

    fn get_section(&self) -> ~str {
        unsafe {
            let s = global::LLVMGetSection(self.to_ref());
            str::raw::from_c_str(s)
        }
    }

    fn set_section(&mut self, section: &str) {
        unsafe {
            do section.with_c_str |s| {
                global::LLVMSetSection(self.to_ref(), s);
            }
        }
    }

    fn get_visibility(&self) -> Visibility {
        unsafe {
            let v = global::LLVMGetVisibility(self.to_ref());
            cast::transmute(v)
        }
    }

    fn set_visibility(&mut self, vis: Visibility) {
        unsafe {
            let v = vis as std::libc::c_uint;
            global::LLVMSetVisibility(self.to_ref(), v);
        }
    }

    fn get_alignment(&self) -> uint {
        unsafe {
            global::LLVMGetAlignment(self.to_ref()) as uint
        }
    }

    fn set_alignment(&mut self, align: uint) {
        unsafe {
            global::LLVMSetAlignment(self.to_ref(), align as std::libc::c_uint);
        }
    }
}

pub struct Value<T> {
    priv r: ValueRef
}

impl<T:ty::Ty> Wrapper<ValueRef> for Value<T> {
    fn from_ref(R: ValueRef) -> Value<T> {
        Value {
            r: R
        }
    }

    fn to_ref(&self) -> ValueRef {
        self.r
    }
}

impl<T:ty::Ty> Val<T> for Value<T> { }

pub struct Constant<T> {
    priv r: ValueRef
}

impl<T:ty::Ty> Wrapper<ValueRef> for Constant<T> {
    fn from_ref(R: ValueRef) -> Constant<T> {
        Constant {
            r: R
        }
    }

    fn to_ref(&self) -> ValueRef {
        self.r
    }
}

impl<T:ty::Ty> Val<T> for Constant<T> {}
impl<T:ty::Ty> ConstVal<T> for Constant<T> {}

impl<T:ty::Ty> Constant<T> {
    fn null(ty: T) -> Constant<T> {
        unsafe {
            let r = constant::LLVMConstNull(ty.to_ref());
            Wrapper::from_ref(r)
        }
    }

    fn all_ones(ty: T) -> Constant<T> {
        unsafe {
            let r = constant::LLVMConstAllOnes(ty.to_ref());
            Wrapper::from_ref(r)
        }
    }

    fn undef(ty: T) -> Constant<T> {
        unsafe {
            let r = constant::LLVMGetUndef(ty.to_ref());
            Wrapper::from_ref(r)
        }
    }

    fn null_ptr(ty: T) -> Constant<ty::Pointer<T>> {
        unsafe {
            let r = constant::LLVMConstPointerNull(ty.to_ref());
            Wrapper::from_ref(r)
        }
    }
}

impl ConstInt for Constant<ty::Integer> {
    fn new(ty: ty::Integer, val: u64, sext: bool) -> Constant<ty::Integer> {
        unsafe {
            let sext = if sext { True } else { False };
            let r = constant::LLVMConstInt(ty.to_ref(), val as std::libc::c_ulonglong, sext);
            Wrapper::from_ref(r)
        }
    }

    fn zext_val(&self) -> u64 {
        unsafe {
            constant::LLVMConstIntGetZExtValue(self.r) as u64
        }
    }

    fn sext_val(&self) -> u64 {
        unsafe {
            constant::LLVMConstIntGetSExtValue(self.r) as u64
        }
    }
}

impl ConstReal for Constant<ty::Real> {
    fn new(ty: ty::Real, val: f64) -> Constant<ty::Real> {
        unsafe {
            let r = constant::LLVMConstReal(ty.to_ref(), val);
            Wrapper::from_ref(r)
        }
    }
}

impl<T:ty::Ty> ConstArray<T> for Constant<ty::Array<T>> {
    fn new(ty: T, els: ~[Constant<T>]) -> Constant<ty::Array<T>> {
        unsafe {
            let llty = ty.to_ref();
            let llels = do els.map |e| { e.to_ref() };

            let r = do llels.as_imm_buf |b, len| {
                constant::LLVMConstArray(llty, b, len as std::libc::c_uint)
            };
            Wrapper::from_ref(r)
        }
    }

    fn new_from_str(c: &Context, data: &str, null_term: bool) -> Constant<ty::Array<T>> {
        unsafe {
            let cr = c.to_ref();
            let no_null_term = if null_term { False } else { True };
            let r = do data.as_imm_buf |s,len| {
                constant::LLVMConstStringInContext(cr,
                                                   s as *std::libc::c_char,
                                                   len as std::libc::c_uint,
                                                   no_null_term)
            };

            Wrapper::from_ref(r)
        }
    }
}

impl<T:ty::Ty> ConstVector<T> for Constant<ty::Vector<T>> {
    fn new(els: ~[Constant<T>]) -> Constant<ty::Vector<T>> {
        unsafe {
            let llels = do els.map |e| { e.to_ref() };

            let r = do llels.as_imm_buf |b, len| {
                constant::LLVMConstVector(b, len as std::libc::c_uint)
            };

            Wrapper::from_ref(r)
        }
    }
}

impl ConstStruct for Constant<ty::Struct> {
    fn new(c: Context, members: ~[Constant<ty::Type>], packed: bool) -> Constant<ty::Struct> {
        unsafe {
            let cr = c.to_ref();
            let llm = do members.map |m| { m.to_ref() };
            let packed = if packed { True } else { False };

            let r = do llm.as_imm_buf |b, len| {
                constant::LLVMConstStructInContext(cr, b, len as std::libc::c_uint, packed)
            };

            Wrapper::from_ref(r)
        }
    }

    fn named(ty: ty::Struct, members: ~[Constant<ty::Type>]) -> Constant<ty::Struct> {
        unsafe {
            let llm = do members.map |m| { m.to_ref() };

            let r = do llm.as_imm_buf |b, len| {
                constant::LLVMConstNamedStruct(ty.to_ref(), b, len as std::libc::c_uint)
            };

            Wrapper::from_ref(r)
        }
    }
}


pub struct Global<T> {
    priv r: ValueRef
}

impl<T:ty::Ty> Wrapper<ValueRef> for Global<T> {
    fn from_ref(R: ValueRef) -> Global<T> {
        Global {
            r: R
        }
    }

    fn to_ref(&self) -> ValueRef {
        self.r
    }
}

impl<T:ty::Ty> Val<T> for Global<T> {}
impl<T:ty::Ty> ConstVal<T> for Global<T> {}
impl<T:ty::Ty> GlobalVal<T> for Global<T> {}

impl<T:ty::Ty> GlobalVar<T> for Global<T> {
    fn delete(&self) {
        unsafe {
            global::LLVMDeleteGlobal(self.to_ref());
        }
    }

    fn get_initializer(&self) -> Constant<T> {
        unsafe {
            let r = global::LLVMGetInitializer(self.r);
            Wrapper::from_ref(r)
        }
    }

    fn set_initializer(&mut self, val: Constant<T>) {
        unsafe {
            let r = val.to_ref();
            global::LLVMSetInitializer(self.r, r);
        }
    }


    fn is_thread_local(&self) -> bool {
        unsafe {
            global::LLVMIsThreadLocal(self.r) == True
        }
    }

    fn set_thread_local(&mut self, thread_local: bool) {
        unsafe {
            let tl = if thread_local { True } else { False };

            global::LLVMSetThreadLocal(self.r, tl);
        }
    }


    fn is_global_constant(&self) -> bool {
        unsafe {
            global::LLVMIsGlobalConstant(self.r) == True
        }
    }

    fn set_global_constant(&mut self, global_constant: bool) {
        unsafe{
            let gc = if global_constant { True } else { False };
            global::LLVMSetGlobalConstant(self.r, gc);
        }
    }


    fn get_thread_local_mode(&self) -> ThreadLocalMode {
        unsafe {
            global::LLVMGetThreadLocalMode(self.r)
        }
    }

    fn set_thread_local_mode(&mut self, mode: ThreadLocalMode) {
        unsafe {
            global::LLVMSetThreadLocalMode(self.r, mode);
        }
    }


    fn is_externally_initialized(&self) -> bool {
        unsafe {
            global::LLVMIsExternallyInitialized(self.r) == True
        }
    }

    fn set_externally_initialized(&self, extern_init: bool) {
        unsafe {
            let ei = if extern_init { True } else { False };
            global::LLVMSetExternallyInitialized(self.r, ei);
        }
    }
}

pub type Function = Global<ty::Function>;

impl FunctionVal for Global<ty::Function> {
    fn delete(&self) {
        unsafe {
            function::LLVMDeleteFunction(self.r);
        }
    }


    fn intrinsic_id(&self) -> uint {
        unsafe {
            function::LLVMGetIntrinsicID(self.r) as uint
        }
    }


    fn get_callconv(&self) -> uint {
        unsafe {
            function::LLVMGetFunctionCallConv(self.r) as uint
        }
    }

    fn set_callconv(&mut self, cc: uint) {
        unsafe {
            function::LLVMSetFunctionCallConv(self.r, cc as std::libc::c_uint);
        }
    }


    fn get_gc(&self) -> ~str {
        unsafe {
            let s = function::LLVMGetGC(self.r);
            str::raw::from_c_str(s)
        }
    }

    fn set_gc(&mut self, name: &str) {
        unsafe {
            do name.with_c_str |s| {
                function::LLVMSetGC(self.r, s);
            }
        }
    }


    fn add_attr(&mut self, attr: Attribute) {
        unsafe {
            function::LLVMAddFunctionAttr(self.r, attr as std::libc::c_ulonglong);
        }
    }

    fn get_attr(&self) -> u64 {
        unsafe {
            let a = function::LLVMGetFunctionAttr(self.r);
            a as u64
        }
    }

    fn remove_attr(&mut self, attr: Attribute) {
        unsafe {
            function::LLVMRemoveFunctionAttr(self.r, attr as std::libc::c_ulonglong);
        }
    }


    fn params(&self) -> ~[Param<ty::Type>] {
        unsafe {
            let num_params = function::LLVMCountParams(self.r) as uint;
            let mut buf : ~[ValueRef] = vec::with_capacity(num_params);
            function::LLVMGetParams(self.r, vec::raw::to_mut_ptr(buf));
            do buf.map |&VR| {
                let t : Param<ty::Type> = Wrapper::from_ref(VR);
                t
            }
        }
    }

    fn get_param(&self, idx: uint) -> Param<ty::Type> {
        unsafe {
            let r = function::LLVMGetParam(self.r, idx as std::libc::c_uint);
            Wrapper::from_ref(r)
        }
    }

    fn add_basic_block(&mut self, c: &Context, name: &str) -> BasicBlock {
        unsafe {
            do name.with_c_str |s| {
                let r = bb::LLVMAppendBasicBlockInContext(c.to_ref(), self.r, s);
                Wrapper::from_ref(bb::LLVMBasicBlockAsValue(r))
            }
        }
    }
}

pub struct Param<T> {
    priv r: ValueRef
}

impl<T:ty::Ty> Wrapper<ValueRef> for Param<T> {
    fn from_ref(R: ValueRef) -> Param<T> {
        Param {
            r: R
        }
    }

    fn to_ref(&self) -> ValueRef {
        self.r
    }
}

impl<T:ty::Ty> Val<T> for Param<T> { }

impl<T:ty::Ty> ParamVal<T> for Param<T> {
    fn add_attr(&mut self, attr: Attribute) {
        unsafe {
            function::LLVMAddAttribute(self.r, attr as std::libc::c_ulonglong);
        }
    }

    fn remove_attr(&mut self, attr: Attribute) {
        unsafe {
            function::LLVMRemoveAttribute(self.r, attr as std::libc::c_ulonglong);
        }
    }

    fn get_attribute(&self) -> u64 {
        unsafe {
            let a = function::LLVMGetAttribute(self.r);
            a as u64
        }
    }

    fn set_alignment(&mut self, align: uint) {
        unsafe {
            function::LLVMSetParamAlignment(self.r, align as std::libc::c_uint);
        }
    }
}

pub type Metadata = Value<ty::Metadata>;

impl MDVal for Value<ty::Metadata> {
    fn operands(&self) -> ~[Value<ty::Type>] {
        unsafe {
            let num_ops = metadata::LLVMGetMDNodeNumOperands(self.r) as uint;
            let mut buf : ~[ValueRef] = vec::with_capacity(num_ops);
            metadata::LLVMGetMDNodeOperands(self.r, vec::raw::to_mut_ptr(buf));
            do buf.map |&VR| {
                let t : Value<ty::Type> = Wrapper::from_ref(VR);
                t
            }
        }
    }

    fn get_string(&self) -> ~str {
        unsafe {
            let mut len = 0 as std::libc::c_uint;
            let s = metadata::LLVMGetMDString(self.r, &mut len);
            str::raw::from_buf_len(s as *u8, len as uint)
        }
    }
}

impl Value<ty::Metadata> {
    fn new_string(c: Context, data: &str) -> Metadata {
        unsafe {
            let cr = c.to_ref();
            let r = do data.as_imm_buf |s, len| {
                metadata::LLVMMDStringInContext(cr,
                                               s as *std::libc::c_char,
                                               len as std::libc::c_uint)
            };

            Wrapper::from_ref(r)
        }
    }

    fn new_node(c: Context, vals: ~[Value<ty::Type>]) -> Metadata {
        unsafe {
            let cr = c.to_ref();
            let llvs = do vals.map |v| { v.to_ref() };
            let r = do llvs.as_imm_buf |buf, len| {
                metadata::LLVMMDNodeInContext(cr, buf, len as std::libc::c_uint)
            };

            Wrapper::from_ref(r)
        }
    }
}

pub type BasicBlock = Value<ty::Label>;

impl BBVal for BasicBlock {
    fn parent(&self) -> Function {
        unsafe {
            let r = bb::LLVMGetBasicBlockParent(bb::LLVMValueAsBasicBlock(self.r));
            Wrapper::from_ref(r)
        }
    }

    fn terminator(&self) -> instruction::Instruction<ty::Type> {
        unsafe {
            let r = bb::LLVMGetBasicBlockTerminator(bb::LLVMValueAsBasicBlock(self.r));
            Wrapper::from_ref(r)
        }
    }


    fn insert(&self, c: Context, name: &str) -> BasicBlock {
        unsafe {
            do name.with_c_str |s| {
                let r = bb::LLVMInsertBasicBlockInContext(c.to_ref(),
                                                          bb::LLVMValueAsBasicBlock(self.r),
                                                          s);
                Wrapper::from_ref(bb::LLVMBasicBlockAsValue(r))
            }
        }
    }

    fn delete(&self) {
        unsafe {
            bb::LLVMDeleteBasicBlock(bb::LLVMValueAsBasicBlock(self.r));
        }
    }


    fn move_before(&self, before: BasicBlock) {
        unsafe {
            bb::LLVMMoveBasicBlockBefore(
                bb::LLVMValueAsBasicBlock(self.r),
                bb::LLVMValueAsBasicBlock(before.r));
        }
    }

    fn move_after(&self, after: BasicBlock) {
        unsafe {
            bb::LLVMMoveBasicBlockAfter(
                bb::LLVMValueAsBasicBlock(self.r),
                bb::LLVMValueAsBasicBlock(after.r));
        }
    }

}
