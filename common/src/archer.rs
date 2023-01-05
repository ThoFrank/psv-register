use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{class::Class, target_face::TargetFace};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Archer {
    first_name: String,
    last_name: String,
    mail: String,
    date_of_birth: NaiveDate,
    cls: Class,
    target_face: TargetFace,
}
