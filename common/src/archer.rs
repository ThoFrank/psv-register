use chrono::NaiveDate;
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};

use crate::{class::Class, target_face::TargetFace};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Archer {
    first_name: String,
    last_name: String,
    mail: EmailAddress,
    date_of_birth: NaiveDate,
    cls: Class,
    target_face: TargetFace,
}

impl Archer {
    pub fn new(
        first_name: String,
        last_name: String,
        mail: EmailAddress,
        dob: NaiveDate,
        cls: Class,
        target_face: TargetFace,
    ) -> Result<Self, ()> {
        if Class::all_classes()
            .filter(|c| c.in_range(dob))
            .find(|&c| c == cls)
            .is_none()
        {
            return Err(());
        }
        if !TargetFace::for_cls(cls).contains(&target_face) {
            return Err(());
        }
        Ok(Self {
            first_name,
            last_name,
            mail,
            date_of_birth: dob,
            cls,
            target_face,
        })
    }
    pub fn first_name(&self) -> &str {
        &self.first_name
    }
    pub fn last_name(&self) -> &str {
        &self.last_name
    }
}
