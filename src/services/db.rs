use mongodb::{
    Client, Collection, bson::de::Error, options::ClientOptions, results::{DeleteResult, InsertOneResult, UpdateResult}
};
use std::env;

pub struct data_structure {
    short_url: Collection<short_url>,
}

impl data_structure {
    async fn db_connection() -> Self {
        let uri = env::var("MONGODB_URI").expect("CONNECTION STRING NOT FOUND");

        let client_options = ClientOptions::parse(&uri).await?;

        let client = Client::with_options(client_options)?;

        client.database("Keepitshort").run_command(doc! {ping:1}).await?;
        println!("MongoDB Connected Successfully!");

        Ok(())
    }

    async fn url_storage(&self, short_url: short_url) -> Result<InsertOneResult, Error> {
        let result = self
            .short_url
            .insert_one(short_url)
            .await
            .ok()
            .expect("Error inserting a url");
        Ok(result)
    }
}
