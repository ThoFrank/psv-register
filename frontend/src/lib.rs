use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use chrono::prelude::*;
use common::{bow_type::BowType, class::Class, target_face::TargetFace};
use seed::{prelude::*, *};

#[derive(Serialize, Deserialize)]
struct Model {
    first_name: String,
    last_name: String,
    date_of_birth: NaiveDate,
    mail: InsertedMail,
    bow_type: BowType,
    cls: Option<Class>,

    possible_target_faces: Vec<TargetFace>,
    selected_target_face: TargetFace,
}

impl Model {
    fn check_and_update_cls(&mut self, orders: &mut impl Orders<Msg>) {
        let available_classes: Vec<Class> = match self.bow_type {
            BowType::Recurve => Class::recurve_classes(),
            BowType::Compound => Class::compound_classes(),
            BowType::Barebow => Class::barebow_classes(),
        }
        .iter()
        .filter(|cls| cls.in_range(self.date_of_birth))
        .copied()
        .collect();

        let new_cls = match (self.cls, available_classes.get(0)) {
            (Some(cls), Some(&new)) => {
                if available_classes.contains(&cls) {
                    return;
                } else {
                    Some(new)
                }
            }
            (_, None) => None,
            (None, Some(&new)) => Some(new),
        };

        self.update_target_face();

        orders.send_msg(Msg::ClassChanged(new_cls));
        orders.force_render_now();
    }
    fn update_target_face(&mut self) {
        self.possible_target_faces = if let Some(cls) = self.cls {
            TargetFace::for_cls(cls).to_owned()
        } else {
            Vec::new()
        };
        if !self
            .possible_target_faces
            .contains(&self.selected_target_face)
        {
            self.selected_target_face = *self
                .possible_target_faces
                .get(0)
                .unwrap_or(&TargetFace::Cm40);
        }
    }
}

#[derive(Serialize, Deserialize)]
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

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    let default = {
        let cls = Class::R12;
        Model {
            first_name: String::new(),
            last_name: String::new(),
            date_of_birth: NaiveDate::default(),
            mail: InsertedMail::Invalid(String::new()),
            bow_type: BowType::Recurve,
            cls: Some(cls),
            possible_target_faces: TargetFace::for_cls(cls).to_owned(),
            selected_target_face: TargetFace::for_cls(cls)[0],
        }
    };

    let window = window();
    let Some(local_storage) = window.local_storage().ok().flatten() else {
        seed::log!("Couldn't load local storage");
        return default;
    };
    if let Some(ser_model) = local_storage.get_item("model").unwrap() {
        match serde_json::from_str::<Model>(&ser_model) {
            Ok(mut model) => {
                model.check_and_update_cls(orders);
                model.update_target_face();
                model
            }
            Err(_) => {
                seed::error!("Failed to load stored session");
                default
            }
        }
    } else {
        default
    }
}

enum Msg {
    FirstNameChanged(String),
    LastNameChanged(String),
    DateOfBirthChanged(String),
    MailChanged(String),
    BowTypeChange(BowType),
    ClassChanged(Option<Class>),
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
            seed::log!("Selected cls", cls.map(|cls| cls.name()));
            model.cls = cls;
            model.update_target_face();
        }
        Msg::TargetFaceChanged(tf) => {
            seed::log!("Selected target", tf);
            model.selected_target_face = tf;
        }
    }

    if let Some(local_storage) = window().local_storage().ok().flatten() {
        local_storage
            .set_item("model", &serde_json::to_string(&model).unwrap())
            .unwrap()
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
            attrs!(At::Name => "cls"),
            select!(
                attrs!(At::Name => "Class",At::AutoComplete => "off", At::Required => AtValue::None),
                model.cls.map(|cls| attrs!(At::Value => cls.name())),
                match model.bow_type {
                    BowType::Recurve => Class::recurve_classes(),
                    BowType::Compound => Class::compound_classes(),
                    BowType::Barebow => Class::barebow_classes(),
                }
                .iter()
                .filter(|cls| cls.in_range(model.date_of_birth))
                .map(|cls| option!(
                    cls.name(),
                    attrs!(At::Value => cls.name()),
                    IF!(Some(*cls) == model.cls => attrs!(At::Selected => AtValue::None)),
                    ev(Ev::Input, |_| { Msg::ClassChanged(Some(*cls)) })
                ))
                .collect::<Vec<_>>(),
                input_ev(Ev::Input, move |cls_name| {
                    Msg::ClassChanged(
                        Some(Class::classes_for(dob, bow_type)
                            .into_iter()
                            .find(|cls| cls.name() == cls_name)
                            .unwrap()),
                    )
                })
            )
        ),
        li!(em!(model.cls.map(|cls| cls.comment()))),
        li!(br!()),
        li!("Auflage:"),
        li!(
            model.possible_target_faces.iter().map(|&tf| div![
                input!(attrs!(At::Type => "radio", At::Name => "target_face", At::Id => format!("{}", tf)), IF!(model.selected_target_face == tf => attrs!(At::Checked => AtValue::None)),input_ev(Ev::Input, move |_| Msg::TargetFaceChanged(tf))),
                label!(format!("{}", tf), attrs!(At::For => format!("{}", tf)))
            ]),

        ),
        li!(br!()),
        li!(button!(
            "Anmelden",
            IF!(model.first_name.is_empty() || model.last_name.is_empty() || !model.mail.is_valid() || model.cls.is_none() => attrs!(At::Disabled => AtValue::None)),
            input_ev(Ev::Click, |_| {seed::log!("Pressed submit button!"); None::<Msg>})
        ))
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
