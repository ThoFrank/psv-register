use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum Locale {
    En,
    #[default]
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

#[test]
fn test_default_locale() {
    assert_eq!(Locale::default(), Locale::De)
}
