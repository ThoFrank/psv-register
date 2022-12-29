use crate::bow_type::BowType;
use chrono::{Months, NaiveDate};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SEASON_START: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    R10,
    R11,
    R20,
    R21,
    R22,
    R23,
    R30,
    R31,
    R40,
    R41,
    R12,
    R13,
    R14,
    R15,
    B10,
    B11,
    B20,
    B30,
    B12,
    C10,
    C11,
    C20,
    C30,
    C40,
    C12,
    C13,
    C14,
    OO,
}

impl Class {
    pub fn name(&self) -> &'static str {
        match self {
            Class::R10 => "Recurve Herren",
            Class::R11 => "Recurve Damen",
            Class::R20 => "Recurve Schüler A m",
            Class::R21 => "Recurve Schüler A w",
            Class::R22 => "Recurve Schüler B m",
            Class::R23 => "Recurve Schüler B w",
            Class::R30 => "Recurve Jugend m",
            Class::R31 => "Recurve Jugend w",
            Class::R40 => "Recurve Junioren m",
            Class::R41 => "Recurve Junioren w",
            Class::R12 => "Recurve Master m",
            Class::R13 => "Recurve Master w",
            Class::R14 => "Recurve Senioren m",
            Class::R15 => "Recurve Senioren w",
            Class::B10 => "Blank Herren",
            Class::B11 => "Blank Damen",
            Class::B20 => "Blank Schüler m/w",
            Class::B30 => "Blank Jugend m/m",
            Class::B12 => "Blank Master m",
            Class::C10 => "Compound Herren",
            Class::C11 => "Compound Damen",
            Class::C20 => "Compound Schüler m/w",
            Class::C30 => "Compound Jugend m/m",
            Class::C40 => "Compound Junioren m/w",
            Class::C12 => "Compound Master m",
            Class::C13 => "Compound Master w",
            Class::C14 => "Compound Senioren m",
            Class::OO => "Offene Klasse",
        }
    }
    pub fn comment(&self) -> &'static str {
        match self{
            Class::OO => "Eine Klasse für alle. Die Auflage ist größer als bei den offizielen Klassen. Dadurch ist eine Qualifikation zur Bezirksmeisterschaft ausgeschlossen.",
            _ => "Reguläre Klasse nach Sportornung. Eine Weitermeldung zur Bezirksmeisterschaft ist möglich"
        }
    }
    pub fn recurve_classes() -> &'static [Self] {
        &[
            Self::R10,
            Self::R11,
            Self::R20,
            Self::R21,
            Self::R22,
            Self::R23,
            Self::R30,
            Self::R31,
            Self::R40,
            Self::R41,
            Self::R12,
            Self::R13,
            Self::R14,
            Self::R15,
            Self::OO,
        ]
    }
    pub fn barebow_classes() -> &'static [Self] {
        &[
            Self::B10,
            Self::B11,
            Self::B20,
            Self::B30,
            Self::B12,
            Self::OO,
        ]
    }
    pub fn compound_classes() -> &'static [Self] {
        &[
            Self::C10,
            Self::C11,
            Self::C20,
            Self::C30,
            Self::C40,
            Self::C12,
            Self::C13,
            Self::C14,
            Self::OO,
        ]
    }
    pub fn in_range(&self, dob: NaiveDate) -> bool {
        let year_range = match self {
            Class::R10 => (21, 49),
            Class::R11 => (21, 49),
            Class::R20 => (13, 14),
            Class::R21 => (13, 14),
            Class::R22 => (11, 12),
            Class::R23 => (11, 12),
            Class::R30 => (15, 17),
            Class::R31 => (15, 17),
            Class::R40 => (18, 20),
            Class::R41 => (18, 20),
            Class::R12 => (50, 65),
            Class::R13 => (50, 65),
            Class::R14 => (66, 120),
            Class::R15 => (66, 120),
            Class::C10 => (21, 49),
            Class::C11 => (21, 49),
            Class::C20 => (1, 14),
            Class::C30 => (15, 17),
            Class::C40 => (18, 20),
            Class::C12 => (50, 65),
            Class::C13 => (50, 120),
            Class::C14 => (66, 120),
            Class::B10 => (21, 49),
            Class::B11 => (21, 120),
            Class::B20 => (1, 14),
            Class::B30 => (15, 20),
            Class::B12 => (50, 120),
            Class::OO => (15, 120),
        };

        let date_range = (*SEASON_START - Months::new(year_range.1 * 12))
            ..(*SEASON_START - Months::new((year_range.0 - 1) * 12));
        date_range.contains(&dob)
    }
    pub fn classes_for(dob: NaiveDate, bow_type: BowType) -> Vec<Class> {
        match bow_type {
            BowType::Recurve => Self::recurve_classes(),
            BowType::Compound => Self::compound_classes(),
            BowType::Barebow => Self::barebow_classes(),
        }
        .iter()
        .filter(|c| c.in_range(dob))
        .copied()
        .collect()
    }
}

#[test]
fn test_in_range() {
    assert!(!Class::R10.in_range(NaiveDate::from_ymd_opt(1973, 12, 31).unwrap()));
    assert!(Class::R10.in_range(NaiveDate::from_ymd_opt(1974, 1, 1).unwrap()));
    assert!(Class::R10.in_range(NaiveDate::from_ymd_opt(2002, 12, 31).unwrap()));
    assert!(!Class::R10.in_range(NaiveDate::from_ymd_opt(2003, 1, 1).unwrap()));
}
