use axum::{http::StatusCode, response::IntoResponse};
use Error::*;

/// All errors produced in the backend
pub enum Error {
    MailError(lettre::transport::smtp::Error),
    DBError(diesel::result::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::MailError(e) => {
                log::warn!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Bestätigungsmail konnte nicht abgeschickt werden.".to_string(),
                )
                    .into_response()
            }
            Error::DBError(e) => {
                log::error!("{}", e);
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "Anmeldung konnte nicht gespeichert werden".to_string(),
                )
                    .into_response()
            }
        }
    }
}

impl From<lettre::transport::smtp::Error> for Error {
    fn from(e: lettre::transport::smtp::Error) -> Self {
        MailError(e)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        DBError(e)
    }
}
