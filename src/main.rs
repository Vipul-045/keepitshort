use axum::extract::State;
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};

use crate::services::db::DataStructure;
use crate::{
    models::url_model::{OriginalUrl, ShortURL}
};

// mod routes;
mod models;
mod services;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let mongo = DataStructure::new().await.expect("Failed to connect");
    let app = Router::new().route("/url", get(get_shorturl)).with_state(mongo);

    let address = "localhost:6650";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn get_shorturl(State(state): State<DataStructure>, Json(payload): Json<OriginalUrl>) -> impl IntoResponse {
    let url = ShortURL::try_from(payload).unwrap();
    match state.url_storage(url.clone()).await{
        Ok(_) => (StatusCode::CREATED, Json(url)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
    }
}
