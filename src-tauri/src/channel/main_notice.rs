use std::pin::Pin;
use std::future::Future;

use chrono::{self, FixedOffset, TimeZone};
use tauri_plugin_http::reqwest;

use crate::article::{self, Article};
use super::ChannelType;


pub struct ChannelMainNotice {
    url_base: String,
    url_patten: (String, String),
    viewed_url_suffix: Option<String>,
}

impl ChannelMainNotice {
    pub fn new() -> Self {
        Self {
            url_base: "https://www.jcu.edu.cn/home/tzgg.htm".into(),
            url_patten: ("https://www.jcu.edu.cn/home/tzgg/".into(), ".htm".into()),
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

        let article_selector = scraper::Selector::parse("li.text_list_li").unwrap();
        let article_url_selector = scraper::Selector::parse("a").unwrap();

        Box::pin(async move {
            let mut articles = vec![];
            
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
                    .value().attr("title")
                    .unwrap_or("[None Title]")
                    .to_string();

                let release_time = None;

                let mut article = Article::new(article_url, article_title, release_time, ChannelType::MainNotice);
                article.set_signature("主站公告".into());

                articles.push(article);

            }
            
            articles
        })
    }

    pub fn load_article(mut article: Article) -> Pin<Box<dyn Future<Output = Article> + Send>> {
        let url = article.get_url().to_string();
        let article_time_selector = scraper::Selector::parse("div.c_extra").unwrap();
        let content_selector = scraper::Selector::parse("div.v_news_content").unwrap();

        Box::pin(async move {
            let mut passages = vec![];

            let response = reqwest::get(&url).await.unwrap();
            let body = response.text().await.unwrap();
            let document = scraper::Html::parse_document(&body);
            let content = document.select(&content_selector).next().unwrap();

            let release_time = document
                .select(&article_time_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .and_then(|time_str| {
                    let time_str = &time_str[9..19];
                    let naive_date = chrono::NaiveDate::parse_from_str(&time_str, "%Y-%m-%d").ok()?;
                    let naive_datetime = naive_date.and_hms_opt(12, 0, 0)?;
                    let beijing_offset = FixedOffset::east_opt(8 * 3600)?;
                    let timestamp = beijing_offset.from_local_datetime(&naive_datetime).single()?.timestamp();
                    Some(timestamp as f32)
                });

            for paragraph in content.child_elements() {
                let paragraph_label = paragraph.value().name();
                let passage = match paragraph_label {
                    "p" => { // Text or Image
                        ChannelMainNotice::parse_passage_img_or_text(paragraph)
                    },
                    "div" => { // Table
                        // ChannelMainNotice::parse_passage_table(paragraph)
                        None
                    },
                    _ => None,
                };

                if let Some(passage) = passage {
                    passages.push(passage);
                };
            }

            if let Some(release_time) = release_time {
                article.set_release_time(release_time);
            }
            article.set_passages(passages);
            article
        })
    }
}

impl ChannelMainNotice {
    fn parse_passage_img_or_text(paragraph: scraper::ElementRef) -> Option<article::Passage> {
        let img_selector = scraper::Selector::parse("img").unwrap();
        let text_span_selector = scraper::Selector::parse("span").unwrap();
        if let Some(img) = paragraph.select(&img_selector).next() {
            let mut url = "https://www.jcu.edu.cn".to_string();
            url.push_str(
                img
                    .value()
                    .attr("src")
                    .unwrap_or("")
            );

            Some(article::Passage::Image(article::Image::new(url)))
        } else if let Some(text_span) = paragraph.select(&text_span_selector).next() {
            let mut text_segments = vec![];
            let is_right_aligned = paragraph
                .value()
                .attr("style")
                .map_or(false, |style| {
                    style.contains("text-align: right") ||
                    style.contains("text-align:right")
                });

            for child in text_span.children() {
                match child.value() {
                    scraper::Node::Text(text_node) => {
                        let text = text_node.text.trim().to_string();
                        let style = if is_right_aligned {
                            vec![article::Style::Right]
                        } else {
                            vec![]
                        };
                        text_segments.push(article::TextSegment::new(text, style));
                    },
                    scraper::Node::Element(_) => {
                        if let Some(element_ref) = scraper::ElementRef::wrap(child) {
                            if element_ref.value().name() == "strong" {
                                let text = element_ref.text().collect::<String>().trim().to_string();
                                let style = vec![article::Style::Bold];
                                text_segments.push(article::TextSegment::new(text, style));
                            } else {
                                let text = paragraph.text().collect::<String>();
                                text_segments.push(article::TextSegment::new(text, vec![]));
                            }
                        }
                    },
                    _ => {},
                }
            }

            if text_segments.len() == 0 {
                text_segments.push(article::TextSegment::new(String::new(), vec![]));
            }
            
            Some(article::Passage::Text(text_segments))
        } else {
            let mut text_segments = vec![];
            let t = paragraph.text().collect::<String>();
            text_segments.push(article::TextSegment::new(t, vec![]));
            Some(article::Passage::Text(text_segments))
        }
    }

    fn parse_passage_table(paragraph: scraper::ElementRef) -> Option<article::Passage> {
        todo!()
    }
}
