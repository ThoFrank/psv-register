use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub port: u16,
    pub mail_server: MailServerConfig,
    pub mail_message: MailMessageConfig,
}

#[derive(Serialize, Deserialize, Default)]
pub struct MailServerConfig {
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

#[derive(Serialize, Deserialize)]
pub struct MailMessageConfig {
    pub sender_name: String,
    pub sender_address: EmailAddress,
    pub subject: String,
}

impl Default for MailMessageConfig {
    fn default() -> Self {
        Self {
            sender_name: String::new(),
            sender_address: EmailAddress::from_str("example@mail.com").unwrap(),
            subject: String::new(),
        }
    }
}
