use serde::{Deserialization, Serialization};

struct ShortURL {
    user_id: u32,
    short_url: String,
    og_url: String,
    created_time: String,
    expiry_time: String
}

impl ShortURL{
    fn short_url(item: ShortURL) -> Result(Self, Self::Error){
        
        Ok(Self {
            short_url: item.short_url
        })
    }
}