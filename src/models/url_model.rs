use mongodb::bson::{ DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom};
use chrono::{ Duration, Utc};
use rand::{Rng, distr::Alphanumeric};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShortURL {
    pub og_url: String,
    pub short_url: String,
    created_time: BsonDateTime,
    expiry_time: BsonDateTime
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OriginalUrl {
    pub og_url: String
}

impl TryFrom<OriginalUrl> for ShortURL{
    type Error = Box<dyn std::error::Error>;
    fn try_from(item: OriginalUrl) -> Result<Self, Self::Error>{
        let now = Utc::now();
        let expires_at = now + Duration::seconds(30);
        
        
        Ok(Self {
            og_url: item.og_url.clone(),
            short_url: generate_four_char(4),
            created_time : BsonDateTime::from_millis(now.timestamp_millis()),
            expiry_time: BsonDateTime::from_millis(expires_at.timestamp_millis())
        })
    }
}

pub fn generate_four_char(len: usize) -> String {
    rand::rng()
    .sample_iter(&Alphanumeric)
    .take(len)
    .map(char::from)
    .collect()
}