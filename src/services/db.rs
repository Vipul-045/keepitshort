use axum::response::Response;
use mongodb::{
    Client, Collection, bson::de::Error, options::ClientOptions, results::{DeleteResult, InsertOneResult, UpdateResult}
};
use std::env;
use mongodb::bson::doc;

use crate::models::url_model::ShortURL;

pub struct DataStructure {
        og_url : Collection<ShortURL>,
}

impl DataStructure {
    async fn db_connection() -> Result<Client, mongodb::error::Error> {
        let uri = env::var("MONGODB_URI").expect("CONNECTION STRING NOT FOUND");

        let client_options = ClientOptions::parse(&uri).await?;

        let client = Client::with_options(client_options)?;

        client.database("SHORT_URL").run_command(doc! {"ping":1}).await?;
        println!("MongoDB Connected Successfully!");

        Ok(client)
    }

    async fn url_storage(&self, og_url: ShortURL) -> Result<InsertOneResult, Error> {
        let result = self
            .og_url
            .insert_one(og_url)
            .await
            .ok()
            .expect("Error inserting a url");
        Ok(result)
    }
}
