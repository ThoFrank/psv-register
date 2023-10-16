use crate::class::Class;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, PartialOrd)]
pub enum TargetFace {
    M18cm60,
    M18cm40,
    M18Spot,
}

impl TargetFace {
    pub fn for_cls(cls: Class) -> &'static [TargetFace] {
        use Class::*;
        use TargetFace::*;
        match cls {
            R24 | R25 | R22 | R23 => &[M18cm60],
            R20 | R21 | C120 | B220 => &[M18cm60],
            B210 | B211 | B230 | B212 | B213 => &[M18cm40],
            C110 | C111 | C112 | C113 | C130 => &[M18Spot],
            _ => &[M18Spot, M18cm40],
        }
    }
}

impl std::fmt::Display for TargetFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TargetFace::M18cm60 => "18m / 60cm",
                TargetFace::M18cm40 => "18m / 40cm",
                TargetFace::M18Spot => "18m / Spot",
            }
        )
    }
}
