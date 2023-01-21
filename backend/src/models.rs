use crate::schema::archers;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Archer {
    pub bib: i32,
    pub session: i32,
    pub division: String,
    pub class: String,
    pub target: String,
    pub individual_qualification: i32,
    pub team_qualification: i32,
    pub individual_finale: i32,
    pub team_final: i32,
    pub mixed_team_final: i32,
    pub last_name: String,
    pub first_name: String,
    pub gender: Option<i32>,
    pub country_code: String,
    pub country_name: String,
    pub date_of_birth: String,
    pub subclass: Option<String>,
    pub country_code2: String,
    pub country_name2: String,
    pub country_code3: String,
    pub country_name3: String,
}

#[derive(Insertable, Default)]
#[diesel(table_name = archers)]
pub struct InsertableArcher {
    pub session: i32,
    pub division: String,
    pub class: String,
    pub target: String,
    pub individual_qualification: i32,
    pub team_qualification: i32,
    pub individual_final: i32,
    pub team_final: i32,
    pub mixed_team_final: i32,
    pub last_name: String,
    pub first_name: String,
    pub gender: Option<i32>,
    pub country_code: String,
    pub country_name: String,
    pub date_of_birth: String,
    pub subclass: Option<String>,
    pub country_code_2: String,
    pub country_name_2: String,
    pub country_code_3: String,
    pub country_name_3: String,
}
