use serde::{Deserialize, Serialize};
use std::{convert::TryFrom};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShortURL {
    // user_id: u32,
    // short_url: String,
    pub og_url: String,
    // created_time: String,
    // expiry_time: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OriginalUrl {
    pub og_url: String
}

impl TryFrom<OriginalUrl> for ShortURL{
    type Error = Box<dyn std::error::Error>;
    fn try_from(item: OriginalUrl) -> Result<Self, Self::Error>{
        
        Ok(Self {
            og_url: item.og_url.clone()
        })
    }
}