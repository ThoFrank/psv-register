mod archer;
mod registrator;

use archer::ArcherModel;
use common::locale::Locale;
use email_address::EmailAddress;
use rust_i18n::{i18n, t};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

use seed::{prelude::*, *};

i18n!();

#[derive(Serialize, Deserialize)]
struct Model {
    registrator: Registrator,
    archers: Vec<ArcherModel>,

    submitting: bool,
    locale: Locale,
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
        Model {
            registrator: Registrator {
                name: String::new(),
                mail: InsertedMail::Invalid(String::new()),
                comment: String::new(),
                club: String::new(),
            },
            archers: vec![ArcherModel::default()],
            submitting: false,
            locale: Locale::De,
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
    rust_i18n::set_locale("de");
    BASE_URL.with(|base_url| {
        *base_url.borrow_mut() = url.to_base_url();
    });
    let window = window();
    let Some(session_storage) = window.session_storage().ok().flatten() else {
        seed::log!("Couldn't load session storage");
        return Model::new();
    };
    let model = if let Some(ser_model) = session_storage.get_item("model").unwrap() {
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
    };
    match model.locale {
        Locale::En => rust_i18n::set_locale("en"),
        Locale::De => rust_i18n::set_locale("de"),
    }
    model
}

pub enum Msg {
    ArcherMsg(usize, archer::ArcherMsg),
    AddArcher,
    RemoveArcher(usize),

    NameChanged(String),
    ClubChanged(String),
    MailChanged(String),
    CommentChanged(String),

    Submit,
    RegistrationFailed(String),
    RegistrationOk,

    ToggleLanguage,
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
                    .filter(|a| !a.first_name.is_empty() && !a.last_name.is_empty())
                    .map(|a| {
                        common::archer::Archer::new(
                            a.first_name.clone(),
                            a.last_name.clone(),
                            mail.clone(),
                            match a.date_of_birth {
                                archer::DoB::Vaild(dob) => dob,
                                archer::DoB::Invalid(_) => {
                                    unreachable!("Submission only impossible if dob is valid")
                                }
                            },
                            a.cls.expect("Submission only possible if class is set"),
                            a.selected_target_face,
                            model.registrator.comment.clone(),
                            model.registrator.club.clone(),
                            a.session,
                        )
                        .expect("It shouldn't be possible to produce invalid values")
                    })
                    .collect(),
                locale: model.locale,
            }));
        }
        Msg::RegistrationFailed(err) => {
            seed::window()
                .alert_with_message(&format!("{}! {err:?}", t!("Error")))
                .ok();
            seed::error!("Submission failed!", err);
            model.submitting = false;
        }
        Msg::RegistrationOk => {
            seed::window()
                .alert_with_message(&t!("Registration successful message"))
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
        Msg::RemoveArcher(index) => {
            model.archers.remove(index);
        }
        Msg::NameChanged(name) => {
            model.registrator.name = name;
        }
        Msg::ClubChanged(club) => {
            model.registrator.club = club;
        }
        Msg::ToggleLanguage => {
            model.locale = match model.locale {
                Locale::En => Locale::De,
                Locale::De => Locale::En,
            };
            rust_i18n::set_locale(match model.locale {
                Locale::En => "en",
                Locale::De => "de",
            })
        }
    }

    if let Some(session_storage) = window().session_storage().ok().flatten() {
        session_storage
            .set_item("model", &serde_json::to_string(&model).unwrap())
            .unwrap()
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![view_headline(), view_body(model), view_footer()]
}

fn view_body(model: &Model) -> Node<Msg> {
    // let dob = model.date_of_birth;
    // let bow_type = model.bow_type;
    ul![
        C!("list"),
        li!(
            button!(
                attrs!(At::Style => "width: 100%"),
                match model.locale {
                    Locale::En => "Ändere auf DE🇩🇪",
                    Locale::De => "Switch to EN 🇬🇧",
                }
            ),
            input_ev(Ev::Click, |_| Msg::ToggleLanguage)
        ),
        registrator::view_registrator(&model.registrator),
        hr!(),
        model
            .archers
            .iter()
            .enumerate()
            .map(|(index, archer)| { p!(li!(archer::archer_view(archer, index)), hr!()) }),
        li!(button!(
            t!("Add archer"),
            input_ev(Ev::Click, |_| Msg::AddArcher)
        )),
        li!(br!()),
        li!(button!(
            t!("Submit"),
            IF!(model.archers.is_empty() || model.archers.iter().any(|a| !a.ready_for_submission()) || model.registrator.club.is_empty() || !model.registrator.mail.is_valid() || model.submitting => attrs!(At::Disabled => AtValue::None)),
            input_ev(Ev::Click, |_| Msg::Submit)
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

fn view_headline() -> Vec<Node<Msg>> {
    vec![h1!(t!("Headline")), h4!(t!("Headline date"))]
}

fn view_footer() -> Node<Msg> {
    footer![
        a!(
            attrs!(At::Href => "https://bogen-psv.de/datenschutz.html"),
            t!("privacy policy")
        ),
        " - ",
        a!(
            attrs!(At::Href => "https://bogen-psv.de/impressum_small.html"),
            t!("legal notice")
        )
    ]
}

pub fn main() {
    App::start("app", init, update, view);
}
