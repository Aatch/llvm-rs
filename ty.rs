use ffi::core::{TypeRef};
use ffi::core;
use super::*;
use std::vec;
use std::str;

pub trait Ty : Wrapper<TypeRef> {
    pub fn kind() -> Kind;
    pub fn is_sized(&self) -> bool;
}

macro_rules! type_wrap (
    ($t:ident) => (
        pub struct $t {
            priv r: core::TypeRef
        }
    )
)

macro_rules! impl_wrapper (
    ($t:ident) => (
        impl Wrapper<core::TypeRef> for $t {
            pub fn from_ref(R: core::TypeRef) -> $t {
                $t {
                    r: R
                }
            }

            pub fn to_ref(&self) -> core::TypeRef {
                self.r
            }
        }
    )
)

pub enum Kind {
    Void,
    Label,
    Real,
    Integer,
    Function,
    Struct,
    Array,
    Pointer,
    Vector,
    Metadata
}

type_wrap!(Type)
type_wrap!(Void)
type_wrap!(Label)
type_wrap!(Real)
type_wrap!(Integer)
type_wrap!(Function)
type_wrap!(Struct)
type_wrap!(Metadata)

impl_wrapper!(Type)
impl_wrapper!(Void)
impl_wrapper!(Label)
impl_wrapper!(Real)
impl_wrapper!(Integer)
impl_wrapper!(Function)
impl_wrapper!(Struct)
impl_wrapper!(Metadata)

pub struct Array<T> {
    priv r: TypeRef
}
pub struct Vector<T> {
    priv r: TypeRef
}
pub struct Pointer<T> {
    priv r: TypeRef
}

impl<T> Wrapper<TypeRef> for Array<T> {
    pub fn from_ref(R: TypeRef) -> Array<T> {
        Array {
            r: R
        }
    }

    pub fn to_ref(&self) -> TypeRef {
        self.r
    }
}

impl<T> Wrapper<TypeRef> for Vector<T> {
    pub fn from_ref(R: TypeRef) -> Vector<T> {
        Vector {
            r: R
        }
    }

    pub fn to_ref(&self) -> TypeRef {
        self.r
    }
}

impl<T> Wrapper<TypeRef> for Pointer<T> {
    pub fn from_ref(R: TypeRef) -> Pointer<T> {
        Pointer {
            r: R
        }
    }

    pub fn to_ref(&self) -> TypeRef {
        self.r
    }
}

impl Ty for Type {
    pub fn kind() -> Kind {
        fail!("Cannot get the kind of an unknown type")
    }

    pub fn is_sized(&self) -> bool {
        unsafe {
            core::types::LLVMTypeIsSized(self.r) == core::True
        }
    }
}

impl Type {
    pub fn try_cast<T:Ty>(&self) -> Option<T> {
        use std::cast;

        let llkind = unsafe { core::types::LLVMGetTypeKind(self.r) };
        let kind = Ty::kind::<T>();

        match (kind, llkind) {
            (Void, core::Void)          |
            (Label, core::Label)        |
            (Real, core::Half)          |
            (Real, core::Float)         |
            (Real, core::Double)        |
            (Real, core::X86_FP80)      |
            (Real, core::FP128)         |
            (Real, core::PPC_FP128)     |
            (Integer, core::Integer)    |
            (Function, core::Function)  |
            (Struct, core::Struct)      |
            (Metadata, core::Metadata) => {
                unsafe {
                    Some(cast::transmute::<Type,T>(*self))
                }
            }
            _ => {
                None
            }
        }
    }

    pub fn cast<T:Ty>(&self) -> T {
        self.try_cast().unwrap()
    }
}

impl Ty for Void {
    pub fn kind() -> Kind { Void }
    pub fn is_sized(&self) -> bool {
        unsafe {
            core::types::LLVMTypeIsSized(self.r) == core::True
        }
    }
}

impl Void {
    pub fn new(c: Context) -> Void {
        unsafe {
            let r = core::types::LLVMVoidTypeInContext(c.to_ref());
            Wrapper::from_ref(r)
        }
    }
}

impl Ty for Label {
    pub fn kind() -> Kind { Label }
    pub fn is_sized(&self) -> bool {
        unsafe {
            core::types::LLVMTypeIsSized(self.r) == core::True
        }
    }
}

impl Label {
    pub fn new(c: Context) -> Label {
        unsafe {
            let r = core::types::LLVMLabelTypeInContext(c.to_ref());
            Wrapper::from_ref(r)
        }
    }
}

impl Ty for Real {
    pub fn kind() -> Kind { Real }
    pub fn is_sized(&self) -> bool {
        unsafe {
            core::types::LLVMTypeIsSized(self.r) == core::True
        }
    }
}

