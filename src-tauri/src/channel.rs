mod main_news;
mod main_notice;
mod library_news;
mod ylc_notice;


use std::pin::Pin;
use std::future::Future;

use chrono::{self, FixedOffset, TimeZone};
use tauri_plugin_http::reqwest;

use crate::article::Article;
use main_notice::ChannelMainNotice;
use main_news::ChannelMainNews;
use library_news::ChannelLibraryNews;
use ylc_notice::ChannelYlcNotice;


#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ChannelType {
    MainNews,
    MainNotice,
    LibraryNews,
    YlcNotice,
    
}

pub enum Channel {
    MiainNews(ChannelMainNews),
    MainNotice(ChannelMainNotice),
    LibraryNews(ChannelLibraryNews),
    YlcNotice(ChannelYlcNotice),
}

impl Channel {
    pub fn new(channel_enum: ChannelType) -> Self {
        match channel_enum {
            ChannelType::MainNews => Channel::MiainNews(ChannelMainNews::new()),
            ChannelType::MainNotice => Channel::MainNotice(ChannelMainNotice::new()),
            ChannelType::LibraryNews => Channel::LibraryNews(ChannelLibraryNews::new()),
            ChannelType::YlcNotice => Channel::YlcNotice(ChannelYlcNotice::new()),
        }
    }

    pub fn synchronize(&self) -> Pin<Box<dyn Future<Output = Vec<Article>> + Send + '_>> {
        match self {
            Channel::MiainNews(channel) => channel.synchronize(),
            Channel::MainNotice(channel) => channel.synchronize(),
            Channel::LibraryNews(channel) => channel.synchronize(),
            Channel::YlcNotice(channel) => channel.synchronize(),
        }
    }

    pub fn load_article(article: Article) -> Pin<Box<dyn Future<Output = Article> + Send + 'static>> {
        match article.get_channel() {
            ChannelType::MainNews => ChannelMainNews::load_article(article),
            ChannelType::MainNotice => ChannelMainNotice::load_article(article),
            ChannelType::LibraryNews => ChannelLibraryNews::load_article(article),
            ChannelType::YlcNotice => ChannelYlcNotice::load_article(article),
        }
    }
}
