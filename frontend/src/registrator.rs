use seed::{prelude::*, *};

use crate::Msg;

pub fn view_registrator(model: &crate::Registrator) -> Node<crate::Msg> {
    ul![
        C!("list"),
        li!(h3!["Melder:"]),
        li!("Name des Melders:"),
        li!(input!(
            attrs!(
                At::Value => model.name,
                At::Style =>if model.name.is_empty() {"border: 1px solid red"} else {""}
            ),
            input_ev(Ev::Input, Msg::NameChanged)
        )),
        li!("Versinsname:"),
        li!(input!(
            attrs!(
                At::Value => model.club
                At::Style =>if model.club.is_empty() {"border: 1px solid red"} else {""}
            ),
            input_ev(Ev::Input, Msg::ClubChanged)
        )),
        li!("Mail-Adresse:"),
        li!(input!(
            attrs!(
                At::Value => model.mail,
                At::Type => "email",
                At::Style =>if !model.mail.is_valid() {"border: 1px solid red"} else {""}
            ),
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
