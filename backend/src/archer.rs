use std::collections::BTreeMap;

use axum::{http::StatusCode, response::IntoResponse, Json};
use common::archer::Archer;
use lettre::message::Mailbox;

use crate::{CONFIG, HANDLEBARS};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, SmtpTransport, Tokio1Executor, Transport,
};

#[axum::debug_handler]
pub async fn create_archer(Json(payload): Json<Archer>) -> impl IntoResponse {
    println!("Received {} {}", payload.first_name, payload.last_name);

    let credentials = Credentials::new(
        CONFIG.read().mail_server.smtp_username.clone(),
        CONFIG.read().mail_server.smtp_password.clone(),
    );

    let email_data = BTreeMap::from([
        ("first_name", payload.first_name.clone()),
        ("last_name", payload.last_name.clone()),
        (
            "date_of_birth",
            payload.date_of_birth().format("%d.%m.%Y").to_string(),
        ),
        ("class", payload.class().name().to_owned()),
        ("target_face", payload.target_face().to_string()),
    ]);

    let email = Message::builder()
        .from(Mailbox::new(
            Some(CONFIG.read().mail_message.sender_name.clone()),
            CONFIG
                .read()
                .mail_message
                .sender_address
                .as_str()
                .parse()
                .unwrap(),
        ))
        .to(Mailbox::new(
            Some(format!("{} {}", payload.first_name, payload.last_name)),
            payload.mail.as_str().parse().unwrap(),
        ))
        .subject(&CONFIG.read().mail_message.subject)
        .body(HANDLEBARS.read().render("user_mail", &email_data).unwrap())
        .unwrap();

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay(&CONFIG.read().mail_server.smtp_server)
            .unwrap()
            .credentials(credentials)
            .build();

    match mailer.send(email).await {
        Ok(_) => (),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)),
    }

    (StatusCode::CREATED, Json(payload))
}

pub async fn list_archers() -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "501 Not implemented!")
}
