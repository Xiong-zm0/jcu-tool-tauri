use std::pin::Pin;
use std::future::Future;

use chrono::{self, FixedOffset, TimeZone};
use tauri_plugin_http::reqwest;

use crate::article::{self, Article};


pub trait Channel: Send + Sync {
    fn new() -> Self where Self: Sized;
    fn synchronize(&self) -> Pin<Box<dyn Future<Output = Vec<Article>> + Send + '_>>;
}

pub struct ChannelMainNews {
    url_base: String,
    url_patten: (String, String),
    viewed_url_suffix: Option<String>,
}

impl Channel for ChannelMainNews {
    fn new() -> Self {
        Self {
            url_base: "https://www.jcu.edu.cn/home/tdxw.htm".into(),
            url_patten: ("https://www.jcu.edu.cn/home/tdxw/".into(), ".htm".into()),
            viewed_url_suffix: None,
        }
    }

    fn synchronize(&self) -> Pin<Box<dyn Future<Output = Vec<Article>> + Send + '_>> {
        let url = match self.viewed_url_suffix {
            Some(ref suffix) => {
                format!("{}{}{}", self.url_patten.0, suffix, self.url_patten.1)
            },
            None => {
                self.url_base.clone()
            },
        };

        let article_selector = scraper::Selector::parse("li.news_list_li").unwrap();
        let article_url_selector = scraper::Selector::parse("a").unwrap();
        let article_title_selector = scraper::Selector::parse("span.news_title").unwrap();
        let article_time_selector = scraper::Selector::parse("span.news_dt11").unwrap();

        Box::pin(async move {
            let mut articles = vec![];
            
            // 这里需要使用 &url，因为 url 的所有权还在
            let response = reqwest::get(&url).await.unwrap();
            let body = response.text().await.unwrap();
            let document = scraper::Html::parse_document(&body);
            
            for element in document.select(&article_selector) {
                let article_url = element
                    .select(&article_url_selector)
                    .next()
                    .and_then(|a| a.value().attr("href"))
                    .unwrap_or("#")
                    .to_string()
                    .replace("../", "https://www.jcu.edu.cn/");

                let article_title = element
                    .select(&article_title_selector)
                    .next()
                    .map(|el| el.text().collect::<String>())
                    .unwrap_or_else(|| "[无标题]".to_string());

                let release_time = element
                    .select(&article_time_selector)
                    .next()
                    .map(|el| el.text().collect::<String>())
                    .and_then(|time_str| {
                        let naive_date = chrono::NaiveDate::parse_from_str(&time_str, "%Y-%m-%d").ok()?;
                        let naive_datetime = naive_date.and_hms_opt(12, 0, 0)?;
                        let beijing_offset = FixedOffset::east_opt(8 * 3600)?;
                        let timestamp = beijing_offset.from_local_datetime(&naive_datetime).single()?.timestamp();
                        Some(timestamp as f32)
                    })
                    .unwrap_or(0.0);

                let mut article = Article::new(article_url.clone(), article_title.clone(), release_time);
                article.set_signature("主站新闻".into());
                
                articles.push(article);

            }
            
            articles
        })
    }
}

pub struct ChannelMainNotice {
    url_base: String,
    newest_url_suffix: Option<String>,
}

impl Channel for ChannelMainNotice {
    fn new() -> Self {
        Self {
            url_base: "https://www.jcu.edu.cn/home/tzgg.htm".into(),
            newest_url_suffix: None,
        }
    }

    fn synchronize(&self) -> Pin<Box<dyn Future<Output = Vec<Article>> + Send + '_>> {
        todo!();
    }
}