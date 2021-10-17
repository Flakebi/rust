use super::{InlineAsmArch, InlineAsmType};
use rustc_macros::HashStable_Generic;
use std::fmt;

def_reg_class! {
    Amdgpu AmdgpuInlineAsmRegClass {
        reg,
        sgpr,
        vgpr,
        agpr,
    }
}

impl AmdgpuInlineAsmRegClass {
    pub fn valid_modifiers(self, _arch: super::InlineAsmArch) -> &'static [char] {
        &[]
    }

    pub fn suggest_class(self, _arch: InlineAsmArch, _ty: InlineAsmType) -> Option<Self> {
        None
    }

    pub fn suggest_modifier(
        self,
        _arch: InlineAsmArch,
        _ty: InlineAsmType,
    ) -> Option<(char, &'static str)> {
        None
    }

    pub fn default_modifier(self, _arch: InlineAsmArch) -> Option<(char, &'static str)> {
        None
    }

    pub fn supported_types(
        self,
        _arch: InlineAsmArch,
    ) -> &'static [(InlineAsmType, Option<&'static str>)] {
        match self {
            Self::reg => types! { _: I8, I16, I32, I64, F32, F64; },
            Self::sgpr => types! { _: I8, I16, I32, F32; },
            Self::vgpr => types! { _: I8, I16, I32, F32; },
            Self::agpr => types! { _: I8, I16, I32, F32; },
        }
    }
}

def_regs! {
    Amdgpu AmdgpuInlineAsmReg AmdgpuInlineAsmRegClass {
        s0: reg, sgpr = ["s0"],
        s1: reg, sgpr = ["s1"],
        s2: reg, sgpr = ["s2"],
        s3: reg, sgpr = ["s3"],
        s4: reg, sgpr = ["s4"],
        s5: reg, sgpr = ["s5"],
        s6: reg, sgpr = ["s6"],
        s7: reg, sgpr = ["s7"],
        s8: reg, sgpr = ["s8"],
        s9: reg, sgpr = ["s9"],
        v0: reg, vgpr = ["v0"],
        v1: reg, vgpr = ["v1"],
        v2: reg, vgpr = ["v2"],
        v3: reg, vgpr = ["v3"],
        v4: reg, vgpr = ["v4"],
        v5: reg, vgpr = ["v5"],
        v6: reg, vgpr = ["v6"],
        v7: reg, vgpr = ["v7"],
        v8: reg, vgpr = ["v8"],
        v9: reg, vgpr = ["v9"],
        a0: reg, agpr = ["a0"],
        a1: reg, agpr = ["a1"],
        a2: reg, agpr = ["a2"],
        a3: reg, agpr = ["a3"],
        a4: reg, agpr = ["a4"],
        a5: reg, agpr = ["a5"],
        a6: reg, agpr = ["a6"],
        a7: reg, agpr = ["a7"],
        a8: reg, agpr = ["a8"],
        a9: reg, agpr = ["a9"],
    }
}

impl AmdgpuInlineAsmReg {
    pub fn emit(
        self,
        out: &mut dyn fmt::Write,
        _arch: InlineAsmArch,
        _modifier: Option<char>,
    ) -> fmt::Result {
        out.write_str(self.name())
    }
}
