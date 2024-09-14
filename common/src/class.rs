use std::{fmt::Display, str::FromStr};

use crate::{bow_type::BowType, locale::Locale};
use chrono::{Months, NaiveDate};
use itertools::Itertools;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use strum::{Display, IntoEnumIterator};

lazy_static! {
    pub static ref SEASON_START: NaiveDate = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    strum::EnumIter,
    PartialOrd,
    Hash,
    Display,
)]
pub enum Class {
    RUE20M,
    RUE20W,
    RU15M,
    RU15W,
    RU13M,
    RU13W,
    RU11M,
    RU11W,
    RU18M,
    RU18W,
    RU21M,
    RU21W,
    RUE49M,
    RUE49W,
    RUE65M,
    RUE65W,
    BUE20M,
    BUE20W,
    BU15,
    BU21,
    CUE20M,
    CUE20W,
    CU15,
    CU21,
    CUE49M,
    CUE49W,
    CUE65M,
    CUE65W,
}

impl Class {
    pub fn name(&self, locale: Locale) -> &'static str {
        match locale {
            Locale::En => match self {
                Class::RUE20M => "Recurve Men",
                Class::RUE20W => "Recurve Women",
                Class::RU15M => "Recurve 13-14 male",
                Class::RU15W => "Recurve 13-14 female",
                Class::RU13M => "Recurve 11-12 male",
                Class::RU13W => "Recurve 11-12 female",
                Class::RU11M => "Recurve 1-10 male",
                Class::RU11W => "Recurve 1-10 female",
                Class::RU18M => "Recurve 15-17 male",
                Class::RU18W => "Recurve 15-17 female",
                Class::RU21M => "Recurve 18-20 male",
                Class::RU21W => "Recurve 18-20 female",
                Class::RUE49M => "Recurve 50-65 male",
                Class::RUE49W => "Recurve 50-65 female",
                Class::RUE65M => "Recurve 66+ male",
                Class::RUE65W => "Recurve 66+ female",
                Class::BUE20M => "Barebow Men",
                Class::BUE20W => "Barebow Women",
                Class::BU15 => "Barebow 1-14 male/female",
                Class::BU21 => "Barebow 15-20 male/female",
                Class::CUE20M => "Compound Men",
                Class::CUE20W => "Compound Women",
                Class::CU15 => "Compound 1-14 male/female",
                Class::CU21 => "Compound 15-20 male/female",
                Class::CUE49M => "Compound 50-65 male",
                Class::CUE49W => "Compound 50-65 female",
                Class::CUE65M => "Compound 66+ male",
                Class::CUE65W => "Compound 66+ female",
            },
            Locale::De => match self {
                Class::RUE20M => "Recurve Herren",
                Class::RUE20W => "Recurve Damen",
                Class::RU15M => "Recurve Schüler A m",
                Class::RU15W => "Recurve Schüler A w",
                Class::RU13M => "Recurve Schüler B m",
                Class::RU13W => "Recurve Schüler B w",
                Class::RU11M => "Recurve Schüler C m",
                Class::RU11W => "Recurve Schüler C w",
                Class::RU18M => "Recurve Jugend m",
                Class::RU18W => "Recurve Jugend w",
                Class::RU21M => "Recurve Junioren m",
                Class::RU21W => "Recurve Junioren w",
                Class::RUE49M => "Recurve Master m",
                Class::RUE49W => "Recurve Master w",
                Class::RUE65M => "Recurve Senioren m",
                Class::RUE65W => "Recurve Senioren w",
                Class::BUE20M => "Blank Herren",
                Class::BUE20W => "Blank Damen",
                Class::BU15 => "Blank Schüler m/w",
                Class::BU21 => "Blank Jugend/Junioren m/w",
                Class::CUE20M => "Compound Herren",
                Class::CUE20W => "Compound Damen",
                Class::CU15 => "Compound Schüler m/w",
                Class::CU21 => "Compound Jugend/Junioren m/w",
                Class::CUE49M => "Compound Master m",
                Class::CUE49W => "Compound Master w",
                Class::CUE65M => "Compound Senioren m",
                Class::CUE65W => "Compound Senioren w",
            },
        }
    }
    pub fn comment(&self) -> &'static str {
        ""
    }
    pub fn all_classes() -> impl Iterator<Item = Self> {
        Self::iter()
    }
    pub fn recurve_classes() -> &'static [Self] {
        &[
            Self::RUE20M,
            Self::RUE20W,
            Self::RU15M,
            Self::RU15W,
            Self::RU13M,
            Self::RU13W,
            Self::RU11M,
            Self::RU11W,
            Self::RU18M,
            Self::RU18W,
            Self::RU21M,
            Self::RU21W,
            Self::RUE49M,
            Self::RUE49W,
            Self::RUE65M,
            Self::RUE65W,
        ]
    }
    pub fn barebow_classes() -> &'static [Self] {
        &[Self::BUE20M, Self::BUE20W, Self::BU15, Self::BU21]
    }
    pub fn compound_classes() -> &'static [Self] {
        &[
            Self::CUE20M,
            Self::CUE20W,
            Self::CU15,
            Self::CU21,
            Self::CUE49M,
            Self::CUE49W,
            Self::CUE65M,
            Self::CUE65W,
        ]
    }
    pub fn in_range(&self, dob: NaiveDate) -> bool {
        let year_range = match self {
            Class::RUE20M => (21, 49),
            Class::RUE20W => (21, 49),
            Class::RU15M => (13, 14),
            Class::RU15W => (13, 14),
            Class::RU13M => (11, 12),
            Class::RU13W => (11, 12),
            Class::RU11M => (1, 10),
            Class::RU11W => (1, 10),
            Class::RU18M => (15, 17),
            Class::RU18W => (15, 17),
            Class::RU21M => (18, 20),
            Class::RU21W => (18, 20),
            Class::RUE49M => (50, 65),
            Class::RUE49W => (50, 65),
            Class::RUE65M => (66, 120),
            Class::RUE65W => (66, 120),

            Class::CUE20M => (21, 49),
            Class::CUE20W => (21, 49),
            Class::CU15 => (1, 14),
            Class::CU21 => (15, 20),
            Class::CUE49M => (50, 65),
            Class::CUE49W => (50, 65),
            Class::CUE65M => (66, 120),
            Class::CUE65W => (66, 120),

            Class::BUE20M => (21, 120),
            Class::BUE20W => (21, 120),
            Class::BU15 => (1, 14),
            Class::BU21 => (15, 20),
        };

        let date_range = (*SEASON_START - Months::new(year_range.1 * 12))
            ..(*SEASON_START - Months::new((year_range.0 - 1) * 12));
        date_range.contains(&dob)
    }

    // Price of starter in class in euro cent
    pub fn price(&self) -> u32 {
        use Class::*;
        match self {
            RU15M | RU15W | RU13M | RU13W | RU11M | RU11W | RU18M | RU18W | BU15 | BU21 | CU15
            | CU21 => 1200,
            _ => 1800,
        }
    }

    pub fn allowed_classes(bow_type: BowType, dob: NaiveDate) -> Vec<(Class, ClassUpgradeStatus)> {
        let default_classes = match bow_type {
            BowType::Recurve => Class::recurve_classes(),
            BowType::Compound => Class::compound_classes(),
            BowType::Barebow => Class::barebow_classes(),
        }
        .iter()
        .filter(|cls| cls.in_range(dob));

        let upgrade_classes = default_classes
            .clone()
            .map(|dc| dc.other_allowed_classes().into_iter())
            .flatten()
            .unique();
        default_classes
            .map(|&c| (c, ClassUpgradeStatus::InDefaultAgeRange))
            .chain(upgrade_classes.map(|&c| (c, ClassUpgradeStatus::Upgrade)))
            .collect()
    }

    fn other_allowed_classes(&self) -> &'static [Self] {
        use Class::*;
        match self {
            // Junioren + Master => Herren/Damen
            RU21M | RUE49M => &[RUE20M],
            RU21W | RUE49W => &[RUE20W],
            RUE65M => &[RUE49M, RUE20M],
            RUE65W => &[RUE49W, RUE20W],

            BU21 => &[BUE20M, BUE20W],

            CU21 => &[CUE20M, CUE20W],
            CUE49M => &[CUE20M],
            CUE49W => &[CUE20W],
            CUE65M => &[CUE49M, CUE20M],
            CUE65W => &[CUE49W, CUE20W],

            _ => &[],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClassUpgradeStatus {
    InDefaultAgeRange,
    Upgrade,
}

impl FromStr for Class {
    type Err = UnknownClassError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::iter()
            .find(|c| format!("{c:?}") == s)
            .ok_or(UnknownClassError { class: s.into() })
    }
}

