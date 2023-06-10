use actix_web::error::{ErrorNotFound, ErrorUnauthorized};
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{web, FromRequest, HttpRequest};

use chrono::{DateTime, NaiveDate, Utc};
use futures::Future as FutureTrait;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};

use std::pin::Pin;

use crate::structs::common::{ErrorResponseType, ErrorType};

use crate::AppState;

use crate::structs::{
    auth::User,
    classroom::Classroom,
    common::{FetchLevel, MultiLangString},
    contacts::Contact,
};

use super::auth::UserRoles;

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
enum ShirtSize {
    #[serde(rename = "S")]
    Small,
    #[serde(rename = "M")]
    Medium,
    #[serde(rename = "L")]
    Large,
    #[serde(rename = "XL")]
    ExtraLarge,
    #[serde(rename = "2XL")]
    ExtraExtraLarge,
    #[serde(rename = "3XL")]
    ExtraExtraExtraLarge,
    #[serde(rename = "4XL")]
    ExtraExtraExtraExtraLarge,
    #[serde(rename = "5XL")]
    ExtraExtraExtraExtraExtraLarge,
    #[serde(rename = "6XL")]
    ExtraExtraExtraExtraExtraExtraLarge,
}

#[derive(FromRow, Debug)]
struct PeopleTable {
    pub id: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub prefix_th: String,
    pub prefix_en: Option<String>,
    pub first_name_th: String,
    pub first_name_en: Option<String>,
    pub last_name_th: String,
    pub last_name_en: Option<String>,
    pub middle_name_th: Option<String>,
    pub middle_name_en: Option<String>,
    pub birthdate: NaiveDate,
    pub citizen_id: String,
    pub contacts: Option<Vec<i64>>,
    pub profile: Option<String>,
    pub nickname_th: Option<String>,
    pub nickname_en: Option<String>,
    pub pants_size: Option<String>,
    // pub shirt_size: Option<String>,
}

impl PeopleTable {
    async fn get_by_id(pool: &Pool<Postgres>, id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            PeopleTable,
            r#"
            SELECT id, created_at, prefix_th, prefix_en, first_name_th, first_name_en, last_name_th, last_name_en, middle_name_th, middle_name_en, birthdate, citizen_id, contacts, profile, nickname_th, nickname_en, pants_size
            FROM people
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
    }
}

#[derive(FromRow, Debug)]
struct StudentTable {
    pub id: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub std_id: String,
    pub person: i64,
}

