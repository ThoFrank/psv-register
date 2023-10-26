use email_address::EmailAddress;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateArchersPayload {
    pub name: String,
    pub mail: EmailAddress,
    pub comment: String,
    pub archers: Vec<crate::archer::Archer>,
}
