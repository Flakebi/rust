// Check that lto with -C linker-plugin-lto actually works and can inline functions.
// A static library is compiled, defining a function. Then a dylib is compiled,
// linking to the static library and calling the function from the library.
// The function from the library should end up inlined and disappear from the output.

//@ needs-llvm-components: amdgpu
//@ needs-rust-lld

#![feature(path_file_prefix)]

use run_make_support::{dynamic_lib_name, llvm_readobj, rustc, static_lib_name};

fn main() {
    rustc()
        .input("lib.rs")
        .output(static_lib_name("lib"))
        .args(&["-Clinker-plugin-lto", "--target=amdgcn-amd-amdhsa", "-Ctarget-cpu=gfx900"])
        .run();
    rustc()
        .input("main.rs")
        .output("main.elf")
        .opt_level("3")
        .args(&[
            "-Clinker-plugin-lto",
            "--target=amdgcn-amd-amdhsa",
            "-Ctarget-cpu=gfx900",
            &format!("-Clink-arg={}", static_lib_name("lib")),
        ])
        .run();

    llvm_readobj()
        .input("main.elf")
        .symbols()
        .run()
        // The linked function should disappear.
        .assert_stdout_not_contains("foo")
        // The function from main.rs should still be there
        .assert_stdout_contains("bar")
        // and the kernel descriptor .kd symbol as well.
        .assert_stdout_contains("bar.kd");
}
