use std::path::PathBuf;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    url: String,
    title: String,
    release_time: Option<f32>,
    cover: Option<Image>,
    passages: Option<Vec<Passage>>,
    signature: Option<String>,
    date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
enum Passage {
    Text(String),
    Image(Image),
    Video(Video),
    Table(Table),
}

#[derive(Serialize, Deserialize, Debug)]
struct Image {
    aspect: f32,
    description: Option<String>,
    url: String,
    cache_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct Video {
    aspect: f32,
    description: Option<String>,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Table {
    header: Vec<String>,
    body: Vec<Vec<String>>,
}


impl Article {
    pub fn new(url: String, title: String, release_time: Option<f32>) -> Self {
        Self {
            url,
            title,
            release_time,
            cover: None,
            passages: None,
            signature: None,
            date: None,
        }
    }

    pub fn set_signature(&mut self, signature: String) {
        self.signature = Some(signature);
    }
}