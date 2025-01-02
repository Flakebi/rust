# `amdgcn-amd-amdhsa`

**Tier: 3**

AMD GPU target for compute/HSA (Heterogeneous System Architecture).

## Target maintainers

- [@Flakebi](https://github.com/Flakebi)

## Requirements

AMD GPUs can be targeted via cross-compilation.
The default binary format is ELF.
Binaries can be loaded by the HSA runtime implemented in [ROCR-Runtime].

Binaries must be built with no-std.
They can use `core` and `alloc` (`alloc` only if an allocator is supplied).
At least one function needs to use the `"amdgpu-kernel"` calling convention.
These functions can be used as kernel entrypoints in HSA.

## Building the target

The target is included in rustc.

## Building Rust programs

The amdgpu target supports many hardware generations, which need different binaries.
The generations are all exposed as different target-cpus in the backend.
As there are many, Rust does not ship pre-compiled libraries for this target.
Therefore, you have to build your own copy of `core` by using `cargo -Zbuild-std=core` or similar.

To build a binary that can be loaded with HSA, create a no-std library:
```rust
// src/lib.rs
#![feature(abi_amdgpu_kernel)]
#![no_std]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "amdgpu-kernel" fn kernel(/* Arguments */) {
    // Code
}
```

Build the library as `cdylib`:
```toml
# Cargo.toml
[lib]
crate-type = ["cdylib"]

[profile.dev]
lto = true # LTO must be explicitly enabled for now
[profile.release]
lto = true
```

The target-cpu must be from the list [supported by LLVM] (or printed with `rustc --target amdgcn-amd-amdhsa --print target-cpus`).
The GPU version on the current system can be found e.g. with [`rocminfo`].

Example `.cargo/config.toml` file to set the target and GPU generation:
```toml
# .cargo/config.toml
[build]
target = "amdgcn-amd-amdhsa"
rustflags = ["-Ctarget-cpu=gfx1100"]

[unstable]
build-std = ["core"] # Optional: "alloc"
```

<!-- Mention an allocator once a suitable one exists for amdgpu -->

<!-- Mention HSA bindings once they work

## Testing

Does the target support running binaries, or do binaries have varying
expectations that prevent having a standard way to run them? If users can run
binaries, can they do so in some common emulator, or do they need native
hardware? Does the target support running the Rust testsuite?

-->

## Additional information

More information can be found on the [LLVM page for amdgpu]

[ROCR-Runtime]: https://github.com/ROCm/ROCR-Runtime
[supported by LLVM]: https://llvm.org/docs/AMDGPUUsage.html#processors
[LLVM page for amdgpu]: https://llvm.org/docs/AMDGPUUsage.html
[`rocminfo`]: https://github.com/ROCm/rocminfo
