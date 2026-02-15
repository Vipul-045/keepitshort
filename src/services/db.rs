use mongodb::bson::doc;
use mongodb::{
    Client, Collection, bson::de::Error, options::ClientOptions, results::InsertOneResult,
};
use std::env;

use crate::models::url_model::ShortURL;

#[derive(Clone)]
pub struct DataStructure {
    pub og_url: Collection<ShortURL>,
}

impl DataStructure {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let uri = env::var("MONGODB_URI").expect("CONNECTION STRING NOT FOUND");

        let client_options = ClientOptions::parse(&uri).await?;

        let client = Client::with_options(client_options)?;

        client
            .database("admin")
            .run_command(doc! {"ping":1})
            .await?;
        println!("MongoDB Connected Successfully!");

        let collection = client.database("SHORT_URL").collection::<ShortURL>("urls");

        Ok(Self { og_url: collection })
    }

    pub async fn url_storage(&self, og_url: ShortURL) -> Result<InsertOneResult, Error> {
        let result = self
            .og_url
            .insert_one(og_url)
            .await
            .expect("Error inserting a url");
        Ok(result)
    }

    pub async fn get_ogurl(&self, short_url: &str) -> Result<Option<ShortURL>, Error> {
        let result = self
            .og_url
            .find_one(doc! {"short_url": short_url })
            .await
            .expect("No url found");
        Ok(result)
    }
}
