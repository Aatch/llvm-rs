#[crate_id = "llvm#0.1"];
#[crate_type = "lib"];

use ffi::core;

use std::ptr;
use std::kinds::marker;

pub mod ffi {
    pub mod core;
}

pub mod ty;
pub mod val;

#[unsafe_no_drop_flag]
pub struct Context {
    priv r: core::LLVMContextRef,
}

#[unsafe_no_drop_flag]
pub struct Module<'a> {
    priv r: core::LLVMModuleRef,
    priv lifetime: marker::ContravariantLifetime<'a>
}

impl Context {

    pub fn new() -> Context {
        unsafe {
            Context {
                r: core::LLVMContextCreate()
            }
        }
    }

    pub fn create_module<'a>(&'a self, name: &str) -> Module<'a> {
        let c_str = name.to_c_str();
        let module = c_str.with_ref(|c_buffer| {
            unsafe {
                core::LLVMModuleCreateWithNameInContext(c_buffer, self.r)
            }
        });

        Module {
            r: module,
            lifetime: marker::ContravariantLifetime
        }
    }

    pub fn new_int1<'a>(&'a self) -> ty::Ty<'a, ty::IntegerType> {
        unsafe {
            ty::Ty::from_ref(core::LLVMInt1TypeInContext(self.r))
        }
    }

    pub fn new_int8<'a>(&'a self) -> ty::Ty<'a, ty::IntegerType> {
        unsafe {
            ty::Ty::from_ref(core::LLVMInt8TypeInContext(self.r))
        }
    }

    pub fn new_int16<'a>(&'a self) -> ty::Ty<'a, ty::IntegerType> {
        unsafe {
            ty::Ty::from_ref(core::LLVMInt16TypeInContext(self.r))
        }
    }

    pub fn new_int32<'a>(&'a self) -> ty::Ty<'a, ty::IntegerType> {
        unsafe {
            ty::Ty::from_ref(core::LLVMInt32TypeInContext(self.r))
        }
    }

    pub fn new_int64<'a>(&'a self) -> ty::Ty<'a, ty::IntegerType> {
        unsafe {
            ty::Ty::from_ref(core::LLVMInt64TypeInContext(self.r))
        }
    }

    pub fn new_int<'a>(&'a self, bits: uint) -> ty::Ty<'a, ty::IntegerType> {
        use std::libc;
        unsafe {
            ty::Ty::from_ref(core::LLVMIntTypeInContext(self.r, bits as libc::c_uint))
        }
    }

    pub fn new_half<'a>(&'a self) -> ty::Ty<'a, ty::Type> {
        unsafe {
            ty::Ty::from_ref(core::LLVMHalfTypeInContext(self.r))
        }
    }

    pub fn new_float<'a>(&'a self) -> ty::Ty<'a, ty::Type> {
        unsafe {
            ty::Ty::from_ref(core::LLVMFloatTypeInContext(self.r))
        }
    }

    pub fn new_double<'a>(&'a self) -> ty::Ty<'a, ty::Type> {
        unsafe {
            ty::Ty::from_ref(core::LLVMDoubleTypeInContext(self.r))
        }
    }

    pub fn new_x86_fp80<'a>(&'a self) -> ty::Ty<'a, ty::Type> {
        unsafe {
            ty::Ty::from_ref(core::LLVMX86FP80TypeInContext(self.r))
        }
    }

    pub fn new_fp128<'a>(&'a self) -> ty::Ty<'a, ty::Type> {
        unsafe {
            ty::Ty::from_ref(core::LLVMFP128TypeInContext(self.r))
        }
    }

    pub fn new_ppc_fp128<'a>(&'a self) -> ty::Ty<'a, ty::Type> {
        unsafe {
            ty::Ty::from_ref(core::LLVMPPCFP128TypeInContext(self.r))
        }
    }

    pub fn new_void<'a>(&'a self) -> ty::Ty<'a, ty::Type> {
        unsafe {
            ty::Ty::from_ref(core::LLVMVoidTypeInContext(self.r))
        }
    }

    pub fn new_label<'a>(&'a self) -> ty::Ty<'a, ty::Type> {
        unsafe {
            ty::Ty::from_ref(core::LLVMLabelTypeInContext(self.r))
        }
    }

    pub fn new_x86mmx<'a>(&'a self) -> ty::Ty<'a, ty::Type> {
        unsafe {
            ty::Ty::from_ref(core::LLVMX86MMXTypeInContext(self.r))
        }
    }

    pub fn named_struct<'a>(&'a self, name: &str) -> ty::Ty<'a, ty::StructType> {
        let c_str = name.to_c_str();
        let r = c_str.with_ref(|c_buffer| {
            unsafe {
                core::LLVMStructCreateNamed(self.r, c_buffer)
            }
        });

        ty::Ty::from_ref(r)
    }

    pub fn new_struct<'a, T:ty::TypeList>(&'a self, types: T, packed: bool)
        -> ty::Ty<'a, ty::StructType> {
        use std::{vec,libc};
        static SMALL_LIST : uint = 16;

        let types_len = types.length();
        let r = if types_len <= SMALL_LIST {
            let mut type_refs = [0 as core::LLVMTypeRef,..SMALL_LIST];
            for i in range(0, types_len) {
                type_refs[i] = types.at(i);
            }

            unsafe {
                core::LLVMStructTypeInContext(self.r,
                    &type_refs[0],
                    types_len as libc::c_uint,
                    if packed { core::TRUE } else { core::FALSE })
            }
        } else {
            let mut type_refs = vec::with_capacity(types_len);
            for i in range(0, types_len) {
                type_refs.push(types.at(i));
            }

            unsafe {
                core::LLVMStructTypeInContext(self.r,
                    &type_refs[0],
                    types_len as libc::c_uint,
                    if packed { core::TRUE } else { core::FALSE })
            }
        };

        ty::Ty::from_ref(r)
    }
}

impl<'a> Module<'a> {

    pub fn get_datalayout<'s>(&'s self) -> &'s str {
        use std::cast::transmute;
        use std::c_str;
        unsafe {
            let datalayout = core::LLVMGetDataLayout(self.r);
            if datalayout.is_null() { return ""; }

            let c_str = c_str::CString::new(datalayout, false);
            match c_str.as_str() {
                Some(s) => transmute(s),
                None => ""
            }
        }
    }

    pub fn set_datalayout(&mut self, layout: &str) {
        let c_str = layout.to_c_str();
        c_str.with_ref(|c_buffer| {
            unsafe {
                core::LLVMSetDataLayout(self.r, c_buffer);
            }
        });
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            if !self.r.is_null() {
                core::LLVMContextDispose(self.r);
                self.r = ptr::null();
            }
        }
    }
}

impl<'a> Drop for Module<'a> {
    fn drop(&mut self) {
        unsafe {
            if !self.r.is_null() {
                core::LLVMDisposeModule(self.r);
                self.r = ptr::null();
            }
        }
    }
}
