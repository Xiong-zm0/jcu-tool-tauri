use std::pin::Pin;
use std::future::Future;

use chrono::{self, FixedOffset, TimeZone};
use tauri_plugin_http::reqwest;

use crate::article::{self, Article};
use super::ChannelType;

pub struct ChannelYlcNotice { // Ylc: Youth League Committee
    url_base: String,
    url_patten: (String, String),
    viewed_url_suffix: Option<String>,
}

impl ChannelYlcNotice {
    pub fn new() -> Self where Self: Sized {
        Self {
            url_base: "https://tuanwei.jcu.edu.cn/gg.htm".into(),
            url_patten: ("https://tuanwei.jcu.edu.cn/gg/".into(), ".htm".into()),
            viewed_url_suffix: None,
        }
    }

    pub fn synchronize(&self) -> Pin<Box<dyn Future<Output = Vec<Article>> + Send + '_>> {
        let url = match self.viewed_url_suffix {
            Some(ref suffix) => {
                format!("{}{}{}", self.url_patten.0, suffix, self.url_patten.1)
            },
            None => {
                self.url_base.clone()
            },
        };

        let article_selector = scraper::Selector::parse("li").unwrap();
        let article_url_selector = scraper::Selector::parse("a").unwrap();
        let article_title_selector = scraper::Selector::parse("a").unwrap();
        let article_time_selector = scraper::Selector::parse("span").unwrap();

        Box::pin(async move {
            let mut articles = vec![];
            
            let response = reqwest::get(&url).await.unwrap();
            let body = response.text().await.unwrap();
            let document = scraper::Html::parse_document(&body);
            
            for element in document.select(&article_selector) {
                // 需要过虑 id 不以 #line_u6 开头的元素
                if let Some(id) = element.value().attr("id") {
                    if !id.starts_with("line_u6") {
                        continue;
                    }
                } else {
                    continue;
                }

                let mut article_url = "https://tuanwei.jcu.edu.cn/".to_string();
                article_url.push_str(element
                    .select(&article_url_selector)
                    .next()
                    .and_then(|a| a.value().attr("href"))
                    .unwrap_or("#")
                );

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
                    });

                let mut article = Article::new(article_url, article_title, release_time, ChannelType::YlcNotice);
                article.set_signature("团委公告".into());

                articles.push(article);

            }
            
            articles
        })
    }

    pub fn load_article(mut article: Article) -> Pin<Box<dyn Future<Output = Article> + Send>> {
        todo!()
    }
}
