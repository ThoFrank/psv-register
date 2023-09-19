use crate::{error::*, schema, CONFIG, HANDLEBARS};
use axum::{http::StatusCode, response::IntoResponse, Json};
use common::archer::{Archer, RegisteredArcher};
use common::class::Class;
use common::line_data::CreateArchersPayload;
use diesel::prelude::*;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use log::warn;

#[axum::debug_handler]
pub async fn create_archers(
    Json(payload): Json<CreateArchersPayload>,
) -> Result<impl IntoResponse> {
    // println!("Received {} {}", payload.first_name, payload.last_name);
    let total_price: u32 = payload.archers.iter().map(|a| a.class().price()).sum();
    let mail_data = EmailData {
        comment: payload.comment.clone(),
        club: payload.club.clone(),
        mail_address: payload.mail.to_string(),
        name: payload.name.clone(),
        archers: payload.archers.iter().map(|a| a.into()).collect(),
        total_price: format!("{},{}€", total_price / 100, total_price % 100),
    };

    let archers = payload.archers.clone();
    let save_task = tokio::task::spawn_blocking(move || {
        archers
            .into_iter()
            .map(|archer| save_archer(archer))
            .collect::<Result<Vec<()>>>()
    });
    let (save, mail) = tokio::join!(save_task, send_registration_mail(mail_data));
    save.unwrap()?;
    mail?;

    Ok((StatusCode::CREATED, Json(payload)))
}

pub async fn list_archers() -> Result<impl IntoResponse> {
    Ok(Json(get_archers()?))
}

fn save_archer(archer: Archer) -> Result<()> {
    let mut connection = crate::db::establish_connection();
    connection.transaction(|conn| -> Result<()> {
        let inserted_bib: i32 = diesel::insert_into(schema::archers::table)
            .values(crate::models::InsertableArcher {
                session: 1,
                division: match archer.class() {
                    c if Class::recurve_classes().contains(&c) => "R".to_string(),
                    c if Class::barebow_classes().contains(&c) => "B".to_string(),
                    c if Class::compound_classes().contains(&c) => "C".to_string(),
                    _ => unreachable!(),
                },
                class: format!("{:?}", archer.class()),
                target: format!("{:?}", archer.target_face()),
                individual_qualification: 1,
                team_qualification: 1,
                individual_final: 1,
                team_final: 1,
                mixed_team_final: 1,
                last_name: archer.last_name.clone(),
                first_name: archer.first_name.clone(),
                gender: None,
                country_code: "PSV".to_string(),
                country_name: "PSV München".to_string(),
                date_of_birth: archer.date_of_birth().format("%Y-%m-%d").to_string(),
                ..Default::default()
            })
            .returning(schema::archers::bib)
            .get_result(conn)?;

        diesel::insert_into(schema::archer_additions::table)
            .values(crate::models::ArcherAdditions {
                bib: inserted_bib,
                email: archer.mail.as_str().to_owned(),
                comment: archer.comment,
            })
            .execute(conn)?;

        Ok(())
    })
}

async fn send_registration_mail(email_data: EmailData) -> Result<()> {
    let credentials = Credentials::new(
        CONFIG.read().mail_server.smtp_username.clone(),
        CONFIG.read().mail_server.smtp_password.clone(),
    );
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
            Some(email_data.name.clone()),
            email_data.mail_address.parse().unwrap(),
        ))
        .bcc(Mailbox::new(
            Some("Thomas Frank".to_string()),
            "sport@bogen-psv.de".parse().unwrap(),
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
    mailer.send(email).await?;
    Ok(())
}

fn get_archers() -> Result<Vec<RegisteredArcher>> {
    use crate::models::*;
    use crate::schema::archers::dsl::*;
    let mut connection = crate::db::establish_connection();
    let ret = archers.load::<Archer>(&mut connection)?;

    Ok(ret.into_iter().map(|a| a.into()).collect())
}

impl From<crate::models::Archer> for RegisteredArcher {
    fn from(val: crate::models::Archer) -> Self {
        RegisteredArcher {
            first_name: val.first_name,
            last_name: val.last_name,
            class: val.class.parse().unwrap(),
        }
    }
}

#[derive(Debug, serde::Serialize)]
struct EmailData {
    comment: String,
    club: String,
    mail_address: String,
    name: String,
    archers: Vec<EmailArcher>,
    total_price: String,
}

#[derive(Debug, serde::Serialize)]
struct EmailArcher {
    first_name: String,
    last_name: String,
    class: String,
    target: String,
    date_of_birth: String,
    price: String,
}

impl From<&common::archer::Archer> for EmailArcher {
    fn from(val: &common::archer::Archer) -> Self {
        EmailArcher {
            first_name: val.first_name.clone(),
            last_name: val.last_name.clone(),
            class: val.class().name().to_string(),
            target: val.target_face().to_string(),
            date_of_birth: val.date_of_birth().format("%Y-%m-%d").to_string(),
            price: format!(
                "{},{}€",
                val.class().price() / 100,
                val.class().price() % 100
            ),
        }
    }
}
