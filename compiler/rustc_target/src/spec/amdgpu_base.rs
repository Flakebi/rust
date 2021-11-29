use std::collections::BTreeMap;
use std::str::FromStr;

use crate::spec::{
    AddrSpaceIdx, AddrSpaceKind, AddrSpaceProps, AddrSpaces, LinkerFlavor, LldFlavor,
    PanicStrategy, TargetOptions,
};

pub fn opts() -> TargetOptions {
    let flat_idx = AddrSpaceIdx(0);
    let global_idx = AddrSpaceIdx(1);
    let region = AddrSpaceKind::from_str("region").unwrap();
    let region_idx = AddrSpaceIdx(2);
    let local = AddrSpaceKind::from_str("local").unwrap();
    let local_idx = AddrSpaceIdx(3);
    let constant_idx = AddrSpaceIdx(4);
    let private_idx = AddrSpaceIdx(5);
    let constant_32b = AddrSpaceKind::from_str("constant_32b").unwrap();
    let constant_32b_idx = AddrSpaceIdx(6);

    let mut addr_spaces = AddrSpaces::default();
    addr_spaces.insert(
        AddrSpaceKind::Flat,
        AddrSpaceProps {
            index: flat_idx,
            shared_with: [
                AddrSpaceKind::Alloca,
                region.clone(),
                local.clone(),
                AddrSpaceKind::ReadOnly,
                AddrSpaceKind::ReadWrite,
                constant_32b.clone(),
            ]
            .into(),
        },
    );

    let insert_as = |addr_spaces: &mut BTreeMap<_, _>, kind, idx| {
        let props = AddrSpaceProps {
            index: idx,
            shared_with: vec![AddrSpaceKind::Flat].into_iter().collect(),
        };
        addr_spaces.insert(kind, props);
    };
    insert_as(&mut addr_spaces, AddrSpaceKind::ReadWrite, global_idx);
    insert_as(&mut addr_spaces, region.clone(), region_idx);
    insert_as(&mut addr_spaces, local.clone(), local_idx);
    insert_as(&mut addr_spaces, AddrSpaceKind::ReadOnly, constant_idx);
    insert_as(&mut addr_spaces, AddrSpaceKind::Alloca, private_idx);
    insert_as(&mut addr_spaces, constant_32b.clone(), constant_32b_idx);

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
        addr_spaces,
        //singlethread: true,
        ..Default::default()
    }
}
