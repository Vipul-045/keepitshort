use axum::{Json,  Router, routing::get, response::{Response}};
use crate::{models::url_model::{ShortURL, OriginalUrl}, services::db::data_structure};


mod routes;
mod services;
mod models;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let app = Router::new().route("/", get(get_shorturl));

    let address = "localhost:6650";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn get_shorturl(db: Data<data_structure>,request: Json<OriginalUrl>) -> Response{
    match db.Original_url(ShortURL::try_from(OriginalUrl{
        og_url: request.og_url.clone()
    }).expect("Error creating short url")
).await{
    Ok(shorturl) => Response::Ok().json(shorturl),
    Err(err) => Response::InternalServerError().body(err.to_string())
}
}