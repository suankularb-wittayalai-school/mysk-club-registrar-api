use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, __private::de};
use sqlx::{Decode, Encode, FromRow, Postgres, Type};
use uuid::Uuid;

use crate::utils::date::get_current_academic_year;

use super::{
    common::{FetchLevel, MultiLangString},
    contacts::Contact,
    student::Student,
};

#[derive(Debug)]
pub enum ActivityDayHouse {
    Felis,
    Cornicula,
    Sciurus,
    Cyprinus,
}
impl ActivityDayHouse {
    pub fn to_string(&self) -> String {
        match self {
            ActivityDayHouse::Felis => "felis".to_string(),
            ActivityDayHouse::Cornicula => "cornicula".to_string(),
            ActivityDayHouse::Sciurus => "sciurus".to_string(),
            ActivityDayHouse::Cyprinus => "cyprinus".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<ActivityDayHouse> {
        match s {
            "felis" => Some(ActivityDayHouse::Felis),
            "cornicula" => Some(ActivityDayHouse::Cornicula),
            "sciurus" => Some(ActivityDayHouse::Sciurus),
            "cyprinus" => Some(ActivityDayHouse::Cyprinus),
            _ => None,
        }
    }
}

impl Type<Postgres> for ActivityDayHouse {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("activity_day_houses")
    }
}

impl Encode<'_, Postgres> for ActivityDayHouse {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let s = self.to_string();
        <String as sqlx::Encode<sqlx::Postgres>>::encode(s, buf)
    }
}

impl Decode<'_, Postgres> for ActivityDayHouse {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as Decode<Postgres>>::decode(value)?;

        match ActivityDayHouse::from_string(&s) {
            Some(house) => Ok(house),
            None => Err("Invalid house".into()),
        }
    }
}

impl Serialize for ActivityDayHouse {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ActivityDayHouse {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;

        match ActivityDayHouse::from_string(&s) {
            Some(house) => Ok(house),
            None => Err(serde::de::Error::custom("Invalid house")),
        }
    }
}

#[derive(Debug)]
pub enum SubmissionStatus {
    Pending,
    Approved,
    Declined,
}

impl SubmissionStatus {
    pub fn to_string(&self) -> String {
        match self {
            SubmissionStatus::Pending => "pending".to_string(),
            SubmissionStatus::Approved => "approved".to_string(),
            SubmissionStatus::Declined => "declined".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<SubmissionStatus> {
        match s {
            "pending" => Some(SubmissionStatus::Pending),
            "approved" => Some(SubmissionStatus::Approved),
            "declined" => Some(SubmissionStatus::Declined),
            _ => None,
        }
    }
}

impl Type<Postgres> for SubmissionStatus {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("submission_status")
    }
}

impl Encode<'_, Postgres> for SubmissionStatus {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let s = self.to_string();
        <String as sqlx::Encode<sqlx::Postgres>>::encode(s, buf)
    }
}

impl Decode<'_, Postgres> for SubmissionStatus {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as Decode<Postgres>>::decode(value)?;

        match SubmissionStatus::from_string(&s) {
            Some(status) => Ok(status),
            None => Err("Invalid submission status".into()),
        }
    }
}

#[derive(FromRow, Debug)]
struct ClubTable {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub name_th: String,
    pub name_en: Option<String>,
    pub description_th: Option<String>,
    pub description_en: Option<String>,
    pub main_room: Option<String>,
    pub logo_url: Option<String>,
    pub background_color: Option<String>,
    pub accent_color: Option<String>,
    pub house: Option<ActivityDayHouse>,
    pub map_location: Option<i64>,
}

impl ClubTable {
    pub async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Self, sqlx::Error> {
        let res = sqlx::query_as!(
            Self,
            r#"
            SELECT clubs.id, clubs.created_at, name_th, name_en, description_th, description_en, main_room, logo_url, background_color, accent_color, house as "house: _", map_location
            FROM clubs INNER JOIN organizations ON clubs.organization_id = organizations.id
            WHERE clubs.id = $1
            "#,
            id
        ).fetch_one(pool)
        .await;

        match res {
            Ok(club) => Ok(club),
            Err(e) => Err(e),
        }
    }

    pub async fn get_members(
        pool: &sqlx::PgPool,
        id: Uuid,
        year: Option<u32>,
        fetch_level: Option<FetchLevel>,
        descendant_fetch_level: Option<FetchLevel>,
    ) -> Result<Vec<Student>, sqlx::Error> {
        let year = match year {
            Some(year) => year,
            None => get_current_academic_year(),
        } as i64;

        // select student_id from club_members where club_id = $1 and academic_year = $2 and membership_status = 'approved'

        let res = sqlx::query!(
            r#"
            SELECT student_id
            FROM club_members
            WHERE club_id = $1 AND year = $2 AND membership_status = 'approved'
            "#,
            id,
            year
        )
        .fetch_all(pool)
        .await?;

        Ok(Student::get_from_ids(
            pool,
            res.iter().map(|r| r.student_id).collect(),
            fetch_level,
            descendant_fetch_level,
        )
        .await?)
    }

