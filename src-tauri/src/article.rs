use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::channel::ChannelType;


#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    url: String,
    title: String,
    channel: ChannelType,
    release_time: Option<f32>,
    cover: Option<Image>,
    passages: Option<Vec<Passage>>,
    signature: Option<String>,
    date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Passage {
    Text(Vec<TextSegment>),
    Image(Image),
    Video(Video),
    Table(Table),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextSegment {
    content: String,
    style: Vec<Style>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Style {
    Append,
    Bold,
    Right,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    aspect: Option<f32>,
    description: Option<String>,
    url: String,
    cache_path: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    aspect: Option<f32>,
    description: Option<String>,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    header: Vec<String>,
    body: Vec<Vec<String>>,
}


impl Article {
    pub fn new(url: String, title: String, release_time: Option<f32>, channel: ChannelType) -> Self {
        Self {
            url,
            title,
            release_time,
            channel,
            cover: None,
            passages: None,
            signature: None,
            date: None,
        }
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn get_channel(&self) -> &ChannelType {
        &self.channel
    }

    pub fn set_cover(&mut self, cover: Image) {
        self.cover = Some(cover);
    }

    pub fn set_passages(&mut self, passages: Vec<Passage>) {
        self.passages = Some(passages);
    }

    pub fn set_signature(&mut self, signature: String) {
        self.signature = Some(signature);
    }
}

impl TextSegment {
    pub fn new(content: String, style: Vec<Style>) -> Self {
        Self { content, style }
    }
}

impl Image {
    pub fn new(url: String) -> Self {
        Self { aspect: None, description: None, url, cache_path: None }
    }
}