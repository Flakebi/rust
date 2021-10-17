use crate::spec::{LinkerFlavor, LldFlavor, PanicStrategy, TargetOptions};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "unknown".to_string(),
        executables: true,
        families: vec!["amdgpu".to_string()],
        linker: Some("rust-lld".to_string()),
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
        max_atomic_width: Some(64),
        panic_strategy: PanicStrategy::Abort,
        position_independent_executables: true,
        vendor: "amd".to_string(),
        //singlethread: true,
        ..Default::default()
    }
}
