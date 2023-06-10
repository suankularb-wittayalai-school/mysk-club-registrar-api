use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::structs::{
    auth::User, classroom::Classroom, common::MultiLangString, contacts::Contact,
};

#[derive(Deserialize, Serialize, Debug)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BloodType {
    #[serde(rename = "A+")]
    APositive,
    #[serde(rename = "A-")]
    ANegative,
    #[serde(rename = "B+")]
    BPositive,
    #[serde(rename = "B-")]
    BNegative,
    #[serde(rename = "AB+")]
    ABPositive,
    #[serde(rename = "AB-")]
    ABNegative,
    #[serde(rename = "O+")]
    OPositive,
    #[serde(rename = "O-")]
    ONegative,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IdOnlyStudent {
    pub id: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CompactStudent {
    pub id: u32,
    pub prefix: MultiLangString,
    pub first_name: MultiLangString,
    pub last_name: MultiLangString,
    // pub middle_name: Option<MultiLangString>,
    pub profile_url: Option<String>,
    pub birthdate: DateTime<Utc>,
    // pub sex: Sex,
    // pub blood_group: Option<bloodType>,
    // contacts: Vec<Contact>,
    pub student_id: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DefaultStudent {
    pub id: u32,
    pub prefix: MultiLangString,
    pub first_name: MultiLangString,
    pub last_name: MultiLangString,
    pub middle_name: Option<MultiLangString>,
    pub profile_url: Option<String>,
    pub birthdate: DateTime<Utc>,
    pub sex: Sex,
    // pub blood_group: Option<BloodType>,
    pub contacts: Vec<Contact>,
    pub student_id: u32,
    pub class: Option<Classroom>,
    pub class_number: Option<u32>,
    pub user: User,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Student {
    Default(DefaultStudent),
    IdOnly(IdOnlyStudent),
    Compact(CompactStudent),
}
