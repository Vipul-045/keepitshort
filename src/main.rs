use axum::extract::State;
use axum::{Json, Router, http::StatusCode, response::Response, routing::get};
use mongodb::Client;

use crate::{
    models::url_model::{OriginalUrl, ShortURL},
    services::db::db_collection;
};

// mod routes;
mod models;
mod services;

#[derive(Clone)]
struct AppState{
    mongo: 
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let app = Router::new().route("/", get(get_shorturl)).with_state(app_state);

    let address = "localhost:6650";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn get_shorturl(State(state): State<AppState>, request: Json<OriginalUrl>) -> Response {
    let store = url_
        .await
    {
        Ok(shorturl) => Json(shorturl).into_response(),
        Err(err) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(err.to_string()),
    }
}
