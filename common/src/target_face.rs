use crate::class::Class;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetFace {
    Spot,
    Cm40,
    Cm60,
    Cm80,
    Cm122,
}

impl TargetFace {
    pub fn for_cls(cls: Class) -> &'static [TargetFace] {
        use Class::*;
        use TargetFace::*;
        match cls {
            C10 | C11 | C30 | C40 | C12 | C13 | C14 => &[Spot],
            R10 | R11 | R40 | R41 | R12 | R13 => &[Spot, Cm40],
            R30 | R31 | R14 | R15 | B10 | B11 | B12 | B30 => &[Cm40],
            R20 | R21 | B20 | C20 | OO => &[Cm60],
            R22 | R23 => &[Cm80],
        }
    }
}

impl std::fmt::Display for TargetFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TargetFace::Spot => "Spot",
                TargetFace::Cm40 => "40cm",
                TargetFace::Cm60 => "60cm",
                TargetFace::Cm80 => "80cm",
                TargetFace::Cm122 => "122cm",
            }
        )
    }
}
