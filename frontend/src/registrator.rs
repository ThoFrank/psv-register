use seed::{prelude::*, *};

use crate::Msg;

pub fn view_registrator(model: &crate::Registrator) -> Node<crate::Msg> {
    ul![
        C!("list"),
        li!(h3!["Melder:"]),
        li!("Name des Melders:"),
        li!(input!(
            attrs!(At::Value => model.name),
            input_ev(Ev::Input, Msg::NameChanged)
        )),
        li!("Versinsname:"),
        li!(input!(
            attrs!(At::Value => model.club),
            input_ev(Ev::Input, Msg::ClubChanged)
        )),
        li!("Mail-Adresse:"),
        li!(input!(
            attrs!(At::Value => model.mail, At::Type => "email", At::Style => format!("color: {}", if model.mail.is_valid(){"black"} else {"red"})),
            input_ev(Ev::Input, Msg::MailChanged)
        )),
        li!("Kommentar:"),
        li!(textarea!(
            attrs!(At::Value => model.comment),
            input_ev(Ev::Input, Msg::CommentChanged)
        )),
        li!(br!()),
    ]
}
