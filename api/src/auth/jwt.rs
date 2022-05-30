use std::env;

use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) name: String,
}

pub(crate) fn generate_jwt(claims: &Claims) -> String {
    let secret = env::var("JWT_SECRET").expect("Could not read JWT_SECRET from env");
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&Header::default(), &claims, &key).expect("Could not generate JWT")
}
