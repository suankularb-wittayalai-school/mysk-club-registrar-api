use actix_web::error::{ErrorNotFound, ErrorUnauthorized};
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{web, FromRequest, HttpRequest};

use chrono::{DateTime, NaiveDate, Utc};
use futures::Future as FutureTrait;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};

use utoipa::openapi::schema;
use utoipa::{IntoParams, ToSchema};

use std::pin::Pin;
use std::vec;

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

    async fn get_from_ids(pool: &Pool<Postgres>, ids: Vec<i64>) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            PeopleTable,
            r#"
            SELECT id, created_at, prefix_th, prefix_en, first_name_th, first_name_en, last_name_th, last_name_en, middle_name_th, middle_name_en, birthdate, citizen_id, contacts, profile, nickname_th, nickname_en, pants_size
            FROM people
            WHERE id = ANY($1)
            "#,
            &ids
        )
        .fetch_all(pool)
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

    async fn get_from_ids(pool: &Pool<Postgres>, ids: Vec<i64>) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            StudentTable,
            r#"
            SELECT id, created_at, std_id, person
            FROM student
            WHERE id = ANY($1)
            "#,
            &ids
        )
        .fetch_all(pool)
        .await
    }
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
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

    pub async fn get_from_ids(
        pool: &Pool<Postgres>,
        ids: Vec<i64>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let students = StudentTable::get_from_ids(pool, ids).await?;

        Ok(students
            .into_iter()
            .map(|x| Self { id: x.id as u32 })
            .collect())
    }
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct CompactStudent {
    pub id: u32,
    pub prefix: MultiLangString,
    pub first_name: MultiLangString,
    pub last_name: MultiLangString,
    // pub middle_name: Option<MultiLangString>,
    pub profile_url: Option<String>,
    #[schema(value_type = String, example = "2022-01-22")]
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

    pub async fn get_from_ids(
        pool: &Pool<Postgres>,
        ids: Vec<i64>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let students = StudentTable::get_from_ids(pool, ids).await?;
        let people = PeopleTable::get_from_ids(
            pool,
            students.iter().map(|x| x.person).collect::<Vec<i64>>(),
        )
        .await?;

        Ok(students
            .into_iter()
            .zip(people.into_iter())
            .map(|(student, person)| Self {
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
            .collect())
    }
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct DefaultStudent {
    pub id: u32,
    pub prefix: MultiLangString,
    pub first_name: MultiLangString,
    pub last_name: MultiLangString,
    pub middle_name: Option<MultiLangString>,
    pub profile_url: Option<String>,
    #[schema(value_type = String, example = "2022-01-22")]
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
    pub async fn get_by_id(
        pool: &Pool<Postgres>,
        id: u32,
        descendant_fetch_level: Option<FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        let descendant_fetch_level = descendant_fetch_level.unwrap_or(FetchLevel::IdOnly);

        let student = StudentTable::get_by_id(pool, id as i64).await?;
        let person = PeopleTable::get_by_id(pool, student.person).await?;
        let user = User::from_student_id(student.id as u32, pool).await?;

        let classroom = Classroom::get_by_student_id(
            pool,
            student.id as u32,
            None,
            descendant_fetch_level.clone(),
            Some(FetchLevel::IdOnly),
        )
        .await?;

        let class_number = match &classroom {
            Some(_) => Classroom::get_class_no_by_student_id(pool, student.id as u32, None).await?,
            None => None,
        };

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
            contacts: Contact::get_from_ids(
                pool,
                person.contacts.unwrap_or(vec![]),
                descendant_fetch_level,
            )
            .await?,
            class: classroom, // TODO: get class based on descendant_fetch_level
            class_number: class_number,
            profile_url: person.profile,
            birthdate: person.birthdate,
            student_id: student.std_id.parse::<u32>().unwrap(),
            user: user,
        })
    }

    pub async fn get_from_ids(
        pool: &Pool<Postgres>,
        ids: Vec<i64>,
        descendant_fetch_level: Option<FetchLevel>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let descendant_fetch_level = descendant_fetch_level.unwrap_or(FetchLevel::IdOnly);

        let students = StudentTable::get_from_ids(pool, ids).await?;
        let people = PeopleTable::get_from_ids(
            pool,
            students.iter().map(|x| x.person).collect::<Vec<i64>>(),
        )
        .await?;
        let users = User::from_student_ids(
            students.iter().map(|x| x.id as u32).collect::<Vec<u32>>(),
            pool,
        )
        .await?;

        // map student to user and person in a way that can use zip with async/await to fetch other data
        let students = students
            .into_iter()
            .zip(users.into_iter())
            .zip(people.into_iter())
            .map(|((student, user), person)| (student, user, person))
            .collect::<Vec<_>>();
        let mut students_to_return = vec![];

        for student in students {
            let (student, user, person) = student;
            // let classroom = Classroom::get_by_student_id(
            //     pool,
            //     student.id as u32,
            //     None,
            //     descendant_fetch_level.clone(),
            //     Some(descendant_fetch_level.clone()),
            // )
            // .await?;

            // let classroom_number = match &classroom {
            //     Some(_) => {
            //         Classroom::get_class_no_by_student_id(pool, student.id as u32, None).await?
            //     }
            //     None => None,
            // };

            students_to_return.push(Self {
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
                contacts: Contact::get_from_ids(
                    pool,
                    person.contacts.unwrap_or(vec![]),
                    descendant_fetch_level.clone(),
                )
                .await?,
                class: None,
                class_number: None,
                profile_url: person.profile,
                birthdate: person.birthdate,
                student_id: student.std_id.parse::<u32>().unwrap(),
                user: user,
            });
        }
        Ok(students_to_return)
    }
}

#[derive(Deserialize, Debug, ToSchema)]
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
        descendant_fetch_level: Option<FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        match level {
            Some(FetchLevel::IdOnly) => Ok(Self::IdOnly(IdOnlyStudent::get_by_id(pool, id).await?)),
            Some(FetchLevel::Compact) => {
                Ok(Self::Compact(CompactStudent::get_by_id(pool, id).await?))
            }
            Some(FetchLevel::Default) | None => Ok(Self::Default(
                DefaultStudent::get_by_id(pool, id, descendant_fetch_level).await?,
            )),
        }
    }

    pub async fn get_from_ids(
        pool: &Pool<Postgres>,
        ids: Vec<i64>,
        level: Option<FetchLevel>,
        descendant_fetch_level: Option<FetchLevel>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        match level {
            Some(FetchLevel::IdOnly) => {
                let res = IdOnlyStudent::get_from_ids(pool, ids).await?;
                let mut students = vec![];
                for student in res {
                    students.push(Self::IdOnly(student));
                }
                Ok(students)
            }
            Some(FetchLevel::Compact) => {
                let res = CompactStudent::get_from_ids(pool, ids).await?;
                let mut students = vec![];
                for student in res {
                    students.push(Self::Compact(student));
                }
                Ok(students)
            }
            Some(FetchLevel::Default) | None => {
                let res = DefaultStudent::get_from_ids(pool, ids, descendant_fetch_level).await?;
                let mut students = vec![];
                for student in res {
                    students.push(Self::Default(student));
                }
                Ok(students)
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
                        Err(_) => Err(ErrorNotFound(ErrorResponseType::new(
                            ErrorType {
                                id: "404".to_string(),
                                detail: "Field missing".to_string(),
                                code: 404,
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