impl Real {
    pub fn new_half(c: Context) -> Real {
        unsafe {
            let r = core::types::LLVMHalfTypeInContext(c.to_ref());
            Wrapper::from_ref(r)
        }
    }
    pub fn new_float(c: Context) -> Real {
        unsafe {
            let r = core::types::LLVMFloatTypeInContext(c.to_ref());
            Wrapper::from_ref(r)
        }
    }
    pub fn new_double(c: Context) -> Real {
        unsafe {
            let r = core::types::LLVMDoubleTypeInContext(c.to_ref());
            Wrapper::from_ref(r)
        }
    }
    pub fn new_x86fp80(c: Context) -> Real {
        unsafe {
            let r = core::types::LLVMX86FP80TypeInContext(c.to_ref());
            Wrapper::from_ref(r)
        }
    }
    pub fn new_fp128(c: Context) -> Real {
        unsafe {
            let r = core::types::LLVMFP128TypeInContext(c.to_ref());
            Wrapper::from_ref(r)
        }
    }
    pub fn new_ppcfp128(c: Context) -> Real {
        unsafe {
            let r = core::types::LLVMPPCFP128TypeInContext(c.to_ref());
            Wrapper::from_ref(r)
        }
    }
}

impl Ty for Integer {
    pub fn kind() -> Kind { Integer }
    pub fn is_sized(&self) -> bool {
        unsafe {
            core::types::LLVMTypeIsSized(self.r) == core::True
        }
    }
}

impl Integer {
    pub fn new_i1(c:Context) -> Integer {
        unsafe {
            let r = core::types::LLVMInt1TypeInContext(c.to_ref());
            Wrapper::from_ref(r)
        }
    }
    pub fn new_i8(c:Context) -> Integer {
        unsafe {
            let r = core::types::LLVMInt8TypeInContext(c.to_ref());
            Wrapper::from_ref(r)
        }
    }
    pub fn new_i16(c:Context) -> Integer {
        unsafe {
            let r = core::types::LLVMInt16TypeInContext(c.to_ref());
            Wrapper::from_ref(r)
        }
    }
    pub fn new_i32(c:Context) -> Integer {
        unsafe {
            let r = core::types::LLVMInt32TypeInContext(c.to_ref());
            Wrapper::from_ref(r)
        }
    }
    pub fn new_i64(c:Context) -> Integer {
        unsafe {
            let r = core::types::LLVMInt64TypeInContext(c.to_ref());
            Wrapper::from_ref(r)
        }
    }
    pub fn new_from_width(c:Context, bits: uint) -> Integer {
        unsafe {
            let r = core::types::LLVMIntTypeInContext(c.to_ref(), bits as std::libc::c_uint);
            Wrapper::from_ref(r)
        }
    }
    pub fn width(&self) -> uint {
        unsafe {
            core::types::LLVMGetIntTypeWidth(self.r) as uint
        }
    }
}

impl Ty for Function {
    pub fn kind() -> Kind { Function }
    pub fn is_sized(&self) -> bool {
        unsafe {
            core::types::LLVMTypeIsSized(self.r) == core::True
        }
    }
}

impl Function {
    pub fn new(ret: Type, params: &[Type], is_var_arg: bool) -> Function {
        let llret = ret.to_ref();
        let llparams = do params.map |t| {
            t.to_ref()
        };
        let is_var_arg = if is_var_arg { core::True } else { core::False };
        let r = unsafe {
            do vec::as_imm_buf(llparams) |b, len| {
                core::types::LLVMFunctionType(llret, b, len as std::libc::c_uint, is_var_arg)
            }
        };
        Wrapper::from_ref(r)
    }

    pub fn is_var_arg(&self) -> bool {
        unsafe {
            core::types::LLVMIsFunctionVarArg(self.r) == core::True
        }
    }

    pub fn return_type(&self) -> Type {
        unsafe {
            let r = core::types::LLVMGetReturnType(self.r);
            Wrapper::from_ref(r)
        }
    }

    pub fn params(&self) -> ~[Type] {
        unsafe {
            let num_params = core::types::LLVMCountParamTypes(self.r) as uint;
            let mut buf : ~[core::TypeRef] = vec::with_capacity(num_params);
            core::types::LLVMGetParamTypes(self.r, vec::raw::to_mut_ptr(buf));
            do buf.map |&VR| {
                let t : Type = Wrapper::from_ref(VR);
                t
            }
        }
    }
}

impl Ty for Struct {
    pub fn kind() -> Kind { Struct }
    pub fn is_sized(&self) -> bool {
        unsafe {
            core::types::LLVMTypeIsSized(self.r) == core::True
        }
    }
}

