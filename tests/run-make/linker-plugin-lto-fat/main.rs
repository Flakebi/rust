#![feature(no_core, abi_gpu_kernel, lang_items)]
#![no_core]
#![crate_type = "cdylib"]

#[lang = "sized"]
trait Sized {}

extern "C" {
    fn foo();
}

#[no_mangle]
extern "gpu-kernel" fn bar() {
    unsafe {
        foo();
    }
}
