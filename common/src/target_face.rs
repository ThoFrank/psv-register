use crate::class::Class;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, PartialOrd)]
pub enum TargetFace {
    M18cm80,
    M18cm60,
    M18cm40,
    M18Spot,
}

impl TargetFace {
    pub fn for_cls(cls: Class) -> &'static [TargetFace] {
        use Class::*;
        use TargetFace::*;
        match cls {
            RU11M | RU11W | RU13M | RU13W => &[M18cm80],
            RU15M | RU15W | CU15 | BU15 => &[M18cm60],
            BUE20M | BUE20W | BU21 => &[M18cm40],
            CUE20M | CUE20W | CUE49M | CUE49W | CUE65M | CUE65W | CU21 => &[M18Spot],
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
                TargetFace::M18cm80 => "18m / 80cm",
                TargetFace::M18cm60 => "18m / 60cm",
                TargetFace::M18cm40 => "18m / 40cm",
                TargetFace::M18Spot => "18m / Spot",
            }
        )
    }
}
