use rust_i18n::t;
use seed::{prelude::*, *};

use crate::Msg;

pub fn view_registrator(model: &crate::Registrator) -> Node<crate::Msg> {
    ul![
        C!("list"),
        li!(h3![t!("Registrator")]),
        li!(t!("Name of Registrator")),
        li!(input!(
            attrs!(
                At::Value => model.name,
                At::Style =>if model.name.is_empty() {"border: 1px solid red"} else {""}
            ),
            input_ev(Ev::Input, Msg::NameChanged)
        )),
        li!(t!("Name of club")),
        li!(input!(
            attrs!(
                At::Value => model.club
                At::Style =>if model.club.is_empty() {"border: 1px solid red"} else {""}
            ),
            input_ev(Ev::Input, Msg::ClubChanged)
        )),
        li!(t!("Mail address")),
        li!(input!(
            attrs!(
                At::Value => model.mail,
                At::Type => "email",
                At::Style =>if !model.mail.is_valid() {"border: 1px solid red"} else {""}
            ),
            input_ev(Ev::Input, Msg::MailChanged)
        )),
        li!(t!("Comment")),
        li!(textarea!(
            attrs!(At::Value => model.comment),
            input_ev(Ev::Input, Msg::CommentChanged)
        )),
        li!(br!()),
    ]
}