impl StudentTable {
    async fn get_by_id(pool: &Pool<Postgres>, id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            StudentTable,
            r#"
            SELECT id, created_at, std_id, person
            FROM student
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IdOnlyStudent {
    pub id: u32,
}

impl IdOnlyStudent {
    pub async fn get_by_id(pool: &Pool<Postgres>, id: u32) -> Result<Self, sqlx::Error> {
        let student = StudentTable::get_by_id(pool, id as i64).await?;

        Ok(Self {
            id: student.id as u32,
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CompactStudent {
    pub id: u32,
    pub prefix: MultiLangString,
    pub first_name: MultiLangString,
    pub last_name: MultiLangString,
    // pub middle_name: Option<MultiLangString>,
    pub profile_url: Option<String>,
    pub birthdate: NaiveDate,
    // pub sex: Sex,
    // pub blood_group: Option<bloodType>,
    // contacts: Vec<Contact>,
    pub student_id: u32,
}

impl CompactStudent {
    pub async fn get_by_id(pool: &Pool<Postgres>, id: u32) -> Result<Self, sqlx::Error> {
        let student = StudentTable::get_by_id(pool, id as i64).await?;
        let person = PeopleTable::get_by_id(pool, student.person).await?;

        Ok(Self {
            id: student.id as u32,
            prefix: MultiLangString {
                th: person.prefix_th,
                en: person.prefix_en,
            },
            first_name: MultiLangString {
                th: person.first_name_th,
                en: person.first_name_en,
            },
            last_name: MultiLangString {
                th: person.last_name_th,
                en: person.last_name_en,
            },
            profile_url: person.profile,
            birthdate: person.birthdate,
            student_id: student.std_id.parse::<u32>().unwrap(),
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DefaultStudent {
    pub id: u32,
    pub prefix: MultiLangString,
    pub first_name: MultiLangString,
    pub last_name: MultiLangString,
    pub middle_name: Option<MultiLangString>,
    pub profile_url: Option<String>,
    pub birthdate: NaiveDate,
    // pub sex: Sex,
    // pub blood_group: Option<BloodType>,
    pub contacts: Vec<Contact>,
    pub student_id: u32,
    pub class: Option<Classroom>,
    pub class_number: Option<u32>,
    pub user: User,
}

impl DefaultStudent {
    pub async fn get_by_id(pool: &Pool<Postgres>, id: u32) -> Result<Self, sqlx::Error> {
        let student = StudentTable::get_by_id(pool, id as i64).await?;
        let person = PeopleTable::get_by_id(pool, student.person).await?;
        let user = User::from_student_id(student.id as u32, pool).await?;

        Ok(Self {
            id: student.id as u32,
            prefix: MultiLangString {
                th: person.prefix_th,
                en: person.prefix_en,
            },
            first_name: MultiLangString {
                th: person.first_name_th,
                en: person.first_name_en,
            },
            middle_name: match (person.middle_name_th, person.middle_name_en) {
                (Some(th), Some(en)) => Some(MultiLangString { th, en: Some(en) }),
                _ => None,
            },
            last_name: MultiLangString {
                th: person.last_name_th,
                en: person.last_name_en,
            },
            contacts: vec![], // TODO: get contacts based on decendent_fetch_level
            class: None,      // TODO: get class based on decendent_fetch_level
            class_number: None,
            profile_url: person.profile,
            birthdate: person.birthdate,
            student_id: student.std_id.parse::<u32>().unwrap(),
            user: user,
        })
    }
}

#[derive(Deserialize, Debug)]
pub enum Student {
    Default(DefaultStudent),
    IdOnly(IdOnlyStudent),
    Compact(CompactStudent),
}

impl Serialize for Student {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            Student::Default(student) => student.serialize(serializer),
            Student::IdOnly(student) => student.serialize(serializer),
            Student::Compact(student) => student.serialize(serializer),
        }
    }
}

impl Student {
    pub async fn get_by_id(
        pool: &Pool<Postgres>,
        id: u32,
        level: Option<FetchLevel>,
        decendent_fetch_level: Option<FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        match level {
            Some(FetchLevel::IdOnly) => Ok(Self::IdOnly(IdOnlyStudent::get_by_id(pool, id).await?)),
            Some(FetchLevel::Compact) => {
                Ok(Self::Compact(CompactStudent::get_by_id(pool, id).await?))
            }
            Some(FetchLevel::Default) | None => {
                Ok(Self::Default(DefaultStudent::get_by_id(pool, id).await?))
            }
        }
    }
}

impl FromRequest for Student {
    type Error = ActixWebError;
    type Future = Pin<Box<dyn FutureTrait<Output = Result<Self, Self::Error>>>>;
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let pool = req.app_data::<web::Data<AppState>>().unwrap().db.clone();

        // run normal user auth
        let fut = User::from_request(req, _payload);

        // then check if user is student
        Box::pin(async move {
            let user = fut.await?;

            // check role
            match user.role {
                UserRoles::Student => {
                    let student_id = match user.student {
                        Some(id) => id,
                        None => {
                            return Err(ErrorNotFound(ErrorResponseType::new(
                                ErrorType {
                                    id: "404".to_string(),
                                    detail: "Student ID not Found".to_string(),
                                    code: 401,
                                    error_type: "entity_not_found".to_string(),
                                    source: "".to_string(),
                                },
                                None,
                            )))
                        }
                    };

                    let student = Student::get_by_id(&pool, student_id, None, None).await;

                    match student {
                        Ok(student) => Ok(student),
                        Err(e) => Err(ErrorUnauthorized(ErrorResponseType::new(
                            ErrorType {
                                id: "401".to_string(),
                                detail: "User not a student".to_string(),
                                code: 401,
                                error_type: "invalid_permission".to_string(),
                                source: "".to_string(),
                            },
                            None,
                        ))),
                    }
                }
                _ => {
                    return Err(ErrorUnauthorized(ErrorResponseType::new(
                        ErrorType {
                            id: "401".to_string(),
                            detail: "User not a student".to_string(),
                            code: 401,
                            error_type: "invalid_permission".to_string(),
                            source: "".to_string(),
                        },
                        None,
                    )))
                }
            }
        })
    }
}
