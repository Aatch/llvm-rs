use ffi::core;
use std::kinds::marker;
use std::vec;

pub struct Ty<'a, T> {
    priv r: core::LLVMTypeRef,
    priv ty: marker::CovariantType<T>,
    priv lifetime: marker::ContravariantLifetime<'a>
}

pub mod traits {
    pub trait Type {}
    pub trait CompositeType : Type {}

    pub trait SequentialType : CompositeType {}

    pub trait ArrayType : SequentialType {}
    pub trait PointerType : SequentialType {}
    pub trait VectorType : SequentialType {}
}

impl<'a, T:traits::Type> Ty<'a, T> {
    pub fn is_sized(&self) -> bool {
        unsafe {
            core::LLVMTypeIsSized(self.r) == core::TRUE
        }
    }

    pub fn from_ref(r: core::LLVMTypeRef) -> Ty<'a, T> {
        Ty {
            r: r,
            ty: marker::CovariantType,
            lifetime: marker::ContravariantLifetime
        }
    }

}

impl<'a> Ty<'a, IntegerType> {
    pub fn type_width(&self) -> uint {
        unsafe {
            core::LLVMGetIntTypeWidth(self.r) as uint
        }
    }
}

impl<'a> Ty<'a, FunctionType> {
    pub fn function_type<R:traits::Type, P:TypeList>(return_type: Ty<'a, R>,
        params: &P, is_var_arg: bool) -> Ty<'a, FunctionType> {
        use std::{vec,libc};
        static SMALL_LIST : uint = 16;

        let params_len = params.length();
        if params_len <= SMALL_LIST {
            let mut param_refs = [0 as core::LLVMTypeRef,..SMALL_LIST];
            for i in range(0, params_len) {
                param_refs[i] = params.at(i);
            }

            unsafe {
                Ty::from_ref(core::LLVMFunctionType(return_type.r,
                    &param_refs[0],
                    params_len as libc::c_uint,
                    if is_var_arg { core::TRUE } else { core::FALSE }))
            }
        } else {
            let mut param_refs = vec::with_capacity(params_len);
            for i in range(0, params_len) {
                param_refs.push(params.at(i));
            }

            unsafe {
                Ty::from_ref(core::LLVMFunctionType(return_type.r,
                    &param_refs[0],
                    params_len as libc::c_uint,
                    if is_var_arg { core::TRUE } else { core::FALSE }))
            }
        }
    }

    pub fn is_var_arg(&self) -> bool {
        unsafe {
            core::LLVMIsFunctionVarArg(self.r) == core::TRUE
        }
    }

    pub fn return_type(&self) -> Ty<'a, Type> {
        unsafe {
            let ty_ref = core::LLVMGetReturnType(self.r);
            Ty {
                r: ty_ref,
                ty: marker::CovariantType,
                lifetime: marker::ContravariantLifetime
            }
        }
    }

    pub fn count_params(&self) -> uint {
        unsafe {
            core::LLVMCountParamTypes(self.r) as uint
        }
    }

    pub fn param_types(&self) -> ~[Ty<'a, Type>] {
        use std::cast;

        let nparams = self.count_params();
        let mut tys = vec::with_capacity(nparams);
        unsafe {
            tys.set_len(nparams);
            core::LLVMGetParamTypes(self.r, &mut tys[0]);
            cast::transmute(tys)
        }
    }
}

