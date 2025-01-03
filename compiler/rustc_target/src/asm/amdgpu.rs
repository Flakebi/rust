use std::fmt;

use rustc_span::Symbol;

use super::{InlineAsmArch, InlineAsmType, ModifierInfo};

def_reg_class! {
    Amdgpu AmdgpuInlineAsmRegClass {
        reg,
        sgpr,
        vgpr,
        agpr,
    }
}

impl AmdgpuInlineAsmRegClass {
    pub fn valid_modifiers(self, _arch: InlineAsmArch) -> &'static [char] {
        &[]
    }

    pub fn suggest_class(self, _arch: InlineAsmArch, _ty: InlineAsmType) -> Option<Self> {
        None
    }

    pub fn suggest_modifier(
        self,
        _arch: InlineAsmArch,
        _ty: InlineAsmType,
    ) -> Option<ModifierInfo> {
        None
    }

    pub fn default_modifier(self, _arch: InlineAsmArch) -> Option<ModifierInfo> {
        None
    }

    pub fn supported_types(
        self,
        _arch: InlineAsmArch,
    ) -> &'static [(InlineAsmType, Option<Symbol>)] {
        // The LLVM backend supports types up to 1024 bits
        types! { _: I16, F16, I32, I64, F32, F64, I128, F128; }
    }

    /// The number of supported registers in this class.
    /// The returned number is the length, so supported register
    /// indices are 0 to max_num()-1.
    fn max_num(self) -> u32 {
        match self {
            Self::reg => 256,
            Self::sgpr => 106,
            Self::vgpr => 256,
            Self::agpr => 256,
        }
    }

    /// Prefix when printed and register constraint in LLVM.
    pub fn prefix(self) -> &'static str {
        match self {
            Self::reg => "r",
            Self::sgpr => "s",
            Self::vgpr => "v",
            Self::agpr => "a",
        }
    }

    /// Get register class from prefix.
    fn parse_prefix(prefix: char) -> Result<Self, &'static str> {
        match prefix {
            's' => Ok(Self::sgpr),
            'v' => Ok(Self::vgpr),
            'a' => Ok(Self::agpr),
            _ => Err("unknown register prefix"),
        }
    }
}

#[derive(
    Copy,
    Clone,
    rustc_macros::Encodable,
    rustc_macros::Decodable,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Hash,
    rustc_macros::HashStable_Generic
)]
#[allow(non_camel_case_types)]
pub struct AmdgpuInlineAsmReg {
    class: AmdgpuInlineAsmRegClass,
    // Register range, end is inclusive
    start: u32,
    end: u32,
}

impl AmdgpuInlineAsmReg {
    pub fn name(self) -> String {
        let c = self.class.prefix();
        if self.start == self.end {
            format!("{c}{}", self.start)
        } else {
            format!("{c}[{}:{}]", self.start, self.end)
        }
    }

    pub fn reg_class(self) -> AmdgpuInlineAsmRegClass {
        self.class
    }

    pub fn parse(name: &str) -> Result<Self, &'static str> {
        if name.is_empty() {
            return Err("invalid empty register");
        }
        let class = AmdgpuInlineAsmRegClass::parse_prefix(name.chars().next().unwrap())?;
        // Form with range, e.g. s[2:3]
        let res;
        if name[1..].starts_with('[') {
            if !name.ends_with(']') {
                return Err("invalid register, missing closing bracket");
            }
            if let Some((start, end)) = name[2..name.len() - 1].split_once(':') {
                let Ok(start) = start.parse() else {
                    return Err("invalid register range start");
                };
                let Ok(end) = end.parse() else {
                    return Err("invalid register range end");
                };
                res = Self { class, start, end };
            } else {
                return Err("invalid register range");
            }
        } else {
            let Ok(start) = name[1..].parse() else {
                return Err("invalid register number");
            };
            res = Self { class, start, end: start };
        }

        // Check range
        if res.start > res.end {
            return Err("invalid reversed register range");
        }

        if res.end >= res.class.max_num() {
            return Err("too large register for this class");
        }
        Ok(res)
    }

    pub fn validate(
        self,
        _arch: super::InlineAsmArch,
        _reloc_model: crate::spec::RelocModel,
        _target_features: &rustc_data_structures::fx::FxIndexSet<Symbol>,
        _target: &crate::spec::Target,
        _is_clobber: bool,
    ) -> Result<(), &'static str> {
        Ok(())
    }
}

pub(super) fn fill_reg_map(
    _arch: super::InlineAsmArch,
    _reloc_model: crate::spec::RelocModel,
    _target_features: &rustc_data_structures::fx::FxIndexSet<Symbol>,
    _target: &crate::spec::Target,
    map: &mut rustc_data_structures::fx::FxHashMap<
        super::InlineAsmRegClass,
        rustc_data_structures::fx::FxIndexSet<super::InlineAsmReg>,
    >,
) {
    use super::{InlineAsmReg, InlineAsmRegClass};

    // Add single registers of each class (no register ranges)
    #[allow(rustc::potential_query_instability)]
    for class in regclass_map().keys() {
        let InlineAsmRegClass::Amdgpu(class) = *class else { unreachable!("Must be amdgpu class") };
        if let Some(set) = map.get_mut(&InlineAsmRegClass::Amdgpu(class)) {
            for i in 0..class.max_num() {
                set.insert(InlineAsmReg::Amdgpu(AmdgpuInlineAsmReg { class, start: i, end: i }));
            }
        }
    }
}

impl AmdgpuInlineAsmReg {
    pub fn emit(
        self,
        out: &mut dyn fmt::Write,
        _arch: InlineAsmArch,
        _modifier: Option<char>,
    ) -> fmt::Result {
        out.write_str(&self.name())
    }

    // FIXME If ranges are taken into account, there are too many conflicts to list.
    pub fn overlapping_regs(self, mut _cb: impl FnMut(AmdgpuInlineAsmReg)) {}
}
