use std::pin::Pin;
use std::future::Future;

use chrono::{self, FixedOffset, TimeZone};
use tauri_plugin_http::reqwest;

use crate::article::{self, Article};
use super::ChannelType;


pub struct ChannelLibraryNews {
    url_base: String,
    url_patten: (String, String),
    viewed_url_suffix: Option<String>,
}

impl ChannelLibraryNews {
    pub fn new() -> Self where Self: Sized {
        todo!()
    }

    pub fn synchronize(&self) ->
    Pin<Box<dyn Future<Output = Vec<Article>> + Send + '_>> {
        todo!()
    }

    pub fn load_article(mut article: Article) -> Pin<Box<dyn Future<Output = Article> + Send>> {
        todo!()
    }
}
