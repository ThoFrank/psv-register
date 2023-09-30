mod archer;
mod registrator;

use archer::ArcherModel;
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

use chrono::prelude::*;
use common::{bow_type::BowType, class::Class, target_face::TargetFace};
use seed::{prelude::*, *};

#[derive(Serialize, Deserialize)]
struct Model {
    registrator: Registrator,
    archers: Vec<ArcherModel>,

    submitting: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Registrator {
    name: String,
    mail: InsertedMail,
    comment: String,
    club: String,
}

thread_local! {
    static BASE_URL: std::cell::RefCell<Url> = std::cell::RefCell::new(Url::new());
}

impl Model {
    fn new() -> Self {
        let date = NaiveDate::default();
        let cls = Class::classes_for(date, BowType::Recurve)[0];
        Model {
            registrator: Registrator {
                name: String::new(),
                mail: InsertedMail::Invalid(String::new()),
                comment: String::new(),
                club: String::new(),
            },
            archers: vec![ArcherModel::default()],
            submitting: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
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

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    BASE_URL.with(|base_url| {
        *base_url.borrow_mut() = url.to_base_url();
    });
    let window = window();
    let Some(session_storage) = window.session_storage().ok().flatten() else {
        seed::log!("Couldn't load session storage");
        return Model::new();
    };
    if let Some(ser_model) = session_storage.get_item("model").unwrap() {
        match serde_json::from_str::<Model>(&ser_model) {
            Ok(mut model) => {
                model.submitting = false;
                for (index, archer) in model.archers.iter_mut().enumerate() {
                    archer.check_and_update_cls(index, orders);
                    archer.update_target_face();
                }
                model
            }
            Err(_) => {
                seed::error!("Failed to load stored session");
                Model::new()
            }
        }
    } else {
        Model::new()
    }
}

pub enum Msg {
    ArcherMsg(usize, archer::ArcherMsg),
    AddArcher,

    NameChanged(String),
    MailChanged(String),
    CommentChanged(String),

    Submit,
    RegistrationFailed(String),
    RegistrationOk,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::MailChanged(mail) => {
            model.registrator.mail = if EmailAddress::is_valid(&mail) {
                InsertedMail::Valid(mail)
            } else {
                InsertedMail::Invalid(mail)
            }
        }
        Msg::Submit => {
            model.submitting = true;
            let mail = match &model.registrator.mail {
                InsertedMail::Invalid(_) => unreachable!(),
                InsertedMail::Valid(mail) => EmailAddress::from_str(mail).unwrap(),
            };
            orders.perform_cmd(post_participants(common::line_data::CreateArchersPayload {
                name: model.registrator.name.clone(),
                mail: mail.clone(),
                comment: model.registrator.comment.clone(),
                club: model.registrator.club.clone(),
                archers: model
                    .archers
                    .iter()
                    .map(|a| {
                        common::archer::Archer::new(
                            a.first_name.clone(),
                            a.last_name.clone(),
                            mail.clone(),
                            a.date_of_birth,
                            a.cls.expect("Submission only possible if class is set"),
                            a.selected_target_face,
                            model.registrator.comment.clone(),
                        )
                        .expect("It shouldn't be possible to produce invalid values")
                    })
                    .collect(),
            }));
        }
        Msg::RegistrationFailed(err) => {
            seed::window()
                .alert_with_message(&format!("Fehler! {err:?}"))
                .ok();
            seed::error!("Submission failed!", err);
            model.submitting = false;
        }
        Msg::RegistrationOk => {
            seed::window()
                .alert_with_message("Anmeldung erfolgreich. Bestätigungsmail wurde abgeschickt.")
                .ok();
            seed::log!("Submission ok!");
            *model = Model {
                registrator: model.registrator.clone(),
                ..Model::new()
            }
        }
        Msg::CommentChanged(c) => model.registrator.comment = c,
        Msg::ArcherMsg(index, a_msg) => {
            archer::update_archer(a_msg, index, &mut model.archers[index], orders)
        }
        Msg::AddArcher => {
            model.archers.push(ArcherModel::default());
        }
        _ => todo!(),
    }

    if let Some(session_storage) = window().session_storage().ok().flatten() {
        session_storage
            .set_item("model", &serde_json::to_string(&model).unwrap())
            .unwrap()
    }
}

fn view(model: &Model) -> Node<Msg> {
    // let dob = model.date_of_birth;
    // let bow_type = model.bow_type;
    ul![
        C!("main"),
        registrator::view_registrator(&model.registrator),
        model
            .archers
            .iter()
            .enumerate()
            .map(|(index, archer)| { li!(archer::archer_view(archer, index)) }),
        // li!("Vorname:"),
        // li!(input!(
        //     attrs!(At::Value => model.first_name),
        //     input_ev(Ev::Input, Msg::FirstNameChanged)
        // )),
        // li!("Nachname:"),
        // li!(input!(
        //     attrs!(At::Value => model.last_name),
        //     input_ev(Ev::Input, Msg::LastNameChanged)
        // )),
        // li!("Email Adresse:"),
        // li!(
        //     input!(
        //         attrs!(At::Value => model.mail, At::Type => "email", At::Style => format!("color: {}", if model.mail.is_valid(){"black"} else {"red"}))
        //     ),
        //     input_ev(Ev::Input, Msg::MailChanged)
        // ),
        // li!("Geburtsdatum:"),
        // li!(input!(
        //     attrs!(At::Value => model.date_of_birth, At::Type => "date", ),
        //     input_ev(Ev::Input, Msg::DateOfBirthChanged)
        // )),
        // li!(br!()),
        // li!("Bogenart:"),
        // li!(
        //     input!(
        //         attrs!(At::Type => "radio", At::Name => "bow_type", At::Id => "recurve"),
        //         if model.bow_type.is_recurve() {
        //             Some(attrs!("checked" => AtValue::None))
        //         } else {
        //             None
        //         },
        //         input_ev(Ev::Input, |_| Msg::BowTypeChange(BowType::Recurve))
        //     ),
        //     label!("Recurve", attrs!(At::For => "recurve")),
        //     input!(
        //         attrs!(At::Type => "radio", At::Name => "bow_type", At::Id => "blank"),
        //         if model.bow_type.is_barebow() {
        //             Some(attrs!("checked" => AtValue::None))
        //         } else {
        //             None
        //         },
        //         input_ev(Ev::Input, |_| Msg::BowTypeChange(BowType::Barebow))
        //     ),
        //     label!("Blank", attrs!(At::For => "blank")),
        //     input!(
        //         attrs!(At::Type => "radio", At::Name => "bow_type", At::Id => "compound", ),
        //         if model.bow_type.is_compound() {
        //             Some(attrs!("checked" => AtValue::None))
        //         } else {
        //             None
        //         },
        //         input_ev(Ev::Input, |_| Msg::BowTypeChange(BowType::Compound))
        //     ),
        //     label!("Compound", attrs!(At::For => "compound"))
        // ),
        // li!(em!(match model.bow_type {
        //     BowType::Recurve => "Der Recurve-Bogen ist am weitesten verbreitet. Er hat ein Visier und optional ein Stabilisationssystem und einen Klicker",
        //     BowType::Compound => "Der Compound-Bogen ist einfach zu erkennen an den Rollen am oberen und unteren Ende, welche das Haltegewicht im Vollauszug reduzieren.",
        //     BowType::Barebow => "Der Blank-Bogen ist der einfachste Bogen. Hier ist kein Visier erlaubt. Auch andere Anbauten sind stark reglementiert.",
        // })),
        // li!(br!()),
        // li!("Klasse:"),
        // li!(
        //     attrs!(At::Name => "cls"),
        //     select!(
        //         attrs!(At::Name => "Class",At::AutoComplete => "off", At::Required => AtValue::None),
        //         model.cls.map(|cls| attrs!(At::Value => cls.name())),
        //         match model.bow_type {
        //             BowType::Recurve => Class::recurve_classes(),
        //             BowType::Compound => Class::compound_classes(),
        //             BowType::Barebow => Class::barebow_classes(),
        //         }
        //         .iter()
        //         .filter(|cls| cls.in_range(model.date_of_birth))
        //         .map(|cls| option!(
        //             cls.name(),
        //             attrs!(At::Value => cls.name()),
        //             IF!(Some(*cls) == model.cls => attrs!(At::Selected => AtValue::None)),
        //             ev(Ev::Input, |_| { Msg::ClassChanged(Some(*cls)) })
        //         ))
        //         .collect::<Vec<_>>(),
        //         input_ev(Ev::Input, move |cls_name| {
        //             Msg::ClassChanged(
        //                 Some(Class::classes_for(dob, bow_type)
        //                     .into_iter()
        //                     .find(|cls| cls.name() == cls_name)
        //                     .unwrap()),
        //             )
        //         })
        //     )
        // ),
        // li!(em!(model.cls.map(|cls| cls.comment()))),
        // li!(br!()),
        // li!("Scheibe:"),
        // li!(
        //     model.possible_target_faces.iter().map(|&tf| div![
        //         input!(attrs!(At::Type => "radio", At::Name => "target_face", At::Id => format!("{}", tf)), IF!(model.selected_target_face == tf => attrs!(At::Checked => AtValue::None)),input_ev(Ev::Input, move |_| Msg::TargetFaceChanged(tf))),
        //         label!(format!("{}", tf), attrs!(At::For => format!("{}", tf)))
        //     ]),

        // ),
        // li!(br!()),
        // li!("Kommentar:"),
        // li!(textarea!(
        //     attrs!(At::Value => model.comment),
        //     input_ev(Ev::Input, Msg::CommentChanged)
        // )),
        // li!(br!()),
        // li!(button!(
        //     "Anmelden",
        //     IF!(model.first_name.is_empty() || model.last_name.is_empty() || !model.mail.is_valid() || model.cls.is_none() || model.submitting => attrs!(At::Disabled => AtValue::None)),
        //     input_ev(Ev::Click, |_| Msg::Submit)
        // ))
        li!(button!(
            "Zusätzlicher Eintrag",
            input_ev(Ev::Click, |_| Msg::AddArcher)
        ))
    ]
}

async fn post_participants(data: common::line_data::CreateArchersPayload) -> Msg {
    let url = BASE_URL.with(|base| base.borrow().clone().set_path(["api", "archers"]));
    let request = Request::new(url.to_string())
        .method(Method::Post)
        .json(&data)
        .unwrap();
    let response = match fetch(request).await {
        Ok(r) => r,
        Err(e) => return Msg::RegistrationFailed(format!("{e:?}")),
    };
    let text = response.text().await;
    match response.check_status() {
        Ok(_) => Msg::RegistrationOk,
        Err(e) => {
            seed::log!(e);
            Msg::RegistrationFailed(text.unwrap_or(format!("{e:?}")))
        }
    }
}

pub fn main() {
    App::start("app", init, update, view);
}
