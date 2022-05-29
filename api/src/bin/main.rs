use api::auth::{sign_in, user_exists};
use axum::{routing::post, Router};
use dotenv::dotenv;
use std::{env, net::SocketAddr, str::FromStr};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_url = env::var("API_URL").expect("Could not read API_URL from env");
    let addr = SocketAddr::from_str(&api_url).expect("Could not create socket addr");
    let app = Router::new()
        .route("/user_exists", post(user_exists))
        .route("/sign_in", post(sign_in));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
