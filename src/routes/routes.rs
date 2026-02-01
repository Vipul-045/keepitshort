use axum::{Router, routing::get, response::{Response}};
use crate::{models::url_model::{ShortURL, Original_url}, services::db::data_structure};


pub async fn get_shorturl(db: Data<Database>,request: Json<Original_url>) -> Response{
    match db.Original_url(ShortURL::try_from(Original_url{
        og_url: request.og_url.clone()
    }).expect("Error creating short url")
).await{
    Ok(shorturl) => Response::Ok().json(shorturl),
    Err(err) => Response::InternalServerError().body(err.to_string())
}
}