impl<'a> Ty<'a, StructType> {
    pub fn name<'s>(&'s self) -> &'s str {
        use std::cast::transmute;
        use std::c_str;
        unsafe {
            let name = core::LLVMGetStructName(self.r);
            if name.is_null() { return ""; }

            let c_str = c_str::CString::new(name, false);
            match c_str.as_str() {
                Some(s) => transmute(s),
                None => ""
            }
        }
    }

    pub fn set_body<T:TypeList>(&mut self, types: &T, packed: bool) {
        use std::{vec,libc};
        static SMALL_LIST : uint = 16;

        let types_len = types.length();
        if types_len <= SMALL_LIST {
            let mut type_refs = [0 as core::LLVMTypeRef,..SMALL_LIST];
            for i in range(0, types_len) {
                type_refs[i] = types.at(i);
            }

            unsafe {
                core::LLVMStructSetBody(self.r,
                    &type_refs[0],
                    types_len as libc::c_uint,
                    if packed { core::TRUE } else { core::FALSE });
            }
        } else {
            let mut type_refs = vec::with_capacity(types_len);
            for i in range(0, types_len) {
                type_refs.push(types.at(i));
            }

            unsafe {
                core::LLVMStructSetBody(self.r,
                    &type_refs[0],
                    types_len as libc::c_uint,
                    if packed { core::TRUE } else { core::FALSE });
            }
        }
    }

    pub fn count_elements(&self) -> uint {
        unsafe {
            core::LLVMCountStructElementTypes(self.r) as uint
        }
    }

    pub fn element_types(&self) -> ~[Ty<'a, Type>] {
        use std::cast;

        let nels = self.count_elements();
        let mut tys = vec::with_capacity(nels);
        unsafe {
            tys.set_len(nels);
            core::LLVMGetStructElementTypes(self.r, &mut tys[0]);
            cast::transmute(tys)
        }
    }

    pub fn is_packed(&self) -> bool {
        unsafe {
            core::LLVMIsPackedStruct(self.r) == core::TRUE
        }
    }

    pub fn is_opaque(&self) -> bool {
        unsafe {
            core::LLVMIsOpaqueStruct(self.r) == core::TRUE
        }
    }
}

impl<'a, T:traits::SequentialType> Ty<'a, T> {
    pub fn element_type(&self) -> Ty<'a, Type> {
        unsafe {
            let ty = core::LLVMGetElementType(self.r);
            Ty {
                r: ty,
                ty: marker::CovariantType,
                lifetime: marker::ContravariantLifetime
            }
        }
    }
}

impl<'a> Ty<'a, ArrayType> {
    pub fn array_type<T:traits::Type>(element_type: &Ty<'a, T>,
        element_count: uint) -> Ty<'a, ArrayType> {
        use std::libc;

        unsafe {
            let r = core::LLVMArrayType(element_type.r, element_count as libc::c_uint);
            Ty {
                r: r,
                ty: marker::CovariantType,
                lifetime: marker::ContravariantLifetime
            }
        }
    }

    pub fn array_length(&self) -> uint {
        unsafe {
            core::LLVMGetArrayLength(self.r) as uint
        }
    }
}

impl<'a> Ty<'a, PointerType> {
    pub fn pointer_type<T:traits::Type>(element_type: &Ty<'a, T>,
        address_space: uint) -> Ty<'a, PointerType> {
        use std::libc;

        unsafe {
            let r = core::LLVMPointerType(element_type.r, address_space as libc::c_uint);
            Ty {
                r: r,
                ty: marker::CovariantType,
                lifetime: marker::ContravariantLifetime
            }
        }
    }

    pub fn address_space(&self) -> uint {
        unsafe {
            core::LLVMGetPointerAddressSpace(self.r) as uint
        }
    }
}

impl<'a> Ty<'a, VectorType> {
    pub fn vector_type<T:traits::Type>(element_type: &Ty<'a, T>,
        element_count: uint) -> Ty<'a, VectorType> {
        use std::libc;

        unsafe {
            let r = core::LLVMArrayType(element_type.r, element_count as libc::c_uint);
            Ty {
                r: r,
                ty: marker::CovariantType,
                lifetime: marker::ContravariantLifetime
            }
        }
    }

    pub fn vector_size(&self) -> uint {
        unsafe {
            core::LLVMGetVectorSize(self.r) as uint
        }
    }
}

pub trait TypeList {
    fn length(&self) -> uint;
    fn at(&self, index: uint) -> core::LLVMTypeRef;
}

impl<'a, T:traits::Type> TypeList for Ty<'a, T> {
    fn length(&self) -> uint { 1 }
    fn at(&self, idx: uint) -> core::LLVMTypeRef {
        assert_eq!(idx, 0);
        self.r
    }
}

