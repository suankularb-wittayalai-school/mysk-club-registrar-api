use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::structs::common::MultiLangString;

use super::common::{FetchLevel, FlexibleMultiLangString};

#[derive(Debug, Clone, Copy, ToSchema)]
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
            ContactType::Phone => "Phone".to_string(),
            ContactType::Email => "Email".to_string(),
            ContactType::Facebook => "Facebook".to_string(),
            ContactType::Line => "Line".to_string(),
            ContactType::Instagram => "Instagram".to_string(),
            ContactType::Website => "Website".to_string(),
            ContactType::Discord => "Discord".to_string(),
            ContactType::Other => "Other".to_string(),
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
}

impl sqlx::Type<sqlx::Postgres> for ContactType {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("contact_type")
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for ContactType {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let s = self.to_string();
        <String as sqlx::Encode<sqlx::Postgres>>::encode(s, buf)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for ContactType {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(ContactType::from_string(&s))
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
pub struct CreateContact {
    pub name: Option<FlexibleMultiLangString>,
    pub value: String,
    pub contact_type: ContactType,
    pub include_students: Option<bool>,
    pub include_teachers: Option<bool>,
    pub include_parents: Option<bool>,
}

#[derive(Debug, FromRow)]
struct ContactTable {
    pub id: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub name_th: Option<String>,
    pub name_en: Option<String>,
    pub value: String,
    pub contact_type: ContactType,
    pub include_students: Option<bool>,
    pub include_teachers: Option<bool>,
    pub include_parents: Option<bool>,
}

impl ContactTable {
    pub async fn get_by_id(pool: &sqlx::PgPool, id: u32) -> Result<ContactTable, sqlx::Error> {
        let res = sqlx::query_as!(
            ContactTable,
            r#"
            SELECT id, created_at, name_th, name_en, value, type as "contact_type: _", include_students, include_teachers, include_parents FROM contacts
            WHERE id = $1
            "#,
            id as i64
        )
        .fetch_one(pool)
        .await?;
        Ok(res)
    }

    pub async fn get_from_ids(
        pool: &sqlx::PgPool,
        ids: Vec<i64>,
    ) -> Result<Vec<ContactTable>, sqlx::Error> {
        let ids: Vec<i32> = ids.into_iter().map(|x| x as i32).collect();

        let res = sqlx::query_as!(
            ContactTable,
            r#"
            SELECT id, created_at, name_th, name_en, value, type as "contact_type: _", include_students, include_teachers, include_parents FROM contacts
            WHERE id in (SELECT unnest($1::int[]))
            "#,
            &ids
        )
        .fetch_all(pool)
        .await?;
        Ok(res)
    }

    pub async fn create_contact(
        pool: &sqlx::PgPool,
        contact: &CreateContact,
    ) -> Result<Self, sqlx::Error> {
        let res = sqlx::query!(
            r#"
            INSERT INTO contacts (name_th, name_en, value, type, include_students, include_teachers, include_parents)
            VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id
            "#,
            match &contact.name {
                Some(name) => name.th.clone(),
                None => None,
            },
            match &contact.name {
                Some(name) => name.en.clone(),
                None => None,
            },
            contact.value,
            contact.contact_type as ContactType,
            contact.include_students,
            contact.include_teachers,
            contact.include_parents
        )
        .fetch_one(pool)
        .await?;

        let contact_id = res.id;

        Ok(Self::get_by_id(pool, contact_id as u32).await?)
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct DefaultContact {
    pub id: u32,
    pub name: MultiLangString,
    pub value: String,
    pub contact_type: ContactType,
    pub include_student: Option<bool>,
    pub include_teacher: Option<bool>,
    pub include_parents: Option<bool>,
}

impl DefaultContact {
    fn from_table(contact: ContactTable) -> Self {
        Self {
            id: contact.id as u32,
            name: MultiLangString::new(contact.name_en, contact.name_th.unwrap_or("".to_string())),
            value: contact.value,
            contact_type: contact.contact_type,
            include_student: contact.include_students,
            include_teacher: contact.include_teachers,
            include_parents: contact.include_parents,
        }
    }

    pub async fn get_by_id(pool: &sqlx::PgPool, id: u32) -> Result<DefaultContact, sqlx::Error> {
        let res = ContactTable::get_by_id(pool, id).await?;
        Ok(DefaultContact {
            id: res.id as u32,
            name: MultiLangString::new(res.name_en, res.name_th.unwrap_or("".to_string())),
            value: res.value,
            contact_type: res.contact_type,
            include_student: res.include_students,
            include_teacher: res.include_teachers,
            include_parents: res.include_parents,
        })
    }

    pub async fn get_from_ids(
        pool: &sqlx::PgPool,
        ids: Vec<i64>,
    ) -> Result<Vec<DefaultContact>, sqlx::Error> {
        let res = ContactTable::get_from_ids(pool, ids).await?;
        let mut contacts: Vec<DefaultContact> = Vec::new();
        for contact in res {
            contacts.push(DefaultContact {
                id: contact.id as u32,
                name: MultiLangString::new(
                    contact.name_en,
                    contact.name_th.unwrap_or("".to_string()),
                ),
                value: contact.value,
                contact_type: contact.contact_type,
                include_student: contact.include_students,
                include_teacher: contact.include_teachers,
                include_parents: contact.include_parents,
            });
        }
        Ok(contacts)
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct IdOnlyContact {
    pub id: u32,
}

impl IdOnlyContact {
    fn from_table(table: ContactTable) -> Self {
        IdOnlyContact {
            id: table.id as u32,
        }
    }

    pub async fn get_by_id(pool: &sqlx::PgPool, id: u32) -> Result<IdOnlyContact, sqlx::Error> {
        let res = ContactTable::get_by_id(pool, id).await?;
        Ok(IdOnlyContact { id: res.id as u32 })
    }

    pub async fn get_from_ids(
        pool: &sqlx::PgPool,
        ids: Vec<i64>,
    ) -> Result<Vec<IdOnlyContact>, sqlx::Error> {
        let res = ContactTable::get_from_ids(pool, ids).await?;
        let mut contacts: Vec<IdOnlyContact> = Vec::new();
        for contact in res {
            contacts.push(IdOnlyContact {
                id: contact.id as u32,
            });
        }
        Ok(contacts)
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CompactContact {
    pub id: u32,
    pub name: MultiLangString,
    pub value: String,
    pub contact_type: ContactType,
}

impl CompactContact {
    fn from_table(contact: ContactTable) -> Self {
        CompactContact {
            id: contact.id as u32,
            name: MultiLangString::new(contact.name_en, contact.name_th.unwrap_or("".to_string())),
            value: contact.value,
            contact_type: contact.contact_type,
        }
    }

    pub async fn get_by_id(pool: &sqlx::PgPool, id: u32) -> Result<CompactContact, sqlx::Error> {
        let res = ContactTable::get_by_id(pool, id).await?;
        Ok(CompactContact {
            id: res.id as u32,
            name: MultiLangString::new(res.name_en, res.name_th.unwrap_or("".to_string())),
            value: res.value,
            contact_type: res.contact_type,
        })
    }

    pub async fn get_from_ids(
        pool: &sqlx::PgPool,
        ids: Vec<i64>,
    ) -> Result<Vec<CompactContact>, sqlx::Error> {
        let res = ContactTable::get_from_ids(pool, ids).await?;
        let mut contacts: Vec<CompactContact> = Vec::new();
        for contact in res {
            contacts.push(CompactContact {
                id: contact.id as u32,
                name: MultiLangString::new(
                    contact.name_en,
                    contact.name_th.unwrap_or("".to_string()),
                ),
                value: contact.value,
                contact_type: contact.contact_type,
            });
        }
        Ok(contacts)
    }
}

#[derive(Deserialize, Debug, ToSchema)]
pub enum Contact {
    Default(DefaultContact),
    IdOnly(IdOnlyContact),
    Compact(CompactContact),
}

impl Contact {
    fn from_table(
        pool: &sqlx::PgPool,
        contact: ContactTable,
        fetch_level: FetchLevel,
    ) -> Result<Contact, sqlx::Error> {
        match fetch_level {
            FetchLevel::Default => Ok(Contact::Default(DefaultContact::from_table(contact))),
            FetchLevel::IdOnly => Ok(Contact::IdOnly(IdOnlyContact::from_table(contact))),
            FetchLevel::Compact => Ok(Contact::Compact(CompactContact::from_table(contact))),
        }
    }

    pub async fn get_by_id(
        pool: &sqlx::PgPool,
        id: u32,
        fetch_level: FetchLevel,
    ) -> Result<Contact, sqlx::Error> {
        match fetch_level {
            FetchLevel::Default => Ok(Contact::Default(DefaultContact::get_by_id(pool, id).await?)),
            FetchLevel::IdOnly => Ok(Contact::IdOnly(IdOnlyContact::get_by_id(pool, id).await?)),
            FetchLevel::Compact => Ok(Contact::Compact(CompactContact::get_by_id(pool, id).await?)),
        }
    }

    pub async fn get_from_ids(
        pool: &sqlx::PgPool,
        ids: Vec<i64>,
        fetch_level: FetchLevel,
    ) -> Result<Vec<Contact>, sqlx::Error> {
        match fetch_level {
            FetchLevel::Default => {
                let res = DefaultContact::get_from_ids(pool, ids).await?;
                let mut contacts: Vec<Contact> = Vec::new();
                for contact in res {
                    contacts.push(Contact::Default(contact));
                }
                Ok(contacts)
            }
            FetchLevel::IdOnly => {
                let res = IdOnlyContact::get_from_ids(pool, ids).await?;
                let mut contacts: Vec<Contact> = Vec::new();
                for contact in res {
                    contacts.push(Contact::IdOnly(contact));
                }
                Ok(contacts)
            }
            FetchLevel::Compact => {
                let res = CompactContact::get_from_ids(pool, ids).await?;
                let mut contacts: Vec<Contact> = Vec::new();
                for contact in res {
                    contacts.push(Contact::Compact(contact));
                }
                Ok(contacts)
            }
        }
    }

    pub async fn create(
        pool: &sqlx::PgPool,
        contact: &CreateContact,
        fetch_level: FetchLevel,
    ) -> Result<Contact, sqlx::Error> {
        let res = ContactTable::create_contact(pool, contact).await?;

        match fetch_level {
            FetchLevel::Default => Ok(Contact::Default(DefaultContact::from_table(res))),
            FetchLevel::IdOnly => Ok(Contact::IdOnly(IdOnlyContact::from_table(res))),
            FetchLevel::Compact => Ok(Contact::Compact(CompactContact::from_table(res))),
        }
    }
}

impl Serialize for Contact {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            Contact::Default(c) => c.serialize(serializer),
            Contact::IdOnly(c) => c.serialize(serializer),
            Contact::Compact(c) => c.serialize(serializer),
        }
    }
}