impl Struct {
    pub fn new(c: Context, elements: &[Type], packed: bool) -> Struct {
        let cr = c.to_ref();
        let llelems = do elements.map |t| {
            t.to_ref()
        };
        let packed = if packed { core::True } else { core::False };
        let r = unsafe {
            do vec::as_imm_buf(llelems) |b, len| {
                core::types::LLVMStructTypeInContext(cr, b, len as std::libc::c_uint, packed)
            }
        };
        Wrapper::from_ref(r)
    }

    pub fn new_named(c: Context, name: &str, elements: &[Type], packed: bool) -> Struct {
        unsafe {
            let cr = c.to_ref();
            let r = do str::as_c_str(name) |s| {
                core::types::LLVMStructCreateNamed(cr, s)
            };


            let llelems = do elements.map |t| {
                t.to_ref()
            };

            let packed = if packed { core::True } else { core::False };
            do vec::as_imm_buf(llelems) |b, len| {
                core::types::LLVMStructSetBody(r, b, len as std::libc::c_uint, packed);
            }

            Wrapper::from_ref(r)
        }
    }

    pub fn get_name(&self) -> ~str {
        unsafe {
            let buf = core::types::LLVMGetStructName(self.r);
            str::raw::from_c_str(buf)
        }
    }

    pub fn elements(&self) -> ~[Type] {
        unsafe {
            let num_elems = core::types::LLVMCountStructElementTypes(self.r) as uint;
            let mut buf : ~[core::TypeRef] = vec::with_capacity(num_elems);
            core::types::LLVMGetStructElementTypes(self.r, vec::raw::to_mut_ptr(buf));
            do buf.map |&VR| {
                let t : Type = Wrapper::from_ref(VR);
                t
            }
        }
    }

    pub fn is_packed(&self) -> bool {
        unsafe {
            core::types::LLVMIsPackedStruct(self.r) == core::True
        }
    }

    pub fn is_opaque(&self) -> bool {
        unsafe {
            core::types::LLVMIsOpaqueStruct(self.r) == core::True
        }
    }
}

impl<T> Ty for Array<T> {
    pub fn kind() -> Kind { Array }
    pub fn is_sized(&self) -> bool {
        unsafe {
            core::types::LLVMTypeIsSized(self.r) == core::True
        }
    }
}

impl<T:Ty> Array<T> {
    pub fn new(ty: T, size: uint) -> Vector<T> {
        unsafe {
            let r = core::types::LLVMArrayType(ty.to_ref(), size as std::libc::c_uint);
            Wrapper::from_ref(r)
        }
    }

    pub fn element_type(&self) -> T {
        unsafe {
            let r = core::types::LLVMGetElementType(self.r);
            Wrapper::from_ref(r)
        }
    }

    pub fn size(&self) -> uint {
        unsafe {
            core::types::LLVMGetArrayLength(self.r) as uint
        }
    }
}

impl<T> Ty for Vector<T> {
    pub fn kind() -> Kind { Vector }
    pub fn is_sized(&self) -> bool {
        unsafe {
            core::types::LLVMTypeIsSized(self.r) == core::True
        }
    }
}

impl<T:Ty> Vector<T> {
    pub fn new(ty: T, size: uint) -> Vector<T> {
        unsafe {
            let r = core::types::LLVMVectorType(ty.to_ref(), size as std::libc::c_uint);
            Wrapper::from_ref(r)
        }
    }

    pub fn element_type(&self) -> T {
        unsafe {
            let r = core::types::LLVMGetElementType(self.r);
            Wrapper::from_ref(r)
        }
    }

    pub fn size(&self) -> uint {
        unsafe {
            core::types::LLVMGetVectorSize(self.r) as uint
        }
    }
}


impl<T> Ty for Pointer<T> {
    pub fn kind() -> Kind { Pointer }
    pub fn is_sized(&self) -> bool {
        unsafe {
            core::types::LLVMTypeIsSized(self.r) == core::True
        }
    }
}

impl<T:Ty> Pointer<T> {
    pub fn new(ty: T, address_space: uint) -> Pointer<T> {
        unsafe {
            let r = core::types::LLVMPointerType(ty.to_ref(), address_space as std::libc::c_uint);
            Wrapper::from_ref(r)
        }
    }

    pub fn pointee_type(&self) -> T {
        unsafe {
            let r = core::types::LLVMGetElementType(self.r);
            Wrapper::from_ref(r)
        }
    }

    pub fn address_space(&self) -> uint {
        unsafe {
            core::types::LLVMGetPointerAddressSpace(self.r) as uint
        }
    }
}
