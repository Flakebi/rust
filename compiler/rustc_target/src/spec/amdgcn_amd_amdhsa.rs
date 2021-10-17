//! An amdgpu hsa target.
//!
//! The `amdgpu-unknown-amdhsa` target is intended for gpgpu applications.
//! The standard library is not available.
//!
//! The hardware generation needs to be explicitly specified, like `-C target-cpu=gfx1010`.

use crate::spec::Target;

pub fn target() -> Target {
    let mut options = super::amdgpu_base::opts();
    options.os = "amdhsa".to_string();

    Target {
        arch: "amdgpu".to_string(),
        data_layout: "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-\
            p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-\
            v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
            .to_string(),
        llvm_target: "amdgcn-amd-amdhsa".to_string(),
        pointer_width: 64,
        options,
    }
}
