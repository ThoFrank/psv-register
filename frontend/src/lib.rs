use email_address::EmailAddress;
use std::fmt::Display;

use chrono::{prelude::*, Months};
use lazy_static::lazy_static;
use seed::{prelude::*, *};

lazy_static! {
    pub static ref SEASON_START: NaiveDate = NaiveDate::from_ymd_opt(2023, 01, 01).unwrap();
}

#[derive(Default)]
struct Model {
    first_name: String,
    last_name: String,
    date_of_birth: NaiveDate,
    mail: InsertedMail,
    bow_type: BowType,
    cls: Class,

    // optional fields
    target_face: TargetFace,
}

impl Model {
    fn check_and_update_cls(&mut self, orders: &mut impl Orders<Msg>) {
        let new_cls: Vec<Class> = match self.bow_type {
            BowType::Recurve => Class::recurve_classes(),
            BowType::Compound => Class::compound_classes(),
            BowType::Barebow => Class::barebow_classes(),
        }
        .into_iter()
        .filter(|cls| cls.in_range(self.date_of_birth))
        .copied()
        .collect();
        let new_cls = if new_cls.contains(&self.cls) {
            self.cls
        } else {
            new_cls.get(0).unwrap_or(&Class::OO).clone()
        };

        self.update_target_face();

        if self.cls.in_range(self.date_of_birth)
            && match self.bow_type {
                BowType::Recurve => Class::recurve_classes(),
                BowType::Compound => Class::compound_classes(),
                BowType::Barebow => Class::barebow_classes(),
            }
            .contains(&self.cls)
        {
            return;
        }

        orders.send_msg(Msg::ClassChanged(new_cls));
        orders.force_render_now();
    }
    fn update_target_face(&mut self) {
        let choosing_classes = [
            Class::R10,
            Class::R11,
            Class::R40,
            Class::R41,
            Class::R12,
            Class::R13,
        ];
        match (choosing_classes.contains(&self.cls), self.target_face) {
            (true, TargetFace::NotApplicaple) => self.target_face = TargetFace::Full,
            (true, _) => (),
            (false, _) => self.target_face = TargetFace::NotApplicaple,
        }
    }
}

enum InsertedMail {
    Invalid(String),
    Valid(String),
}

impl InsertedMail {
    fn is_valid(&self) -> bool {
        matches!(self, Self::Valid(_))
    }
}

impl Default for InsertedMail {
    fn default() -> Self {
        Self::Invalid(String::new())
    }
}
impl Display for InsertedMail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InsertedMail::Invalid(s) => s.fmt(f),
            InsertedMail::Valid(s) => s.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum BowType {
    Recurve,
    Compound,
    Barebow,
}

impl BowType {
    fn is_recurve(&self) -> bool {
        matches!(self, Self::Recurve)
    }
    fn is_compound(&self) -> bool {
        matches!(self, Self::Compound)
    }
    fn is_barebow(&self) -> bool {
        matches!(self, Self::Barebow)
    }
}

impl Default for BowType {
    fn default() -> Self {
        Self::Recurve
    }
}

