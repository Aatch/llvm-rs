#[link(
    name = "llvm",
    vers = "0.1"
    )];
#[crate_type="lib"];

use ffi::core::*;
use std::str;
use std::vec;
use std::ptr;

pub mod ffi {
    pub mod core;
    pub mod exec_engine;
    pub mod target_machine;
}

pub mod ty;
pub mod value;
pub mod instruction;
pub mod ir_builder;

pub struct Context {
    priv r: ContextRef,
}

pub struct Module<'self> {
    priv r: ModuleRef,
    priv ctx: &'self Context
}

pub trait Wrapper<T> {
    pub fn from_ref(R:T) -> Self;
    pub fn to_ref(&self) -> T;
}

impl Wrapper<ContextRef> for Context {
    pub fn from_ref(R: ContextRef) -> Context {
        Context {
            r: R,
        }
    }

    pub fn to_ref(&self) -> ContextRef {
        self.r
    }
}

pub fn initialize_core() {
    unsafe {
        let R = passes::LLVMGetGlobalPassRegistry();
        LLVMInitializeCore(R);
    }
}

pub fn shutdown() {
    unsafe {
        LLVMShutdown();
    }
}

pub fn start_multithreaded() -> bool {
    unsafe {
        LLVMStartMultithreaded() == True
    }
}

pub fn stop_multithreaded() {
    unsafe {
        LLVMStopMultithreaded();
    }
}

pub fn is_multithreaded() -> bool {
    unsafe {
        LLVMIsMultithreaded() == True
    }
}

impl Context {
    pub fn new() -> Context {
        unsafe {
            Context {
                r: context::LLVMContextCreate(),
            }
        }
    }

    pub fn get_md_kind_id(&self, name: &str) -> uint {
        use std::libc::{c_char, c_uint};
        unsafe {
            do str::as_buf(name) |s,len| {
                let s = s as *c_char;
                context::LLVMGetMDKindIDInContext(self.r, s, len as c_uint) as uint
            }
        }
    }

    pub fn new_module<'r>(&'r self, name: &str) -> Module<'r> {
        unsafe {
            do str::as_c_str(name) |s| {
                let MR = module::LLVMModuleCreateWithNameInContext(s, self.r);
                Module {
                    r: MR,
                    ctx: self
                }
            }
        }
    }
}

impl Drop for Context {
    pub fn finalize(&self) {
        unsafe {
            context::LLVMContextDispose(self.r);
        }
    }
}

impl<'self> Module<'self> {
    pub fn get_data_layout(&self) -> ~str {
        unsafe {
            let buf = module::LLVMGetDataLayout(self.r);
            str::raw::from_c_str(buf)
        }
    }

    pub fn set_data_layout(&mut self, triple: &str) {
        unsafe {
            do str::as_c_str(triple) |s| {
                module::LLVMSetDataLayout(self.r, s);
            }
        }
    }

    pub fn get_target(&self) -> ~str {
        unsafe {
            let buf = module::LLVMGetTarget(self.r);
            str::raw::from_c_str(buf)
        }
    }

    pub fn set_target(&mut self, triple: &str) {
        unsafe {
            do str::as_c_str(triple) |s| {
                module::LLVMSetTarget(self.r, s);
            }
        }
    }

    pub fn dump(&self) {
        unsafe { module::LLVMDumpModule(self.r); }
    }

    pub fn print(&self, filename: &str) -> Result<(),~str> {
        use std::libc::c_char;
        unsafe {
            do str::as_c_str(filename) |s| {
                let mut raw_msg : *c_char = ptr::null();
                let res = module::LLVMPrintModuleToFile(self.r, s, &mut raw_msg);
                if res == True {
                    Ok(())
                } else {
                    let err = Err(str::raw::from_c_str(raw_msg));
                    LLVMDisposeMessage(raw_msg);
                    err
                }
            }
        }
    }

    pub fn set_inline_asm(&mut self, asm: &str) {
        unsafe {
            do str::as_c_str(asm) |s| {
                module::LLVMSetModuleInlineAsm(self.r, s);
            }
        }
    }

    pub fn get_type(&self, name: &str) -> ty::Type {
        unsafe {
            do str::as_c_str(name) |s| {
                let TR = module::LLVMGetTypeByName(self.r, s);
                Wrapper::from_ref(TR)
            }
        }
    }

    pub fn get_named_md_operands(&self, name: &str) -> ~[value::Metadata] {
        unsafe {
            do str::as_c_str(name) |s| {
                let num_ops = module::LLVMGetNamedMetadataNumOperands(self.r, s) as uint;
                let mut buf : ~[ValueRef] = vec::with_capacity(num_ops);
                module::LLVMGetNamedMetadataOperands(self.r, s, vec::raw::to_mut_ptr(buf));
                do buf.map |&VR| {
                    let t : value::Metadata = Wrapper::from_ref(VR);
                    t
                }
            }
        }
    }

    pub fn add_named_md_operand(&mut self, name: &str, val: value::Metadata) {
        unsafe {
            do str::as_c_str(name) |s| {
                module::LLVMAddNamedMetadataOperand(self.r, s, val.to_ref());
            }
        }
    }

    pub fn add_function(&mut self, name: &str, fty: ty::Function) -> value::Function {
        unsafe {
            do str::as_c_str(name) |s| {
                let r = module::LLVMAddFunction(self.r, s, fty.to_ref());
                Wrapper::from_ref(r)
            }
        }
    }

    pub fn each_function(&self, f:&fn(value::Function) -> bool) -> bool {
        unsafe {
            let mut fr = module::LLVMGetFirstFunction(self.r);
            loop {
                if fr.is_null() { return true }

                if !f(Wrapper::from_ref(fr)) {
                    return false;
                }

                fr = module::LLVMGetNextFunction(fr);
            }
        }
    }
}

#[unsafe_destructor]
impl<'self> Drop for Module<'self> {
    fn finalize(&self) {
        unsafe {
            module::LLVMDisposeModule(self.r);
        }
    }
}
