use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};

use crate::structs::{contacts::Contact, student::Student};
use crate::utils::date::get_current_academic_year;
// use crate::utils::logger;

use super::common::FetchLevel;

#[derive(FromRow, Debug)]
struct ClassroomTable {
    pub id: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub number: i64,
    pub year: i64,
    pub students: Vec<i64>,
    pub advisors: Vec<i64>,
    pub contacts: Vec<i64>,
    pub subjects: Vec<i64>,
    pub no_list: Vec<i64>,
}

impl ClassroomTable {
    pub async fn get_by_id(pool: &Pool<Postgres>, id: u32) -> Result<ClassroomTable, sqlx::Error> {
        sqlx::query_as!(
            ClassroomTable,
            r#"
            SELECT * FROM classroom WHERE id = $1
            "#,
            id as i64
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_student_id(
        pool: &Pool<Postgres>,
        id: u32,
        year: Option<u32>,
    ) -> Result<Option<ClassroomTable>, sqlx::Error> {
        let year = match year {
            Some(year) => year,
            None => get_current_academic_year(),
        };

        let classroom = sqlx::query_as!(
            ClassroomTable,
            r#"
            SELECT * FROM classroom WHERE $1 = ANY(students) AND year = $2
            "#,
            id as i64,
            year as i64
        )
        .fetch_optional(pool)
        .await;

        match classroom {
            Ok(classroom) => Ok(classroom),
            Err(e) => {
                // logger::log(logger::Header::ERROR, &format!("{:?}", e));
                Err(e)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdOnlyClassroom {
    pub id: u32,
}

impl IdOnlyClassroom {
    pub async fn get_by_id(pool: &Pool<Postgres>, id: u32) -> Result<IdOnlyClassroom, sqlx::Error> {
        let classroom = ClassroomTable::get_by_id(pool, id).await?;
        Ok(IdOnlyClassroom {
            id: classroom.id as u32,
        })
    }
    pub async fn get_by_student_id(
        pool: &Pool<Postgres>,
        id: u32,
        year: Option<u32>,
    ) -> Result<Option<IdOnlyClassroom>, sqlx::Error> {
        let classrooms = ClassroomTable::get_by_student_id(pool, id, year).await?;

        match classrooms {
            Some(classroom) => Ok(Some(IdOnlyClassroom {
                id: classroom.id as u32,
            })),
            None => Ok(None),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompactClassroom {
    pub id: u32,
    pub number: u32,
    pub room: String,
}

impl CompactClassroom {
    pub async fn get_by_id(
        pool: &Pool<Postgres>,
        id: u32,
    ) -> Result<CompactClassroom, sqlx::Error> {
        let classroom = ClassroomTable::get_by_id(pool, id).await?;
        Ok(CompactClassroom {
            id: classroom.id as u32,
            number: classroom.number as u32,
            room: "TODO".to_string(),
        })
    }
    pub async fn get_by_student_id(
        pool: &Pool<Postgres>,
        id: u32,
        year: Option<u32>,
    ) -> Result<Option<CompactClassroom>, sqlx::Error> {
        let classroom = ClassroomTable::get_by_student_id(pool, id, year).await?;

        match classroom {
            Some(classroom) => Ok(Some(CompactClassroom {
                id: classroom.id as u32,
                number: classroom.number as u32,
                room: "TODO".to_string(),
            })),
            None => Ok(None),
        }
    }
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

impl DefaultClassroom {
    pub async fn get_by_id(
        pool: &Pool<Postgres>,
        id: u32,
        descendant_fetch_level: Option<FetchLevel>,
    ) -> Result<DefaultClassroom, sqlx::Error> {
        // let descendant_fetch_level = descendant_fetch_level.unwrap_or(FetchLevel::IdOnly);

        let classroom = ClassroomTable::get_by_id(pool, id).await?;
        let contacts = Contact::get_from_ids(
            pool,
            classroom.contacts,
            descendant_fetch_level.clone().unwrap_or(FetchLevel::IdOnly),
        )
        .await?;

        Ok(DefaultClassroom {
            id: classroom.id as u32,
            number: classroom.number as u32,
            room: "TODO".to_string(),
            students: Student::get_from_ids(
                pool,
                classroom.students,
                descendant_fetch_level.clone(),
                descendant_fetch_level.clone(),
            )
            .await?,
            contacts: contacts,
            year: classroom.year as u32,
        })
    }

    pub async fn get_by_student_id(
        pool: &Pool<Postgres>,
        id: u32,
        year: Option<u32>,
        descendant_fetch_level: Option<FetchLevel>,
    ) -> Result<Option<DefaultClassroom>, sqlx::Error> {
        let year = match year {
            Some(year) => year,
            None => get_current_academic_year(),
        };

        let classroom = ClassroomTable::get_by_student_id(pool, id, Some(year)).await?;

        match classroom {
            Some(classroom) => {
                let contacts = Contact::get_from_ids(
                    pool,
                    classroom.contacts,
                    descendant_fetch_level.clone().unwrap_or(FetchLevel::IdOnly),
                )
                .await?;

                Ok(Some(DefaultClassroom {
                    id: classroom.id as u32,
                    number: classroom.number as u32,
                    room: "TODO".to_string(),
                    students: Student::get_from_ids(
                        pool,
                        classroom.students,
                        descendant_fetch_level.clone(),
                        descendant_fetch_level.clone(),
                    )
                    .await?,
                    contacts: contacts,
                    year: classroom.year as u32,
                }))
            }
            None => Ok(None),
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum Classroom {
    Default(DefaultClassroom),
    IdOnly(IdOnlyClassroom),
    Compact(CompactClassroom),
}

impl Classroom {
    pub async fn get_by_id(
        pool: &Pool<Postgres>,
        id: u32,
        fetch_level: FetchLevel,
        descendant_fetch_level: Option<FetchLevel>,
    ) -> Result<Classroom, sqlx::Error> {
        match fetch_level {
            FetchLevel::Default => Ok(Classroom::Default(
                DefaultClassroom::get_by_id(pool, id, descendant_fetch_level).await?,
            )),
            FetchLevel::Compact => Ok(Classroom::Compact(
                CompactClassroom::get_by_id(pool, id).await?,
            )),
            FetchLevel::IdOnly => Ok(Classroom::IdOnly(
                IdOnlyClassroom::get_by_id(pool, id).await?,
            )),
        }
    }

    pub async fn get_by_student_id(
        pool: &Pool<Postgres>,
        id: u32,
        year: Option<u32>,
        fetch_level: FetchLevel,
        descendant_fetch_level: Option<FetchLevel>,
    ) -> Result<Option<Classroom>, sqlx::Error> {
        match fetch_level {
            FetchLevel::Default => Ok(
                match DefaultClassroom::get_by_student_id(pool, id, year, descendant_fetch_level)
                    .await?
                {
                    Some(classroom) => Some(Classroom::Default(classroom)),
                    None => None,
                },
            ),
            FetchLevel::Compact => Ok(
                match CompactClassroom::get_by_student_id(pool, id, year).await? {
                    Some(classroom) => Some(Classroom::Compact(classroom)),
                    None => None,
                },
            ),
            FetchLevel::IdOnly => Ok(
                match IdOnlyClassroom::get_by_student_id(pool, id, year).await? {
                    Some(classroom) => Some(Classroom::IdOnly(classroom)),
                    None => None,
                },
            ),
        }
    }

    pub async fn get_class_no_by_student_id(
        pool: &Pool<Postgres>,
        id: u32,
        year: Option<u32>,
    ) -> Result<Option<u32>, sqlx::Error> {
        let year = match year {
            Some(year) => year,
            None => get_current_academic_year(),
        };

        let classroom = ClassroomTable::get_by_student_id(pool, id, Some(year)).await?;

        match classroom {
            Some(classroom) => {
                let no_list = classroom.no_list;
                let class_no = no_list.iter().position(|&x| x == id as i64).unwrap_or(0);
                Ok(Some(class_no as u32 + 1))
            }
            None => Ok(None),
        }
    }
}

impl Serialize for Classroom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Classroom::Default(classroom) => classroom.serialize(serializer),
            Classroom::Compact(classroom) => classroom.serialize(serializer),
            Classroom::IdOnly(classroom) => classroom.serialize(serializer),
        }
    }
}
