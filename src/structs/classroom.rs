use serde::{Deserialize, Serialize};

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
pub struct Classroom {
    pub id: u32,
    pub number: u32,
    pub room: String,
    pub class_advisor: Vec<String>, // TODO: Change to Teacher
    pub students: Vec<String>,      // TODO: Change to Student
    pub contacts: Vec<String>,      // TODO: Change to Contact
    pub year: u32,
}
