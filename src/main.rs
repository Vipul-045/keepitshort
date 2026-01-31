use mongodb::{Client, options::ClientOptions};
use std::env;
use mongodb::bson::doc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mongodb_uri = env::var("MONGODB_URI").expect("NO CONNECTION URL FOUND");

    let client_options = ClientOptions::parse(&mongodb_uri).await?;

    let client = Client::with_options(client_options)?;

    client.database("admin").run_command(doc!{"ping":1}).await?;
    
    Ok(println!("MongoDB connected"))
}
