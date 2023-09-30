use seed::{prelude::*, *};

use crate::Msg;

pub fn view_registrator(model: &crate::Registrator) -> Node<crate::Msg> {
    ul![
        li!(h3!["Melder:"]),
        li!("Name:"),
        li!(input!(
            attrs!(At::Value => model.name),
            input_ev(Ev::Input, Msg::NameChanged)
        ))
    ]
}