#[derive(Default, Clone, Copy, Debug)]
enum TargetFace {
    #[default]
    NotApplicaple,
    Full,
    Spot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Class {
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
    B21,
    B22,
    B23,
    B30,
    B31,
    B40,
    B41,
    B12,
    B13,
    B14,
    B15,
    C10,
    C11,
    C20,
    C21,
    C22,
    C23,
    C30,
    C31,
    C40,
    C41,
    C12,
    C13,
    C14,
    C15,
    OO,
}

impl Class {
    fn name(&self) -> &'static str {
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
            Class::B20 => "Blank Schüler A m",
            Class::B21 => "Blank Schüler A w",
            Class::B22 => "Blank Schüler B m",
            Class::B23 => "Blank Schüler B w",
            Class::B30 => "Blank Jugend m",
            Class::B31 => "Blank Jugend w",
            Class::B40 => "Blank Junioren m",
            Class::B41 => "Blank Junioren w",
            Class::B12 => "Blank Master m",
            Class::B13 => "Blank Master w",
            Class::B14 => "Blank Senioren m",
            Class::B15 => "Blank Senioren w",
            Class::C10 => "Compound Herren",
            Class::C11 => "Compound Damen",
            Class::C20 => "Compound Schüler A m",
            Class::C21 => "Compound Schüler A w",
            Class::C22 => "Compound Schüler B m",
            Class::C23 => "Compound Schüler B w",
            Class::C30 => "Compound Jugend m",
            Class::C31 => "Compound Jugend w",
            Class::C40 => "Compound Junioren m",
            Class::C41 => "Compound Junioren w",
            Class::C12 => "Compound Master m",
            Class::C13 => "Compound Master w",
            Class::C14 => "Compound Senioren m",
            Class::C15 => "Compound Senioren w",
            Class::OO => "Offene Klasse",
        }
    }
    fn comment(&self) -> &'static str {
        match self{
            Class::OO => "Eine Klasse für alle. Die Auflage ist größer als bei den offizielen Klassen. Dadurch ist eine Qualifikation zur Bezirksmeisterschaft ausgeschlossen.",
            _ => "Reguläre Klasse nach Sportornung. Eine Weitermeldung zur Bezirksmeisterschaft ist möglich"
        }
    }
    fn recurve_classes() -> &'static [Self] {
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
    fn barebow_classes() -> &'static [Self] {
        &[
            Self::B10,
            Self::B11,
            Self::B20,
            Self::B21,
            Self::B22,
            Self::B23,
            Self::B30,
            Self::B31,
            Self::B40,
            Self::B41,
            Self::B12,
            Self::B13,
            Self::B14,
            Self::B15,
            Self::OO,
        ]
    }
    fn compound_classes() -> &'static [Self] {
        &[
            Self::C10,
            Self::C11,
            Self::C20,
            Self::C21,
            Self::C22,
            Self::C23,
            Self::C30,
            Self::C31,
            Self::C40,
            Self::C41,
            Self::C12,
            Self::C13,
            Self::C14,
            Self::C15,
            Self::OO,
        ]
    }
    fn in_range(&self, dob: NaiveDate) -> bool {
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
            Class::C20 => (13, 14),
            Class::C21 => (13, 14),
            Class::C22 => (11, 12),
            Class::C23 => (11, 12),
            Class::C30 => (15, 17),
            Class::C31 => (15, 17),
            Class::C40 => (18, 20),
            Class::C41 => (18, 20),
            Class::C12 => (50, 65),
            Class::C13 => (50, 65),
            Class::C14 => (66, 120),
            Class::C15 => (66, 120),
            Class::B10 => (21, 49),
            Class::B11 => (21, 49),
            Class::B20 => (13, 14),
            Class::B21 => (13, 14),
            Class::B22 => (11, 12),
            Class::B23 => (11, 12),
            Class::B30 => (15, 17),
            Class::B31 => (15, 17),
            Class::B40 => (18, 20),
            Class::B41 => (18, 20),
            Class::B12 => (50, 65),
            Class::B13 => (50, 65),
            Class::B14 => (66, 120),
            Class::B15 => (66, 120),
            Class::OO => (15, 120),
        };

        let date_range = (*SEASON_START - Months::new(year_range.1 * 12))
            ..(*SEASON_START - Months::new((year_range.0 - 1) * 12));
        date_range.contains(&dob)
    }
    fn classes_for(dob: NaiveDate, bow_type: BowType) -> Vec<Class> {
        match bow_type {
            BowType::Recurve => Self::recurve_classes(),
            BowType::Compound => Self::compound_classes(),
            BowType::Barebow => Self::barebow_classes(),
        }
        .into_iter()
        .filter(|c| c.in_range(dob))
        .map(|&c| c)
        .collect()
    }
}

impl Default for Class {
    fn default() -> Self {
        // default class for Recurve (default) and dob 1970 (default)
        Self::R12
    }
}

#[test]
fn test_in_range() {
    assert!(!Class::R10.in_range(NaiveDate::from_ymd_opt(1973, 12, 31).unwrap()));
    assert!(Class::R10.in_range(NaiveDate::from_ymd_opt(1974, 1, 1).unwrap()));
    assert!(Class::R10.in_range(NaiveDate::from_ymd_opt(2002, 12, 31).unwrap()));
    assert!(!Class::R10.in_range(NaiveDate::from_ymd_opt(2003, 1, 1).unwrap()));
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        target_face: TargetFace::Full,
        ..Default::default()
    }
}

