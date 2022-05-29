use std::env;

use api::auth::{SignInRequest, UserExistsRequest};
use reqwest::blocking::Response;

pub(crate) fn user_exists(name: String) -> Response {
    let api_url = env::var("API_URL").expect("Could not read API_URL from env");
    let client = reqwest::blocking::Client::new();

    client
        .post(format!("http://{api_url}/user_exists"))
        .json(&UserExistsRequest { name })
        .send()
        .expect("Could not reach API.")
}

pub(crate) fn sign_in(name: String, password: String) -> Response {
    let api_url = env::var("API_URL").expect("Could not read API_URL from env");
    let client = reqwest::blocking::Client::new();

    client
        .post(format!("http://{api_url}/sign_in"))
        .json(&SignInRequest { name, password })
        .send()
        .expect("Could not reach API.")
}
