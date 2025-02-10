#![feature(no_core, lang_items)]
#![no_core]
#![crate_type = "staticlib"]

#[lang = "sized"]
trait Sized {}

#[no_mangle]
extern "C" fn foo() {}