    pub async fn get_staffs(
        pool: &sqlx::PgPool,
        id: Uuid,
        year: Option<u32>,
        fetch_level: Option<FetchLevel>,
        descendant_fetch_level: Option<FetchLevel>,
    ) -> Result<Vec<Student>, sqlx::Error> {
        let year = match year {
            Some(year) => year,
            None => get_current_academic_year(),
        } as i64;

        let res = sqlx::query!(
            r#"
            SELECT student_id
            FROM club_staffs
            WHERE club_id = $1 AND year = $2
            "#,
            id,
            year
        )
        .fetch_all(pool)
        .await?;

        Ok(Student::get_from_ids(
            pool,
            res.iter().map(|r| r.student_id).collect(),
            fetch_level,
            descendant_fetch_level,
        )
        .await?)
    }

    pub async fn get_contacts(
        pool: &sqlx::PgPool,
        id: Uuid,
        fetch_level: Option<FetchLevel>,
    ) -> Result<Vec<Contact>, sqlx::Error> {
        let fetch_level = match fetch_level {
            Some(fetch_level) => fetch_level,
            None => FetchLevel::IdOnly,
        };

        let res = sqlx::query!(
            r#"
            SELECT contact_id
            FROM club_contacts
            WHERE club_id = $1
            "#,
            id
        )
        .fetch_all(pool)
        .await?;

        Ok(Contact::get_from_ids(
            pool,
            res.iter().map(|r| r.contact_id).collect(),
            fetch_level,
        )
        .await?)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IdOnlyClub {
    pub id: Uuid,
}

impl IdOnlyClub {
    pub async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<IdOnlyClub, sqlx::Error> {
        let res = ClubTable::get_by_id(pool, id).await?;

        Ok(IdOnlyClub { id: res.id })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CompactClub {
    pub id: Uuid,
    pub name: MultiLangString,
    pub description: Option<MultiLangString>,
    pub logo_url: Option<String>,
    pub house: Option<ActivityDayHouse>,
    pub map_location: Option<u32>,
}

impl CompactClub {
    pub async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<CompactClub, sqlx::Error> {
        let res = ClubTable::get_by_id(pool, id).await?;

        Ok(CompactClub {
            id: res.id,
            name: MultiLangString {
                th: res.name_th,
                en: res.name_en,
            },
            description: match (res.description_th, res.description_en) {
                (Some(th), Some(en)) => Some(MultiLangString { th, en: Some(en) }),
                _ => None,
            },
            logo_url: res.logo_url,
            house: res.house,
            map_location: res.map_location.map(|l| l as u32),
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DefaultClub {
    pub id: Uuid,
    pub name: MultiLangString,
    pub description: Option<MultiLangString>,
    pub logo_url: Option<String>,
    pub staffs: Vec<Student>,
    pub members: Vec<Student>,
    // pub advisors: Vec<Teacher>,
    pub background_color: Option<String>,
    pub accent_color: Option<String>,
    pub contacts: Vec<Contact>,
    pub main_room: Option<String>,
    pub house: Option<ActivityDayHouse>,
    pub map_location: Option<u32>,
}

impl DefaultClub {
    pub async fn get_by_id(
        pool: &sqlx::PgPool,
        id: Uuid,
        descendant_fetch_level: Option<FetchLevel>,
    ) -> Result<DefaultClub, sqlx::Error> {
        let res = ClubTable::get_by_id(pool, id).await?;

        let members =
            ClubTable::get_members(pool, id, None, descendant_fetch_level.clone(), None).await?;
        let staffs =
            ClubTable::get_staffs(pool, id, None, descendant_fetch_level.clone(), None).await?;
        let contacts = ClubTable::get_contacts(pool, id, descendant_fetch_level).await?;

        Ok(DefaultClub {
            id: res.id,
            name: MultiLangString {
                th: res.name_th,
                en: res.name_en,
            },
            description: match (res.description_th, res.description_en) {
                (Some(th), Some(en)) => Some(MultiLangString { th, en: Some(en) }),
                _ => None,
            },
            logo_url: res.logo_url,
            staffs,
            members,
            background_color: res.background_color,
            accent_color: res.accent_color,
            contacts,
            main_room: res.main_room,
            house: res.house,
            map_location: res.map_location.map(|l| l as u32),
        })
    }
}

#[derive(Deserialize, Debug)]
pub enum Club {
    Default(DefaultClub),
    Compact(CompactClub),
    IdOnly(IdOnlyClub),
}

impl Club {
    pub async fn get_by_id(
        pool: &sqlx::PgPool,
        id: Uuid,
        fetch_level: Option<FetchLevel>,
        descendant_fetch_level: Option<FetchLevel>,
    ) -> Result<Club, sqlx::Error> {
        let fetch_level = match fetch_level {
            Some(fetch_level) => fetch_level,
            None => FetchLevel::Default,
        };

        match fetch_level {
            FetchLevel::IdOnly => Ok(Club::IdOnly(IdOnlyClub::get_by_id(pool, id).await?)),
            FetchLevel::Compact => Ok(Club::Compact(CompactClub::get_by_id(pool, id).await?)),
            FetchLevel::Default => Ok(Club::Default(
                DefaultClub::get_by_id(pool, id, descendant_fetch_level).await?,
            )),
        }
    }
}
