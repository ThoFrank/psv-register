use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Locale {
    En,
    De,
}

impl Locale {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "en" => Some(Self::En),
            "de" => Some(Self::De),
            _ => None,
        }
    }
}
