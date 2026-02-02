use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct ShortURL {
    // user_id: u32,
    // short_url: String,
    og_url: String,
    // created_time: String,
    // expiry_time: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OriginalUrl {
    og_url: String
}

impl ShortURL{
    fn short_url(&self) -> Result<Self, Box<dyn std::error::Error>>{
        
        Ok(Self {
            og_url: self.og_url.clone()
        })
    }
}