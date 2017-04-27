use llvm_sys::core as llvm;
use llvm_sys::prelude::*;

pub struct PassManager {
    pub ptr: LLVMPassManagerRef,
}

impl PassManager {
    pub fn new() -> PassManager {
        PassManager {
            ptr: unsafe { llvm::LLVMCreatePassManager() }
        }
    }
}
