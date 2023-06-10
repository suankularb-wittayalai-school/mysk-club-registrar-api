use actix_web::dev::{Payload, ServiceRequest};
use actix_web::{web, FromRequest, HttpRequest, HttpResponse, Error};
use async_trait::async_trait;
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;

use std::pin::Pin;
use std::task::{Context, Poll};


use crate::AppState;

use crate::structs::common::{ResponseType, ErrorType};

#[derive(Debug, Deserialize, Serialize)]
pub enum UserRoles {
    Teacher,
    Student,
}

impl UserRoles {
    pub fn to_string(&self) -> String {
        match self {
            UserRoles::Teacher => "\"teacher\"".to_string(),
            UserRoles::Student => "\"student\"".to_string(),
        }
    }
    pub fn from_string(role: &str) -> UserRoles {
        match role {
            "\"teacher\"" => UserRoles::Teacher,
            "\"student\"" => UserRoles::Student,
            _ => UserRoles::Student,
        }
    }

    fn new(role: &str) -> UserRoles {
        self::UserRoles::from_string(role)
    }
}

// implement serialization and deserialization for UserRoles

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct UserTable {
    pub id: Uuid,
    pub email: Option<String>,
    pub role: String,
    pub student: Option<i64>,
    pub teacher: Option<i64>,
    pub onboarded: bool,
    pub is_admin: bool,
}

pub struct UserNotFoundError;

impl UserTable {
    async fn from_id(id: Uuid, db: &Pool<Postgres>) -> Result<UserTable, UserNotFoundError> {
        let user = sqlx::query_as!(
            UserTable,
            r#"
            SELECT id, email, role, student, teacher, onboarded, is_admin
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(db)
        .await;
        match user {
            Ok(user) => Ok(user),
            Err(_) => Err(UserNotFoundError),
        }
    }
}

#[async_trait]
impl FromRequest for UserTable {
    //  example jwt payload
    // {
    //     "aud": "authenticated",
    //     "exp": 1686293979,
    //     "sub": "e2104f1d-a1e4-4b3a-90c3-46e11d7592f9",
    //     "email": "smart.wat@student.sk.ac.th",
    //     "phone": "",
    //     "app_metadata": {
    //       "provider": "email",
    //       "providers": [
    //         "email"
    //       ]
    //     },
    //     "user_metadata": {
    //       "isAdmin": false,
    //       "onboarded": true,
    //       "role": "student",
    //       "student": 3205
    //     },
    //     "role": "authenticated",
    //     "aal": "aal1",
    //     "amr": [
    //       {
    //         "method": "password",
    //         "timestamp": 1686290379
    //       }
    //     ],
    //     "session_id": "1ef871ef-0c0f-4f6a-b60a-6249a02ac2c7"
    //   }

    // if an error occurs, return the response type with the error and none T value


    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    // type Config = ();
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        todo!()
    }
}