#[derive(Debug)]
pub struct UnknownClassError {
    pub class: String,
}

impl std::error::Error for UnknownClassError {}
impl Display for UnknownClassError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown class: {}", self.class)
    }
}

#[test]
fn test_in_range() {
    use chrono::Datelike;
    let now_ = SEASON_START.year() - 1;
    let died = SEASON_START.year() - 120;
    let test_cases = std::collections::HashMap::from([
        (Class::RUE20M, (1976, 2004)),
        (Class::RUE20W, (1976, 2004)),
        (Class::RU15M, (2011, 2012)),
        (Class::RU15W, (2011, 2012)),
        (Class::RU13M, (2013, 2014)),
        (Class::RU13W, (2013, 2014)),
        (Class::RU11M, (2015, now_)),
        (Class::RU11W, (2015, now_)),
        (Class::RU18M, (2008, 2010)),
        (Class::RU18W, (2008, 2010)),
        (Class::RU21M, (2005, 2007)),
        (Class::RU21W, (2005, 2007)),
        (Class::RUE49M, (1960, 1975)),
        (Class::RUE49W, (1960, 1975)),
        (Class::RUE65M, (died, 1959)),
        (Class::RUE65W, (died, 1959)),
        //
        (Class::BUE20M, (died, 2004)),
        (Class::BUE20W, (died, 2004)),
        (Class::BU15, (2011, now_)),
        (Class::BU21, (2005, 2010)),
        //
        (Class::CUE20M, (1976, 2004)),
        (Class::CUE20W, (1976, 2004)),
        (Class::CU15, (2011, now_)),
        (Class::CU21, (2005, 2010)),
        (Class::CUE49M, (1960, 1975)),
        (Class::CUE49W, (1960, 1975)),
        (Class::CUE65M, (died, 1959)),
        (Class::CUE65W, (died, 1959)),
    ]);

    for cls in Class::iter() {
        let range = test_cases[&cls];
        let lower_out = NaiveDate::from_ymd_opt(range.0 - 1, 12, 31).unwrap();
        let lower_in = NaiveDate::from_ymd_opt(range.0, 1, 1).unwrap();
        let upper_in = NaiveDate::from_ymd_opt(range.1, 12, 31).unwrap();
        let upper_out = NaiveDate::from_ymd_opt(range.1 + 1, 1, 1).unwrap();

        println!("{}", lower_out);
        println!("{}", lower_in);
        println!("{}", upper_in);
        println!("{}", upper_out);
        println!();

        assert!(
            !cls.in_range(lower_out),
            "Lower out bound not respected for {}: {}",
            cls,
            lower_out
        );
        assert!(
            cls.in_range(lower_in),
            "Lower in bound not respected for {}: {}",
            cls,
            lower_in
        );
        assert!(
            cls.in_range(upper_in),
            "Upper in bound not respected for {}: {}",
            cls,
            upper_in
        );
        assert!(
            !cls.in_range(upper_out),
            "Upper out bound not respected for {}: {}",
            cls,
            upper_out
        );
    }
}
