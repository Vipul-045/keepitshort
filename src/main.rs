use axum::extract::{State, Path};
use axum::http::header;
use axum::{
    Json, Router,
    http::{StatusCode, Method, HeaderValue},
    response::IntoResponse,
    routing::{get, post},
};
use tower_http::cors::{CorsLayer, Any};

use crate::models::url_model::{OriginalUrl, ShortURL};
use crate::services::db::DataStructure;

// mod routes;
mod models;
mod services;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let cors = CorsLayer::new()
    .allow_origin(HeaderValue::from_static("https://ks.vipul.live"))
    .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
    .allow_headers(Any);
    let mongo = DataStructure::new().await.expect("Failed to connect");
    let app = Router::new()
        .route("/", post(get_shorturl))
        .route("/{shortcode}", get(get_ogurl)).layer(cors)
        .with_state(mongo);

    let address = "0.0.0.0:3001";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn get_shorturl(
    State(state): State<DataStructure>,
    Json(payload): Json<OriginalUrl>,
) -> impl IntoResponse {
    let url = ShortURL::try_from(payload).unwrap();
    match state.url_storage(url.clone()).await {
        Ok(_) => {
            let short_url = url.short_url;
            let full_url = format!("ks.vipul.live/{}", short_url);
            (StatusCode::CREATED, Json(full_url)).into_response()
        }
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

async fn get_ogurl(State(state): State<DataStructure>, Path(shortcode): Path<String>) -> impl IntoResponse{
    match state.get_ogurl(&shortcode).await{
        Ok(Some(record)) => {
            let og_url = record.og_url;
            (StatusCode::FOUND, [(header::LOCATION, &og_url)]).into_response()
        }
        Ok(None) => {
            StatusCode::NOT_FOUND.into_response()
        }
        Err(err)=>(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }

}
