use chrono::NaiveDate;
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};

use crate::{class::Class, target_face::TargetFace};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Archer {
    pub first_name: String,
    pub last_name: String,
    pub mail: EmailAddress,
    pub comment: String,
    date_of_birth: NaiveDate,
    class: Class,
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
        comment: String,
    ) -> Result<Self, ()> {
        if !Class::all_classes()
            .filter(|c| c.in_range(dob))
            .any(|c| c == cls)
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
            class: cls,
            target_face,
            comment,
        })
    }
    pub fn date_of_birth(&self) -> NaiveDate {
        self.date_of_birth
    }
    pub fn class(&self) -> Class {
        self.class
    }
    pub fn target_face(&self) -> TargetFace {
        self.target_face
    }
}