impl<'a, 'b, T:traits::Type> TypeList for &'a [Ty<'b, T>] {
    fn length(&self) -> uint {
        self.len()
    }
    fn at(&self, index: uint) -> core::LLVMTypeRef {
        self[index].r
    }
}

impl<'a, T:traits::Type> TypeList for (Ty<'a, T>,) {
    fn length(&self) -> uint { 1 }
    fn at(&self, index: uint) -> core::LLVMTypeRef {
        assert_eq!(index, 0);
        let &(ref t,) = self;
        t.r
    }
}

impl<'a, T1:traits::Type, T2:traits::Type> TypeList for (Ty<'a, T1>,Ty<'a, T2>) {
    fn length(&self) -> uint { 2 }
    fn at(&self, index: uint) -> core::LLVMTypeRef {
        assert!(index < 2);
        match index {
            0 => self.ref0().r,
            1 => self.ref1().r,
            _ => unreachable!()
        }
    }
}

impl<'a,
    T1:traits::Type,
    T2:traits::Type,
    T3:traits::Type
> TypeList for
    (Ty<'a, T1>,
     Ty<'a, T2>,
     Ty<'a, T3>
) {
    fn length(&self) -> uint { 3 }
    fn at(&self, index: uint) -> core::LLVMTypeRef {
        assert!(index < 3);
        match index {
            0 => self.ref0().r,
            1 => self.ref1().r,
            2 => self.ref2().r,
            _ => unreachable!()
        }
    }
}

impl<'a,
    T1:traits::Type,
    T2:traits::Type,
    T3:traits::Type,
    T4:traits::Type
> TypeList for
    (Ty<'a, T1>,
     Ty<'a, T2>,
     Ty<'a, T3>,
     Ty<'a, T4>
) {
    fn length(&self) -> uint { 4 }
    fn at(&self, index: uint) -> core::LLVMTypeRef {
        assert!(index < 4);
        match index {
            0 => self.ref0().r,
            1 => self.ref1().r,
            2 => self.ref2().r,
            3 => self.ref3().r,
            _ => unreachable!()
        }
    }
}

impl<'a,
    T1:traits::Type,
    T2:traits::Type,
    T3:traits::Type,
    T4:traits::Type,
    T5:traits::Type
> TypeList for
    (Ty<'a, T1>,
     Ty<'a, T2>,
     Ty<'a, T3>,
     Ty<'a, T4>,
     Ty<'a, T5>
) {
    fn length(&self) -> uint { 5 }
    fn at(&self, index: uint) -> core::LLVMTypeRef {
        assert!(index < 5);
        match index {
            0 => self.ref0().r,
            1 => self.ref1().r,
            2 => self.ref2().r,
            3 => self.ref3().r,
            4 => self.ref4().r,
            _ => unreachable!()
        }
    }
}

pub enum Type {}
pub enum CompositeType {}
pub enum FunctionType {}
pub enum IntegerType {}
pub enum StructType {}
pub enum SequentialType {}
pub enum ArrayType {}
pub enum PointerType {}
pub enum VectorType {}

impl traits::Type for Type {}
impl traits::Type for CompositeType {}
impl traits::Type for FunctionType {}
impl traits::Type for IntegerType {}
impl traits::Type for SequentialType {}
impl traits::Type for StructType {}
impl traits::Type for ArrayType {}
impl traits::Type for PointerType {}
impl traits::Type for VectorType {}

impl traits::CompositeType for CompositeType {}
impl traits::CompositeType for SequentialType {}
impl traits::CompositeType for StructType {}
impl traits::CompositeType for ArrayType {}
impl traits::CompositeType for PointerType {}
impl traits::CompositeType for VectorType {}

impl traits::SequentialType for SequentialType {}
impl traits::SequentialType for ArrayType {}
impl traits::SequentialType for PointerType {}
impl traits::SequentialType for VectorType {}