enum Msg {
    FirstNameChanged(String),
    LastNameChanged(String),
    DateOfBirthChanged(String),
    MailChanged(String),
    BowTypeChange(BowType),
    ClassChanged(Class),
    TargetFaceChanged(TargetFace),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FirstNameChanged(n) => model.first_name = n,
        Msg::LastNameChanged(n) => model.last_name = n,
        Msg::DateOfBirthChanged(dob) => {
            model.date_of_birth = match chrono::NaiveDate::parse_from_str(&dob, "%Y-%m-%d") {
                Ok(valid) => valid,
                Err(e) => {
                    seed::error!("Date of birth is not valid:", e);
                    Default::default()
                }
            };
            model.check_and_update_cls(orders);
        }
        Msg::MailChanged(mail) => {
            model.mail = if EmailAddress::is_valid(&mail) {
                InsertedMail::Valid(mail)
            } else {
                InsertedMail::Invalid(mail)
            }
        }
        Msg::BowTypeChange(bt) => {
            seed::log!("Selected bow type", bt);
            model.bow_type = bt;
            model.check_and_update_cls(orders);
        }
        Msg::ClassChanged(cls) => {
            seed::log!("Selected cls", cls.name());
            model.cls = cls;
            model.update_target_face();
        }
        Msg::TargetFaceChanged(tf) => {
            seed::log!("Selected target", tf);
            model.target_face = tf;
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    let dob = model.date_of_birth;
    let bow_type = model.bow_type;
    ul![
        C!("main"),
        li!("Vorname:"),
        li!(input!(
            attrs!(At::Value => model.first_name),
            input_ev(Ev::Input, Msg::FirstNameChanged)
        )),
        li!("Nachname:"),
        li!(input!(
            attrs!(At::Value => model.last_name),
            input_ev(Ev::Input, Msg::LastNameChanged)
        )),
        li!("Email Adresse:"),
        li!(
            input!(
                attrs!(At::Value => model.mail, At::Type => "email", At::Style => format!("color: {}", if model.mail.is_valid(){"black"} else {"red"}))
            ),
            input_ev(Ev::Input, Msg::MailChanged)
        ),
        li!("Geburtsdatum:"),
        li!(input!(
            attrs!(At::Value => model.date_of_birth, At::Type => "date", ),
            input_ev(Ev::Input, Msg::DateOfBirthChanged)
        )),
        li!(br!()),
        li!("Bogenart:"),
        li!(
            input!(
                attrs!(At::Type => "radio", At::Name => "bow_type", At::Id => "recurve"),
                if model.bow_type.is_recurve() {
                    Some(attrs!("checked" => AtValue::None))
                } else {
                    None
                },
                input_ev(Ev::Input, |_| Msg::BowTypeChange(BowType::Recurve))
            ),
            label!("Recurve", attrs!(At::For => "recurve")),
            input!(
                attrs!(At::Type => "radio", At::Name => "bow_type", At::Id => "blank"),
                if model.bow_type.is_barebow() {
                    Some(attrs!("checked" => AtValue::None))
                } else {
                    None
                },
                input_ev(Ev::Input, |_| Msg::BowTypeChange(BowType::Barebow))
            ),
            label!("Blank", attrs!(At::For => "blank")),
            input!(
                attrs!(At::Type => "radio", At::Name => "bow_type", At::Id => "compound", ),
                if model.bow_type.is_compound() {
                    Some(attrs!("checked" => AtValue::None))
                } else {
                    None
                },
                input_ev(Ev::Input, |_| Msg::BowTypeChange(BowType::Compound))
            ),
            label!("Compound", attrs!(At::For => "compound"))
        ),
        li!(em!(match model.bow_type {
            BowType::Recurve => "Der Recurve-Bogen ist am weitesten verbreitet. Er hat ein Visier und optional ein Stabilisationssystem und einen Klicker",
            BowType::Compound => "Der Compound-Bogen ist einfach zu erkennen an den Rollen am oberen und unteren Ende, welche das Haltegewicht im Vollauszug reduzieren.",
            BowType::Barebow => "Der Blank-Bogen ist der einfachste Bogen. Hier ist kein Visier erlaubt. Auch andere Anbauten sind stark reglementiert.",
        })),
        li!(br!()),
        li!("Klasse:"),
        li!(
            attrs!(At::Name => "fcls"),
            select!(
                attrs!(At::Name => "Class",At::AutoComplete => "off", At::Value => model.cls.name(), At::Required => AtValue::None),
                match model.bow_type {
                    BowType::Recurve => Class::recurve_classes(),
                    BowType::Compound => Class::compound_classes(),
                    BowType::Barebow => Class::barebow_classes(),
                }
                .into_iter()
                .filter(|cls| cls.in_range(model.date_of_birth))
                .map(|cls| option!(
                    cls.name(),
                    attrs!(At::Value => cls.name()),
                    IF!(*cls == model.cls => attrs!(At::Selected => AtValue::None)),
                    ev(Ev::Input, |_| { Msg::ClassChanged(*cls) })
                ))
                .collect::<Vec<_>>(),
                input_ev(Ev::Input, move |cls_name| {
                    Msg::ClassChanged(
                        Class::classes_for(dob, bow_type)
                            .into_iter()
                            .filter(|cls| cls.name() == cls_name)
                            .next()
                            .unwrap(),
                    )
                })
            )
        ),
        li!(em!(model.cls.comment())),
        li!(br!()),
        IF!(!matches!(model.target_face, TargetFace::NotApplicaple) => li!("Auflage:")),
        IF!(!matches!(model.target_face, TargetFace::NotApplicaple) => li!(
            input!(
                attrs!(At::Type => "radio", At::Name => "target_face", At::Id => "40"),
                if matches!(model.target_face, TargetFace::Full) {
                    Some(attrs!("checked" => AtValue::None))
                } else {
                    None
                },
                input_ev(Ev::Input, |_| Msg::TargetFaceChanged(TargetFace::Full))
            ),
            label!("40er", attrs!(At::For => "40")),
            input!(
                attrs!(At::Type => "radio", At::Name => "target_face", At::Id => "Spot"),
                if matches!(model.target_face, TargetFace::Spot) {
                    Some(attrs!("checked" => AtValue::None))
                } else {
                    None
                },
                input_ev(Ev::Input, |_| Msg::TargetFaceChanged(TargetFace::Spot))
            ),
            label!("Spot", attrs!(At::For => "Spot")),

        )),
        li!(br!()),
        li!(button!("Anmelden", IF!(model.first_name.is_empty()||model.last_name.is_empty()|| !model.mail.is_valid() => attrs!(At::Disabled => AtValue::None))))
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
