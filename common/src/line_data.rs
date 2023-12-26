use email_address::EmailAddress;

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub struct CreateArchersPayload {
    pub name: String,
    pub mail: EmailAddress,
    pub comment: String,
    pub club: String,
    pub archers: Vec<crate::archer::Archer>,
    #[serde(default)]
    pub locale: crate::locale::Locale,
}

#[test]
fn test_deserialize_create_archers_payload_missing_locale() {
    use std::str::FromStr;
    let json = r#"
        {
            "name": "Foo Bar",
            "mail": "foo@bar.com",
            "comment": "",
            "club": "PSV",
            "archers": []
        }
    "#;
    let payload: CreateArchersPayload = serde_json::from_str(json).unwrap();
    assert_eq!(
        payload,
        CreateArchersPayload {
            name: "Foo Bar".into(),
            mail: EmailAddress::from_str("foo@bar.com").unwrap(),
            comment: "".into(),
            club: "PSV".into(),
            archers: vec![],
            locale: crate::locale::Locale::De
        }
    )
}
