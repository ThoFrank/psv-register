use crate::class::Class;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetFace {
    M70cm122,
    M60cm122,
    M50cm122,
    M40cm122,
    M40cm80,
    M25cm80,
    M18cm80,
    M50Spot10_5,
}

impl TargetFace {
    pub fn for_cls(cls: Class) -> &'static [TargetFace] {
        use Class::*;
        use TargetFace::*;
        match cls {
            R10 | R11 | R40 | R41 => &[M70cm122],
            R12 | R13 | R30 | R31 => &[M60cm122],
            R14 | R15 => &[M50cm122],
            R20 | R21 => &[M40cm122],
            R22 | R23 => &[M25cm80],
            R24 | R25 => &[M18cm80],
            C10 | C11 | C12 | C13 | C14 | C15 | C30 | C40 => &[M50Spot10_5],
            C20 => &[M40cm80],
            B10 | B11 | B12 | B13 | B30 => &[M40cm122],
            B20 => &[M25cm80],
            OO => &[M25cm80],
        }
    }
}

impl std::fmt::Display for TargetFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TargetFace::M70cm122 => "70m/122cm",
                TargetFace::M60cm122 => "60m / 122cm",
                TargetFace::M50cm122 => "50m / 122cm",
                TargetFace::M40cm122 => "40m / 122cm",
                TargetFace::M40cm80 => "40m / 80cm",
                TargetFace::M25cm80 => "25m / 80cm",
                TargetFace::M18cm80 => "18m / 80cm",
                TargetFace::M50Spot10_5 => "50m / Spot(10-5)",
            }
        )
    }
}
