use serde::{Deserialization, Serialization};

struct ShortURL {
    user_id: u32,
    short_url: String,
    og_url: String,
    created_time: String,
    expiry_time: String
}

impl ShortURL{
    fn short_url(&self) -> Result(&self, &self::Error){
        
        Ok(Self {
            short_url: self.short_url
        })
    }
}