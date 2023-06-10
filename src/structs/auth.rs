use actix_web::error::{ErrorNotFound, ErrorUnauthorized};
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, web, FromRequest, HttpRequest};
// use anyhow::Ok;
use async_trait::async_trait;
use futures::Future as FutureTrait;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;

use std::pin::Pin;

use crate::AppState;

use crate::structs::common::{ErrorResponseType, ErrorType};

#[derive(Debug)]
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

// implement serialization and deserialization for UserRoles enum using from and to string
impl Serialize for UserRoles {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for UserRoles {
    fn deserialize<D>(deserializer: D) -> Result<UserRoles, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(UserRoles::from_string(&s))
    }
}

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

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: Option<String>,
    pub role: UserRoles,
    pub student: Option<i64>,
    pub teacher: Option<i64>,
    pub onboarded: bool,
    pub is_admin: bool,
}

impl User {
    pub fn new(user: UserTable) -> Self {
        User {
            id: user.id,
            email: user.email,
            role: UserRoles::new(&user.role),
            student: user.student,
            teacher: user.teacher,
            onboarded: user.onboarded,
            is_admin: user.is_admin,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TokenClaim {
    aud: String,
    exp: i64,
    sub: String,
    email: String,
}

#[async_trait]
impl FromRequest for User {
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

    type Error = ActixWebError;
    type Future = Pin<Box<dyn FutureTrait<Output = Result<Self, Self::Error>>>>;
    // type Config = ();
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let pool = req.app_data::<web::Data<AppState>>().unwrap().db.clone();
        let jwt_secret = req
            .app_data::<web::Data<AppState>>()
            .unwrap()
            .jwt_secret
            .clone();

        let auth_header = req.headers().get(http::header::AUTHORIZATION);

        let token = match auth_header {
            Some(token) => match token.to_str() {
                Ok(token) => token,
                Err(_) => {
                    return Box::pin(async {
                        // return 401 unauthorized if the token is not a string as ResponseType
                        Err(ErrorUnauthorized(ErrorResponseType::new(
                            ErrorType {
                                id: "401".to_string(),
                                detail: "Invalid token".to_string(),
                                code: 401,
                                error_type: "invalid_token".to_string(),
                                source: "".to_string(),
                            },
                            None,
                        )))
                    });
                }
            },
            None => {
                return Box::pin(async {
                    Err(ErrorUnauthorized(ErrorResponseType::new(
                        ErrorType {
                            id: "401".to_string(),
                            detail: "Missing Token".to_string(),
                            code: 401,
                            error_type: "missing_token".to_string(),
                            source: "".to_string(),
                        },
                        None,
                    )))
                })
            }
        };

        let token = token.replace("Bearer ", "");

        let claims = match decode::<TokenClaim>(
            &token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(claims) => claims,
            Err(_) => {
                return Box::pin(async {
                    Err(ErrorUnauthorized(ErrorResponseType::new(
                        ErrorType {
                            id: "401".to_string(),
                            detail: "Invalid token".to_string(),
                            code: 401,
                            error_type: "invalid_token".to_string(),
                            source: "".to_string(),
                        },
                        None,
                    )))
                })
            }
        };

        let user_id = match Uuid::parse_str(&claims.claims.sub) {
            Ok(user_id) => user_id,
            Err(_) => {
                return Box::pin(async {
                    Err(ErrorNotFound(ErrorResponseType::new(
                        ErrorType {
                            id: "404".to_string(),
                            detail: "User not found".to_string(),
                            code: 404,
                            error_type: "entity_not_found".to_string(),
                            source: "".to_string(),
                        },
                        None,
                    )))
                })
            }
        };

        // use box pin to pin the future to the heap and get user asyncronously that will be used in the future
        Box::pin(async move {
            let user_from_table = UserTable::from_id(user_id, &pool).await;

            let user: User = match user_from_table {
                Ok(user) => User::new(user),
                Err(_) => {
                    return Err(ErrorNotFound(ErrorResponseType::new(
                        ErrorType {
                            id: "404".to_string(),
                            detail: "User not found".to_string(),
                            code: 404,
                            error_type: "entity_not_found".to_string(),
                            source: "".to_string(),
                        },
                        None,
                    )))
                }
            };

            Ok(user)
        })
    }
}
