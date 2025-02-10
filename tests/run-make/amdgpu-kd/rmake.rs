// Checks that the produced object has the .kd symbol exported.

//@ needs-llvm-components: amdgpu
//@ needs-rust-lld

use run_make_support::{cmd, dynamic_lib_name, llvm_readobj, run_in_tmpdir, rustc};

fn main() {
    rustc()
        .crate_name("foo")
        .target("amdgcn-amd-amdhsa")
        .arg("-Ctarget-cpu=gfx900")
        .crate_type("cdylib")
        .input("foo.rs")
        .run();
    llvm_readobj().input("foo.elf").symbols().run().assert_stdout_contains("kernel.kd");
}
