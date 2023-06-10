use serde::{Deserialize, Serialize};

use crate::structs::common::MultiLangString;

#[derive(Debug)]
pub enum ContactType {
    Phone,
    Email,
    Facebook,
    Line,
    Instagram,
    Website,
    Discord,
    Other,
}

impl ContactType {
    pub fn to_string(&self) -> String {
        match self {
            ContactType::Phone => "phone".to_string(),
            ContactType::Email => "email".to_string(),
            ContactType::Facebook => "facebook".to_string(),
            ContactType::Line => "line".to_string(),
            ContactType::Instagram => "instagram".to_string(),
            ContactType::Website => "website".to_string(),
            ContactType::Discord => "discord".to_string(),
            ContactType::Other => "other".to_string(),
        }
    }
    pub fn from_string(role: &str) -> ContactType {
        match role {
            "Phone" => ContactType::Phone,
            "Email" => ContactType::Email,
            "Facebook" => ContactType::Facebook,
            "Line" => ContactType::Line,
            "Instagram" => ContactType::Instagram,
            "Website" => ContactType::Website,
            "Discord" => ContactType::Discord,
            "Other" => ContactType::Other,
            _ => ContactType::Other,
        }
    }

    pub fn new(role: &str) -> ContactType {
        self::ContactType::from_string(role)
    }
}

impl Serialize for ContactType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ContactType {
    fn deserialize<D>(deserializer: D) -> Result<ContactType, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(ContactType::from_string(&s))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultContact {
    pub id: u32,
    pub name: MultiLangString,
    pub value: String,
    pub contact_type: ContactType,
    pub include_student: bool,
    pub include_teacher: bool,
    pub include_parents: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdOnlyContact {
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompactContact {
    pub id: u32,
    pub name: MultiLangString,
    pub value: String,
    pub contact_type: ContactType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Contact {
    Default(DefaultContact),
    IdOnly(IdOnlyContact),
    Compact(CompactContact),
}
