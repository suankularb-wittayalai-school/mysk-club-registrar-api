use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use super::{
    clubs::{Club, SubmissionStatus},
    common::{FetchLevel, PaginationConfig, RequestType},
    student::Student,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryableClubRequest {
    pub id: Option<Uuid>,
    pub club_id: Option<Uuid>,
    pub student_id: Option<i64>,
    pub year: Option<i64>,
    pub membership_status: Option<SubmissionStatus>,
    // pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatableClubRequest {
    pub club_id: Uuid,
    pub student_id: i64,
    pub year: Option<i64>,
    pub membership_status: SubmissionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatableClubRequest {
    pub club_id: Option<Uuid>,
    pub student_id: Option<i64>,
    pub year: Option<i64>,
    pub membership_status: Option<SubmissionStatus>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ClubRequestSortableField {
    Id,
    ClubId,
    StudentId,
    Year,
    MembershipStatus,
    CreatedAt,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ClubRequestTable {
    pub id: Uuid,
    pub club_id: Uuid,
    pub student_id: i64,
    pub year: i64,
    pub membership_status: SubmissionStatus,
    pub created_at: Option<DateTime<Utc>>,
}

impl ClubRequestTable {
    pub async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Self, sqlx::Error> {
        Ok(sqlx::query_as!(
            Self,
            r#"
                SELECT id, club_id, student_id, year, membership_status as "membership_status: _", created_at FROM club_members WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?)
    }

    fn construct_query_string(
        request_params: &RequestType<Self, QueryableClubRequest, ClubRequestSortableField>,
    ) -> (
        String,
        Vec<&Uuid>,
        Vec<&i64>,
        Vec<&SubmissionStatus>,
        Vec<u32>,
    ) {
        let query = r#"
            SELECT id, club_id, student_id, year, membership_status as "membership_status: _", created_at FROM club_members
        "#;

        let mut query = String::from(query);
        let mut query_counts = 1;

        let mut uuid_params = Vec::new();
        let mut i64_params = Vec::new();
        let mut submission_status_params = Vec::new();
        let mut pagination_params = Vec::new();

        if let Some(filter) = &request_params.filter {
            if let Some(data) = &filter.data {
                if let Some(id) = &data.id {
                    uuid_params.push(id);

                    if query.contains("WHERE") {
                        query.push_str(&format!(" AND id = ${query_counts}$"));
                    } else {
                        query.push_str(&format!(" WHERE id = ${query_counts}$"));
                    }

                    query_counts += 1;
                }

                if let Some(club_id) = &data.club_id {
                    uuid_params.push(club_id);

                    if query.contains("WHERE") {
                        query.push_str(&format!(" AND club_id = ${query_counts}$"));
                    } else {
                        query.push_str(&format!(" WHERE club_id = ${query_counts}$"));
                    }

                    query_counts += 1;
                }

                if let Some(student_id) = &data.student_id {
                    i64_params.push(student_id);

                    if query.contains("WHERE") {
                        query.push_str(&format!(" AND student_id = ${query_counts}$"));
                    } else {
                        query.push_str(&format!(" WHERE student_id = ${query_counts}$"));
                    }

                    query_counts += 1;
                }

                if let Some(year) = &data.year {
                    i64_params.push(year);

                    if query.contains("WHERE") {
                        query.push_str(&format!(" AND year = ${query_counts}$"));
                    } else {
                        query.push_str(&format!(" WHERE year = ${query_counts}$"));
                    }

                    query_counts += 1;
                }

                if let Some(membership_status) = &data.membership_status {
                    submission_status_params.push(membership_status);

                    if query.contains("WHERE") {
                        query.push_str(&format!(" AND membership_status = ${query_counts}$"));
                    } else {
                        query.push_str(&format!(" WHERE membership_status = ${query_counts}$"));
                    }

                    query_counts += 1;
                }
            }
        }

        if let Some(sort) = &request_params.sorting {
            let sort_vec = match sort.by.clone() {
                Some(sort) => sort,
                // return vector of id if sort.by is None
                None => vec![ClubRequestSortableField::Id],
            };

            if !sort_vec.is_empty() {
                query.push_str(" ORDER BY");

                let mut first = true;

                for sort in sort_vec {
                    if !first {
                        query.push_str(",");
                    }

                    match sort {
                        ClubRequestSortableField::Id => query.push_str(" id"),
                        ClubRequestSortableField::ClubId => query.push_str(" club_id"),
                        ClubRequestSortableField::StudentId => query.push_str(" student_id"),
                        ClubRequestSortableField::Year => query.push_str(" year"),
                        ClubRequestSortableField::MembershipStatus => {
                            query.push_str(" membership_status")
                        }
                        ClubRequestSortableField::CreatedAt => query.push_str(" created_at"),
                    }

                    first = false;
                }
            }
        }

        let pagination = match &request_params.pagination {
            Some(pagination) => pagination,
            None => &PaginationConfig {
                size: Some(50),
                p: 1,
            },
        };

        let size = pagination.size.unwrap_or(50);
        let page = pagination.p;

        let next_count = query_counts + 1;

        query.push_str(&format!(" LIMIT ${query_counts} OFFSET ${next_count}",));

        pagination_params.push(size);
        pagination_params.push((page - 1) * size);

        (
            query,
            uuid_params,
            i64_params,
            submission_status_params,
            pagination_params,
        )
    }

    pub async fn query(
        pool: &sqlx::PgPool,
        request_params: &RequestType<ClubRequest, QueryableClubRequest, ClubRequestSortableField>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let (query, uuid_params, i64_params, submission_status_params, pagination_params) =
            Self::construct_query_string(request_params);

        let mut res = sqlx::query_as::<_, Self>(&query);

        for uuid_param in uuid_params {
            res = res.bind(uuid_param);
        }

        for i64_param in i64_params {
            res = res.bind(i64_param);
        }

        for submission_status_param in submission_status_params {
            res = res.bind(submission_status_param);
        }

        for pagination_param in pagination_params {
            res = res.bind(pagination_param as i64);
        }

        Ok(res.fetch_all(pool).await?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyClubRequest {
    pub id: Uuid,
}

impl IdOnlyClubRequest {
    fn from_table(table: ClubRequestTable) -> Self {
        Self { id: table.id }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultClubRequest {
    pub id: Uuid,
    pub club: Club,
    pub student: Student,
    pub year: i64,
    pub membership_status: SubmissionStatus,
}

impl DefaultClubRequest {
    async fn from_table(
        pool: &sqlx::PgPool,
        table: ClubRequestTable,
        descendant_fetch_level: Option<FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        let club = Club::get_by_id(
            pool,
            table.club_id,
            descendant_fetch_level.clone(),
            Some(FetchLevel::IdOnly),
        )
        .await?;
        let student = Student::get_by_id(
            pool,
            table.student_id as u32,
            descendant_fetch_level.clone(),
            Some(FetchLevel::IdOnly),
        )
        .await?;

        Ok(Self {
            id: table.id,
            club,
            student,
            year: table.year,
            membership_status: table.membership_status,
        })
    }
}

#[derive(Debug, Deserialize)]
pub enum ClubRequest {
    Default(DefaultClubRequest),
    Compact(DefaultClubRequest),
    IdOnly(IdOnlyClubRequest),
}

impl ClubRequest {
    async fn from_table(
        pool: &sqlx::PgPool,
        table: ClubRequestTable,
        descendant_fetch_level: Option<FetchLevel>,
        fetch_level: Option<FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        match fetch_level {
            Some(FetchLevel::Default) => Ok(Self::Default(
                DefaultClubRequest::from_table(pool, table, descendant_fetch_level).await?,
            )),
            Some(FetchLevel::Compact) => Ok(Self::Compact(
                DefaultClubRequest::from_table(pool, table, descendant_fetch_level).await?,
            )),
            Some(FetchLevel::IdOnly) => Ok(Self::IdOnly(IdOnlyClubRequest::from_table(table))),
            None => Ok(Self::IdOnly(IdOnlyClubRequest::from_table(table))),
        }
    }

    pub async fn get_by_id(
        pool: &sqlx::PgPool,
        id: Uuid,
        descendent_fetch_level: Option<FetchLevel>,
        fetch_level: Option<FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        let table = sqlx::query_as::<_, ClubRequestTable>(
            r#"
            SELECT * FROM club_requests WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Self::from_table(pool, table, descendent_fetch_level, fetch_level).await
    }

    pub async fn query(
        pool: &sqlx::PgPool,
        request_params: &RequestType<Self, QueryableClubRequest, ClubRequestSortableField>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let fetch_level = match &request_params.fetch_level {
            Some(fetch_level) => fetch_level,
            None => &FetchLevel::Default,
        };

        let join_requests = match ClubRequestTable::query(pool, request_params).await {
            Ok(join_requests) => join_requests,
            Err(e) => {
                // error!("Error querying club requests: {}", e);
                // Err(e)
                return Err(e);
            }
        };

        match fetch_level {
            FetchLevel::Default => {
                let res = join_requests
                    .into_iter()
                    .map(|join_request| {
                        Self::from_table(
                            pool,
                            join_request,
                            request_params.descendant_fetch_level.clone(),
                            Some(*fetch_level),
                        )
                    })
                    .collect::<Result<Vec<Self>, sqlx::Error>>()
                    .await?;
                Ok(res)
            }
            FetchLevel::Compact => {
                let res = join_requests
                    .into_iter()
                    .map(|join_request| {
                        Self::from_table(
                            pool,
                            join_request,
                            request_params.descendant_fetch_level.clone(),
                            Some(*fetch_level),
                        )
                    })
                    .collect::<Result<Vec<Self>, sqlx::Error>>()
                    .await?;
                Ok(res)
            }
            FetchLevel::IdOnly => {
                let res = join_requests
                    .into_iter()
                    .map(|join_request| {
                        Self::from_table(
                            pool,
                            join_request,
                            request_params.descendant_fetch_level.clone(),
                            Some(*fetch_level),
                        )
                    })
                    .collect::<Result<Vec<Self>, sqlx::Error>>()
                    .await?;
                Ok(res)
            }
        }
    }
}

impl Serialize for ClubRequest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ClubRequest::Default(default) => default.serialize(serializer),
            ClubRequest::Compact(compact) => compact.serialize(serializer),
            ClubRequest::IdOnly(id_only) => id_only.serialize(serializer),
        }
    }
}
