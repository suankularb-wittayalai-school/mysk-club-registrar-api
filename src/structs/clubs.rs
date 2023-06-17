use std::vec;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, FromRow, Postgres, Type};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{structs::common::PaginationConfig, utils::date::get_current_academic_year};

use super::{
    common::{FetchLevel, FlexibleMultiLangString, MultiLangString, RequestType},
    contacts::Contact,
    student::Student,
};

#[derive(Debug, Clone, Copy, ToSchema)]
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

#[derive(Debug, Clone)]
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

impl Serialize for SubmissionStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for SubmissionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;

        match SubmissionStatus::from_string(&s) {
            Some(status) => Ok(status),
            None => Err(serde::de::Error::custom("Invalid submission status")),
        }
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

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryableClub {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub main_room: Option<String>,
    pub logo_url: Option<String>,
    pub background_color: Option<String>,
    pub accent_color: Option<String>,
    pub house: Option<ActivityDayHouse>,
    pub map_location: Option<i64>,
    pub staffs: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatableClubTable {
    pub name_th: Option<String>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatableClub {
    pub name: Option<MultiLangString>,
    pub description: Option<FlexibleMultiLangString>,
    pub main_room: Option<String>,
    pub logo_url: Option<String>,
    pub background_color: Option<String>,
    pub accent_color: Option<String>,
    pub house: Option<ActivityDayHouse>,
    pub map_location: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ClubSortableField {
    Id,
    CreatedAt,
    NameTh,
    NameEn,
    DescriptionTh,
    DescriptionEn,
    MainRoom,
    LogoUrl,
    BackgroundColor,
    AccentColor,
    House,
    MapLocation,
}

#[derive(FromRow, Clone)]
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

    // A function to construct query string from request params
    // the function will return a tuple of (query string, query params)
    fn construct_query_string(
        request_params: &RequestType<Club, QueryableClub, ClubSortableField>,
    ) -> (String, Vec<String>, Vec<ActivityDayHouse>, Vec<i64>) {
        let request = request_params;

        let query_clause = r#"
            SELECT clubs.id, clubs.created_at, name_th, name_en, description_th, description_en, main_room, logo_url, background_color, accent_color, house, map_location
            FROM clubs INNER JOIN organizations ON clubs.organization_id = organizations.id
            "#;

        let mut query_counts = 1;

        let mut query = String::from(query_clause);
        let mut string_params = Vec::new();
        let mut house_params = Vec::new();
        let mut int_params = Vec::new();

        if let Some(filter) = &request.filter {
            if let Some(q) = &filter.q {
                query.push_str(&format!("WHERE name_th ILIKE ${query_counts} OR name_en ILIKE ${query_counts} OR description_th ILIKE ${query_counts} OR description_en ILIKE ${query_counts} OR main_room ILIKE ${query_counts}"));
                string_params.push(format!("%{}%", q));
                query_counts += 1;
            }

            if let Some(data) = &filter.data {
                if let Some(name) = &data.name {
                    query.push_str(&format!(
                        "WHERE (name_th ILIKE ${query_counts} OR name_en ILIKE ${query_counts})"
                    ));
                    string_params.push(format!("%{}%", name));
                    query_counts += 1;
                }

                if let Some(description) = &data.description {
                    if query.contains("WHERE") {
                        query.push_str(&format!(
                            " AND (description_th ILIKE ${query_counts} OR description_en ILIKE ${query_counts})"
                        ));
                    } else {
                        query.push_str(&format!(
                            "WHERE (description_th ILIKE ${query_counts} OR description_en ILIKE ${query_counts})"
                        ));
                    }
                    string_params.push(format!("%{}%", description));
                    query_counts += 1;
                }

                if let Some(main_room) = &data.main_room {
                    if query.contains("WHERE") {
                        query.push_str(&format!(" AND main_room ILIKE ${query_counts}"));
                    } else {
                        query.push_str(&format!("WHERE main_room ILIKE ${query_counts}"));
                    }

                    string_params.push(format!("%{}%", main_room));
                    query_counts += 1;
                }

                if let Some(logo_url) = &data.logo_url {
                    if query.contains("WHERE") {
                        query.push_str(&format!(" AND logo_url ILIKE ${query_counts}"));
                    } else {
                        query.push_str(&format!("WHERE logo_url ILIKE ${query_counts}"));
                    }
                    string_params.push(format!("%{}%", logo_url));
                    query_counts += 1;
                }

                if let Some(background_color) = &data.background_color {
                    if query.contains("WHERE") {
                        query.push_str(&format!(" AND background_color ILIKE ${query_counts}"));
                    } else {
                        query.push_str(&format!("WHERE background_color ILIKE ${query_counts}"));
                    }

                    string_params.push(format!("%{}%", background_color));
                    query_counts += 1;
                }

                if let Some(accent_color) = &data.accent_color {
                    if query.contains("WHERE") {
                        query.push_str(&format!(" AND accent_color ILIKE ${query_counts}"));
                    } else {
                        query.push_str(&format!("WHERE accent_color ILIKE ${query_counts}"));
                    }

                    string_params.push(format!("%{}%", accent_color));
                    query_counts += 1;
                }

                if let Some(house) = &data.house {
                    if query.contains("WHERE") {
                        query.push_str(&format!(" AND house = ${query_counts}"));
                    } else {
                        query.push_str(&format!("WHERE house = ${query_counts}"));
                    }

                    house_params.push(*house);
                    query_counts += 1;
                }

                if let Some(map_location) = &data.map_location {
                    if query.contains("WHERE") {
                        query.push_str(&format!(" AND map_location = ${query_counts}"));
                    } else {
                        query.push_str(&format!("WHERE map_location = ${query_counts}"));
                    }

                    int_params.push(*map_location);
                    query_counts += 1;
                }

                // SELECT from club_staffs where club_id = $1 and student_id is in data.staff vector
                if let Some(staffs) = &data.staffs {
                    if query.contains("WHERE") {
                        query.push_str(&format!(" AND clubs.id IN (SELECT club_id FROM club_staffs WHERE student_id IN ("));
                    } else {
                        query.push_str(&format!("WHERE clubs.id IN (SELECT club_id FROM club_staffs WHERE student_id IN ("));
                    }

                    for (i, staff) in staffs.iter().enumerate() {
                        if i != 0 {
                            query.push_str(&format!(", ${}", query_counts));
                            int_params.push(*staff);
                            query_counts += 1;
                        } else {
                            query.push_str(&format!("${}", query_counts));
                            int_params.push(*staff);
                            query_counts += 1;
                        }
                    }
                    query.push_str("))");
                }
            }
        }

        // if sort is not empty, add ORDER BY clause and check the sort fields are valid
        if let Some(sort) = &request.sorting {
            let sort_vec = match sort.by.clone() {
                Some(sort) => sort,
                // return vector of id if sort.by is None
                None => vec![ClubSortableField::Id],
            };

            if !sort_vec.is_empty() {
                query.push_str(" ORDER BY");

                let mut first = true;
                for s in sort_vec {
                    if !first {
                        query.push_str(",");
                    }

                    match s {
                        ClubSortableField::Id => query.push_str(" clubs.id"),
                        ClubSortableField::CreatedAt => query.push_str(" clubs.created_at"),
                        ClubSortableField::NameTh => query.push_str(" name_th"),
                        ClubSortableField::NameEn => query.push_str(" name_en"),
                        ClubSortableField::DescriptionTh => query.push_str(" description_th"),
                        ClubSortableField::DescriptionEn => query.push_str(" description_en"),
                        ClubSortableField::MainRoom => query.push_str(" main_room"),
                        ClubSortableField::LogoUrl => query.push_str(" logo_url"),
                        ClubSortableField::BackgroundColor => query.push_str(" background_color"),
                        ClubSortableField::AccentColor => query.push_str(" accent_color"),
                        ClubSortableField::House => query.push_str(" house"),
                        ClubSortableField::MapLocation => query.push_str(" map_location"),
                    }

                    first = false;
                }

                // check if it is ascending or descending
                if sort.ascending {
                    query.push_str(" ASC");
                } else {
                    query.push_str(" DESC");
                }
            }
        }
        // do pagination by default with size = 50 and page = 1 if not specified
        let pagination = match &request.pagination {
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

        int_params.push(size as i64);

        int_params.push(((page - 1) * size) as i64);

        (query, string_params, house_params, int_params)
    }

    pub async fn update_by_id(
        pool: &sqlx::PgPool,
        id: Uuid,
        club: &UpdatableClubTable,
    ) -> Result<Self, sqlx::Error> {
        // construct 2 queries
        // 1. update clubs table
        // 2. update organizations table
        // while making sure that the queries have to be ran because it might leave a SET clause with no fields to update
        // make sure to use the same transaction
        // only the colors, house, and map_location are in the clubs table
        // the rest are in the organizations table

        let mut update_query = String::from("UPDATE clubs SET ");
        let mut query_counts = 1;

        let mut string_params = Vec::new();
        let mut house_params = Vec::new();
        let mut int_params = Vec::new();

        if let Some(background_color) = &club.background_color {
            update_query.push_str(&format!("background_color = ${}, ", query_counts));
            query_counts += 1;
            string_params.push(background_color);
        }

        if let Some(accent_color) = &club.accent_color {
            update_query.push_str(&format!("accent_color = ${}, ", query_counts));
            query_counts += 1;
            string_params.push(accent_color);
        }

        if let Some(house) = &club.house {
            update_query.push_str(&format!("house = ${}, ", query_counts));
            query_counts += 1;
            house_params.push(house);
        }

        if let Some(map_location) = &club.map_location {
            update_query.push_str(&format!("map_location = ${}, ", query_counts));
            query_counts += 1;
            int_params.push(map_location);
        }

        update_query.pop();
        update_query.pop();

        update_query.push_str(&format!(" WHERE id = ${query_counts}"));

        let mut update_organization_query = String::from("UPDATE organizations SET ");
        let mut query_counts = 1;

        let mut organization_string_params = Vec::new();

        if let Some(name_th) = &club.name_th {
            update_organization_query.push_str(&format!("name_th = ${}, ", query_counts));
            query_counts += 1;
            organization_string_params.push(name_th);
        }

        if let Some(name_en) = &club.name_en {
            update_organization_query.push_str(&format!("name_en = ${}, ", query_counts));
            query_counts += 1;
            organization_string_params.push(name_en);
        }

        if let Some(description_th) = &club.description_th {
            update_organization_query.push_str(&format!("description_th = ${}, ", query_counts));
            query_counts += 1;
            organization_string_params.push(description_th);
        }

        if let Some(description_en) = &club.description_en {
            update_organization_query.push_str(&format!("description_en = ${}, ", query_counts));
            query_counts += 1;
            organization_string_params.push(description_en);
        }

        if let Some(main_room) = &club.main_room {
            update_organization_query.push_str(&format!("main_room = ${}, ", query_counts));
            query_counts += 1;
            organization_string_params.push(main_room);
        }

        if let Some(logo_url) = &club.logo_url {
            update_organization_query.push_str(&format!("logo_url = ${}, ", query_counts));
            query_counts += 1;
            organization_string_params.push(logo_url);
        }

        update_organization_query.pop();
        update_organization_query.pop();

        update_organization_query.push_str(&format!(" WHERE id = ${query_counts}"));

        let mut transaction = pool.begin().await?;

        // dbg!(
        //     &update_query,
        //     &string_params,
        //     &house_params,
        //     &int_params,
        //     &update_organization_query,
        //     &organization_string_params
        // );

        // check if the query actually updated anything by checking if there is SET WHERE clause in the query
        // if there is SET WHERE directly after each other, then it means that there is no fields to update which mean to not run the query

        if !update_query.contains("SE WHERE") {
            let mut club_update_res = sqlx::query(&update_query);

            for param in string_params {
                club_update_res = club_update_res.bind(param);
            }

            for param in house_params {
                club_update_res = club_update_res.bind(param);
            }

            for param in int_params {
                club_update_res = club_update_res.bind(param);
            }

            club_update_res = club_update_res.bind(id);

            let _ = club_update_res.execute(&mut transaction).await?;

            // if club_update_res.rows_affected() == 0 {
            //     return Err(sqlx::Error::RowNotFound);
            // }
        }

        if !update_organization_query.contains("SE WHERE") {
            let organization_id = sqlx::query!(
                r#"
            SELECT organization_id
            FROM clubs
            WHERE id = $1
            "#,
                id
            )
            .fetch_one(&mut transaction)
            .await?;

            let organization_id = organization_id.organization_id;

            let mut res = sqlx::query(&update_organization_query);

            for param in organization_string_params {
                res = res.bind(param);
            }

            res = res.bind(organization_id);

            let _ = res.execute(&mut transaction).await?;
        }

        transaction.commit().await?;

        Ok(Self::get_by_id(pool, id).await?)
    }

    pub async fn query(
        pool: &sqlx::PgPool,
        request: &RequestType<Club, QueryableClub, ClubSortableField>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        // construct query string and parameters with request.filter
        // request.filter.q is the search query
        // request.filter.data is the filter data with type Self

        let (query, string_params, house_params, int_params) =
            ClubTable::construct_query_string(request);

        let mut res = sqlx::query_as::<_, Self>(&query);

        for param in string_params {
            res = res.bind(param);
        }

        for param in house_params {
            res = res.bind(param);
        }

        for param in int_params {
            res = res.bind(param);
        }

        let res = res.fetch_all(pool).await?;

        // let mut pagination = PaginationType {
        //     total: res.len() as i64,
        //     first: None, // TODO use the RequestType to unparse using serde and return the first page using pagination.p = 1
        //     last: None,  // TODO use the RequestType to unparse using serde and return the last page using pagination.p = last_page
        //     prev: None,  // TODO use the RequestType to unparse using serde and return the previous page using pagination.p = current_page - 1 or None if current_page == 1
        //     next: None,  // TODO use the RequestType to unparse using serde and return the next page using pagination.p = current_page + 1 or None if current_page == last_page
        //     size: match request.pagination {
        //         Some(pagination) => pagination.size,
        //         None => 50,
        //     }
        // };

        // cause LIMIT to be a text[] instead of a bigint in the query

        Ok(res)
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

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct IdOnlyClub {
    #[schema(value_type = String)]
    pub id: Uuid,
}

impl IdOnlyClub {
    fn from_table(club: ClubTable) -> Self {
        Self { id: club.id }
    }

    pub async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<IdOnlyClub, sqlx::Error> {
        let res = ClubTable::get_by_id(pool, id).await?;

        Ok(IdOnlyClub { id: res.id })
    }

    pub async fn query(
        pool: &sqlx::PgPool,
        request: &RequestType<Club, QueryableClub, ClubSortableField>,
    ) -> Result<Vec<IdOnlyClub>, sqlx::Error> {
        let res = ClubTable::query(pool, request).await?;

        Ok(res.iter().map(|r| IdOnlyClub { id: r.id }).collect())
    }
}

#[derive(Deserialize, Serialize, Debug, ToSchema, Clone)]
pub struct CompactClub {
    #[schema(value_type = String)]
    pub id: Uuid,
    pub name: MultiLangString,
    pub description: Option<MultiLangString>,
    pub logo_url: Option<String>,
    pub house: Option<ActivityDayHouse>,
    pub map_location: Option<u32>,
    pub background_color: Option<String>,
}

impl CompactClub {
    fn from_table(club: ClubTable) -> Self {
        Self {
            id: club.id,
            name: MultiLangString {
                th: club.name_th,
                en: club.name_en,
            },
            description: match (club.description_th, club.description_en) {
                (Some(th), Some(en)) => Some(MultiLangString { th, en: Some(en) }),
                _ => None,
            },
            logo_url: club.logo_url,
            house: club.house,
            map_location: club.map_location.map(|l| l as u32),
            background_color: club.background_color,
        }
    }

    pub async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<CompactClub, sqlx::Error> {
        let res = ClubTable::get_by_id(pool, id).await?;

        Ok(CompactClub::from_table(res))
    }

    pub async fn query(
        pool: &sqlx::PgPool,
        request: &RequestType<Club, QueryableClub, ClubSortableField>,
    ) -> Result<Vec<CompactClub>, sqlx::Error> {
        let res = ClubTable::query(pool, request).await?;

        Ok(res
            .iter()
            .map(|r| CompactClub::from_table(r.clone()))
            .collect())
    }
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct DefaultClub {
    #[schema(value_type = String)]
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
    async fn from_table(
        pool: &sqlx::PgPool,
        club: ClubTable,
        descendant_fetch_level: Option<FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        let members =
            ClubTable::get_members(pool, club.id, None, descendant_fetch_level.clone(), None)
                .await?;
        let staffs =
            ClubTable::get_staffs(pool, club.id, None, descendant_fetch_level.clone(), None)
                .await?;
        let contacts = ClubTable::get_contacts(pool, club.id, descendant_fetch_level).await?;

        Ok(Self {
            id: club.id,
            name: MultiLangString {
                th: club.name_th,
                en: club.name_en,
            },
            description: match (club.description_th, club.description_en) {
                (Some(th), Some(en)) => Some(MultiLangString { th, en: Some(en) }),
                _ => None,
            },
            logo_url: club.logo_url,
            staffs,
            members,
            // advisors: vec![],
            background_color: club.background_color,
            accent_color: club.accent_color,
            contacts,
            main_room: club.main_room,
            house: club.house,
            map_location: club.map_location.map(|l| l as u32),
        })
    }

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

    pub async fn query(
        pool: &sqlx::PgPool,
        request: &RequestType<Club, QueryableClub, ClubSortableField>,
        // descendant_fetch_level: Option<FetchLevel>,
    ) -> Result<Vec<DefaultClub>, sqlx::Error> {
        let res = ClubTable::query(pool, request).await?;

        let descendant_fetch_level = request.descendant_fetch_level.clone();

        let mut clubs = Vec::new();

        for r in res.iter() {
            let members =
                ClubTable::get_members(pool, r.id, None, descendant_fetch_level.clone(), None)
                    .await?;
            let staffs =
                ClubTable::get_staffs(pool, r.id, None, descendant_fetch_level.clone(), None)
                    .await?;
            let contacts =
                ClubTable::get_contacts(pool, r.id, descendant_fetch_level.clone()).await?;

            clubs.push(DefaultClub {
                id: r.id,
                name: MultiLangString {
                    th: r.name_th.clone(),
                    en: r.name_en.clone(),
                },
                description: match (r.description_th.clone(), r.description_en.clone()) {
                    (Some(th), Some(en)) => Some(MultiLangString { th, en: Some(en) }),
                    _ => None,
                },
                logo_url: r.logo_url.clone(),
                staffs,
                members,
                background_color: r.background_color.clone(),
                accent_color: r.accent_color.clone(),
                contacts,
                main_room: r.main_room.clone(),
                house: r.house.clone(),
                map_location: r.map_location.map(|l| l as u32),
            });
        }

        Ok(clubs)
    }
}

#[derive(Deserialize, Debug, ToSchema)]
pub enum Club {
    Default(DefaultClub),
    Compact(CompactClub),
    IdOnly(IdOnlyClub),
}

impl Club {
    async fn from_table(
        pool: &sqlx::PgPool,
        club: ClubTable,
        fetch_level: FetchLevel,
        descendant_fetch_level: Option<FetchLevel>,
    ) -> Result<Self, sqlx::Error> {
        let fetch_level = match fetch_level {
            FetchLevel::IdOnly => FetchLevel::IdOnly,
            FetchLevel::Compact => FetchLevel::Compact,
            FetchLevel::Default => FetchLevel::Default,
        };

        match fetch_level {
            FetchLevel::IdOnly => Ok(Club::IdOnly(IdOnlyClub::from_table(club))),
            FetchLevel::Compact => Ok(Club::Compact(CompactClub::from_table(club))),
            FetchLevel::Default => Ok(Club::Default(
                DefaultClub::from_table(pool, club, descendant_fetch_level).await?,
            )),
        }
    }

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

    pub async fn update_by_id(
        pool: &sqlx::PgPool,
        id: Uuid,
        update: &UpdatableClub,
        fetch_level: Option<FetchLevel>,
        descendant_fetch_level: Option<FetchLevel>,
    ) -> Result<Club, sqlx::Error> {
        let fetch_level = match fetch_level {
            Some(fetch_level) => fetch_level,
            None => FetchLevel::Default,
        };

        let update = UpdatableClubTable {
            name_th: match &update.name {
                Some(name) => Some(name.th.clone()),
                None => None,
            },
            name_en: match &update.name {
                Some(name) => name.en.clone(),
                None => None,
            },
            description_th: match &update.description {
                Some(description) => match &description.th {
                    Some(th) => Some(th.clone()),
                    None => None,
                },
                None => None,
            },
            description_en: match &update.description {
                Some(description) => match &description.en {
                    Some(en) => Some(en.clone()),
                    None => None,
                },
                None => None,
            },
            logo_url: update.logo_url.clone(),
            background_color: update.background_color.clone(),
            accent_color: update.accent_color.clone(),
            main_room: update.main_room.clone(),
            house: update.house.clone(),
            map_location: update.map_location,
        };

        let res = ClubTable::update_by_id(pool, id, &update).await?;

        Ok(Club::from_table(pool, res, fetch_level, descendant_fetch_level).await?)
    }
    pub async fn query(
        pool: &sqlx::PgPool,
        request: &RequestType<Club, QueryableClub, ClubSortableField>,
    ) -> Result<Vec<Club>, sqlx::Error> {
        let fetch_level = match &request.fetch_level {
            Some(fetch_level) => fetch_level,
            None => &FetchLevel::Default,
        };

        match fetch_level {
            FetchLevel::IdOnly => Ok(IdOnlyClub::query(pool, request)
                .await?
                .into_iter()
                .map(|c| Club::IdOnly(c))
                .collect()),
            FetchLevel::Compact => Ok(CompactClub::query(pool, request)
                .await?
                .into_iter()
                .map(|c| Club::Compact(c))
                .collect()),
            FetchLevel::Default => Ok(DefaultClub::query(pool, request)
                .await?
                .into_iter()
                .map(|c| Club::Default(c))
                .collect()),
        }
    }
}

impl Serialize for Club {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Club::Default(c) => c.serialize(serializer),
            Club::Compact(c) => c.serialize(serializer),
            Club::IdOnly(c) => c.serialize(serializer),
        }
    }
}
