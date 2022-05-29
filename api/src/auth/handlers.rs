use axum::{http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use database::{establish_connection, models::User, schema::users::dsl::*};
use diesel::{prelude::*, result::Error::NotFound};
use serde::{Deserialize, Serialize};

use super::jwt::{generate_jwt, Claims};

#[derive(Serialize, Deserialize)]
pub struct UserExistsRequest {
    pub name: String,
}

pub async fn user_exists(Json(input): Json<UserExistsRequest>) -> impl IntoResponse {
    let connection = establish_connection();

    match users
        .filter(name.eq(&input.name))
        .first::<User>(&connection)
    {
        Ok(_) => StatusCode::FOUND,
        Err(NotFound) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignInRequest {
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignInResponse {
    token: String,
}

pub async fn sign_in(Json(input): Json<SignInRequest>) -> Result<Json<SignInResponse>, StatusCode> {
    let connection = establish_connection();

    match users
        .filter(name.eq(&input.name))
        .first::<User>(&connection)
    {
        Ok(user) => {
            if let Ok(verified) = verify(input.password, &user.password) {
                if verified {
                    let token = generate_jwt(&Claims {
                        name: input.name.clone(),
                    });

                    Ok(Json(SignInResponse { token }))
                } else {
                    Err(StatusCode::FORBIDDEN)
                }
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Err(NotFound) => {
            if let Ok(hashed) = hash(input.password, DEFAULT_COST) {
                let result = diesel::insert_into(users)
                    .values((name.eq(&input.name), password.eq(hashed)))
                    .execute(&connection);

                match result {
                    Ok(_) => {
                        let token = generate_jwt(&Claims {
                            name: input.name.clone(),
                        });

                        Ok(Json(SignInResponse { token }))
                    }
                    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
