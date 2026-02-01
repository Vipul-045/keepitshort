use serde::{Deserialization, Serialization};

#[derive(Debug, Deserialize, Serialize)]
pub struct ShortURL {
    user_id: u32,
    short_url: String,
    og_url: String,
    created_time: String,
    expiry_time: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OriginalUrl {
    og_url: String
}

impl ShortURL{
    fn short_url(&self) -> Result<&Self, &self::Error>{
        
        Ok(&Self {
            short_url: self.short_url
        })
    }
}