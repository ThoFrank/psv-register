use crate::{schema, CONFIG, HANDLEBARS};
use axum::{http::StatusCode, response::IntoResponse, Json};
use common::archer::Archer;
use common::class::Class;
use diesel::prelude::*;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use log::warn;
use std::collections::BTreeMap;

#[axum::debug_handler]
pub async fn create_archer(
    Json(payload): Json<Archer>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("Received {} {}", payload.first_name, payload.last_name);

    let mut connection = crate::db::establish_connection();
    diesel::insert_into(schema::archers::table)
        .values(crate::models::InsertableArcher {
            session: 1,
            division: match payload.class() {
                c if Class::recurve_classes().contains(&c) => "R".to_string(),
                c if Class::barebow_classes().contains(&c) => "B".to_string(),
                c if Class::compound_classes().contains(&c) => "C".to_string(),
                _ => unreachable!(),
            },
            class: format!("{:?}", payload.class()),
            target: format!("{:?}", payload.target_face()),
            individual_qualification: 1,
            team_qualification: 1,
            individual_final: 1,
            team_final: 1,
            mixed_team_final: 1,
            last_name: payload.last_name.clone(),
            first_name: payload.first_name.clone(),
            gender: None,
            country_code: "PSV".to_string(),
            country_name: "PSV München".to_string(),
            ..Default::default()
        })
        .execute(&mut connection)
        .map_err(|e| {
            warn!(
                "Couldn't write to database for request {:?}. Error: {:?}",
                payload, e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Speichern der Anmeldung fehlgeschlagen!".to_string(),
            )
        })?;

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
                .map_err(|e| {
                    warn!("received wrong email format: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Falsches Email Format!".to_string(),
                    )
                })
                .unwrap(),
        ))
        .to(Mailbox::new(
            Some(format!("{} {}", payload.first_name, payload.last_name)),
            payload.mail.as_str().parse().unwrap(),
        ))
        .header(lettre::message::header::ContentType::TEXT_PLAIN)
        .subject(&CONFIG.read().mail_message.subject)
        .body(HANDLEBARS.read().render("user_mail", &email_data).unwrap())
        .unwrap();

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay(&CONFIG.read().mail_server.smtp_server)
            .unwrap()
            .credentials(credentials)
            .build();

    mailer.send(email).await.map_err(|e| {
        warn!(
            "Could not send mail for request {:?}. Error: {:?}",
            payload, e
        );
        (
            StatusCode::BAD_REQUEST,
            format!(
                "Email an {} konnte nicht gesendet werden. Anmeldung gespeichert",
                payload.mail.as_str()
            ),
        )
    })?;
    Ok((StatusCode::CREATED, Json(payload)))
}

pub async fn list_archers() -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "501 Not implemented!")
}
