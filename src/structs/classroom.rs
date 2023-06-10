use serde::{Deserialize, Serialize};

use crate::structs::{contacts::Contact, student::Student};

#[derive(Serialize, Deserialize, Debug)]
pub struct IdOnlyClassroom {
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompactClassroom {
    pub id: u32,
    pub number: u32,
    pub room: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultClassroom {
    pub id: u32,
    pub number: u32,
    pub room: String,
    // pub class_advisor: Vec<String>, // TODO: Change to Teacher
    pub students: Vec<Student>,
    pub contacts: Vec<Contact>, // TODO: Change to Contact
    pub year: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Classroom {
    Default(DefaultClassroom),
    IdOnly(IdOnlyClassroom),
    Compact(CompactClassroom),
}
