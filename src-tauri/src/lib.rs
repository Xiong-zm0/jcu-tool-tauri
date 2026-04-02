mod cache;
mod settings;
mod article;
mod channel;

use std::sync::{Arc, Mutex};

use scraper;
use tauri::{self, Manager};
use tauri_plugin_http::reqwest;

use settings::Settings;
use article::Article;
use channel::Channel;


struct AppState {
    settings: Settings,
    channels: Vec<Arc<dyn channel::Channel>>,
}

#[tauri::command]
async fn synchronize_channels(state: tauri::State<'_, Mutex<AppState>>) -> Result<Vec<Article>, String> {
    let channels = {
        let state = state.lock().unwrap();
        state.channels.clone() 
    };

    let mut all_articles = Vec::new();
    for c in channels {
        let articles = c.synchronize().await;
        all_articles.extend(articles);
    }
    Ok(all_articles)
}

struct Info {
    department: String,
    title: String,
    release_time: f32,
    artical_url: String,
    cover_cache: String,
    cover_url: String,
}

impl Info {
    fn to_json(&self) -> String {
        format!(
            r##"{{
                "department": "{}",
                "title": "{}",
                "releaseTime": {},
                "articalUrl": "{}",
                "coverCache": "{}",
                "coverUrl": "{}"
            }}"##,
            self.department,
            self.title,
            self.release_time,
            self.artical_url,
            self.cover_cache,
            self.cover_url,
        )
    }
}

#[tauri::command]
async fn get_informations<'r>() -> Result<String, String> {
    let url = "https://www.jcu.edu.cn/home/tdxw.htm".to_string();

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!(r#"{{"error": "{} [2]"}}"#, e))?;

    let body = response
        .text()
        .await
        .map_err(|e| format!(r#"{{"error": "{} [3]"}}"#, e))?;

    let mut informations = {
        let document = scraper::Html::parse_document(&body);
        extract_news(&document)
            .iter()
            .map(|i| i.to_json())
            .collect::<Vec<String>>()
            .join(", ")
    };

    let url = "https://www.jcu.edu.cn/home/tzgg.htm".to_string();
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!(r#"{{"error": "{} [4]"}}"#, e))?;
    let body = response
        .text()
        .await
        .map_err(|e| format!(r#"{{"error": "{} [5]"}}"#, e))?;
    let document = scraper::Html::parse_document(&body);
    informations = format!(
        "{}, {}",
        informations,
        extract_notice(&document)
            .iter()
            .map(|i| i.to_json())
            .collect::<Vec<String>>()
            .join(", ")
    );
    let informations = format!("[{}]", informations);
    Ok(informations)
}

#[tauri::command]
async fn get_artical<'r>(url: String) -> Result<String, String> {
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!(r#"{{"error": "{} [4]"}}"#, e))?;

    let body = response
        .text()
        .await
        .map_err(|e| format!(r#"{{"error": "{}"[5]}}"#, e))?;

    let document = scraper::Html::parse_document(&body);
    let artical = extract_artical(&document);
    Ok(artical)
}

fn parse_total_page(document: &scraper::Html) -> u32 {
    let total_page_selector = scraper::Selector::parse(".pb_sys_common .p_t").unwrap();
    let mut total_page = document.select(&total_page_selector);
    total_page.next();
    let mut total_page = total_page
        .next()
        .map(|el| el.text().collect::<String>())
        .unwrap_or_else(|| "[无总页数]".to_string())
        .trim()
        .to_string();

    total_page.pop();
    total_page.remove(0);
    total_page.parse::<u32>().unwrap()
}

fn extract_news(document: &scraper::Html) -> Vec<Info> {
    let li_selector = scraper::Selector::parse("li.news_list_li").unwrap();
    let title_selector = scraper::Selector::parse("span.news_title").unwrap();
    let date_selector = scraper::Selector::parse("span.news_dt11").unwrap();
    let summary_selector = scraper::Selector::parse(".news_summary").unwrap();
    let mut info_vec = Vec::new();

    for li in document.select(&li_selector) {
        let title = li
            .select(&title_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_else(|| "[无标题]".to_string())
            .trim()
            .to_string();

        let date = li
            .select(&date_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_else(|| "[无日期]".to_string())
            .trim()
            .to_string();

        let summary = li
            .select(&summary_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or("[无摘要]".to_string())
            .trim()
            .to_string();

        let artical_url = li
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .unwrap_or("#");

        let artical_url = artical_url.replace("../", "https://www.jcu.edu.cn/");

        let cover_url = li
            .select(&scraper::Selector::parse(".news_thumb img").unwrap())
            .next()
            .and_then(|img| img.value().attr("src"))
            .unwrap_or("#");

        let cover_url = format!("https://www.jcu.edu.cn{}", cover_url);
        // println!("{}", title);
        // println!("   日期: {}", date);
        // println!("   摘要: {}", summary);
        // println!("   详情: {}\n", detail_url);

        info_vec.push(Info {
            title: title,
            department: "主站新闻".to_string(),
            release_time: 1.0,
            artical_url: artical_url.to_string(),
            cover_cache: "".to_string(),
            cover_url: cover_url.to_string(),
        })
    }

    info_vec
}

fn extract_notice(document: &scraper::Html) -> Vec<Info> {
    let li_selector = scraper::Selector::parse("li.text_list_li").unwrap();
    let title_selector = scraper::Selector::parse(".a_tit").unwrap();
    // let date_selector = scraper::Selector::parse(".news_dt11").unwrap();
    // let summary_selector = scraper::Selector::parse(".news_summary").unwrap();
    let mut info_vec = Vec::new();

    for li in document.select(&li_selector) {
        let title = li
            .select(&title_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_else(|| "[无标题]".to_string())
            .trim()
            .to_string();

        // let date = li
        //     .select(&date_selector)
        //     .next()
        //     .map(|el| el.text().collect::<String>())
        //     .unwrap_or_else(|| "[无日期]".to_string())
        //     .trim()
        //     .to_string();

        // let summary = li
        //     .select(&summary_selector)
        //     .next()
        //     .map(|el| el.text().collect::<String>())
        //     .unwrap_or("[无摘要]".to_string())
        //     .trim()
        //     .to_string();

        let artical_url = li
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .unwrap_or("#");

        let artical_url = artical_url.replace("../", "https://www.jcu.edu.cn/");

        // let cover_url = li
        //     .select(&scraper::Selector::parse(".news_thumb img").unwrap())
        //     .next()
        //     .and_then(|img| img.value().attr("src"))
        //     .unwrap_or("#");

        let cover_url = format!("https://www.jcu.edu.cn{}", "not_a_picture");
        // println!("{}", title);
        // println!("   日期: {}", date);
        // println!("   摘要: {}", summary);
        // println!("   详情: {}\n", detail_url);

        info_vec.push(Info {
            title: title,
            department: "主站公告".to_string(),
            release_time: 1.0,
            artical_url: artical_url.to_string(),
            cover_cache: "".to_string(),
            cover_url: cover_url.to_string(),
        })
    }

    info_vec
}

fn extract_artical(document: &scraper::Html) -> String {
    let title_selector = scraper::Selector::parse("div.c_title").unwrap();
    // let release_time_selector = scraper::Selector::parse("").unwrap();
    let passages_selector = scraper::Selector::parse("div.v_news_content").unwrap();

    // some passages might need Selector::parse("p"), "https://www.jci.edu.cn/info/1055/50805.htm"
    let text_selector = scraper::Selector::parse("p span").unwrap();
    let img_selector = scraper::Selector::parse("img.img_vsb_content").unwrap();

    let title = document
        .select(&title_selector)
        .next()
        .map(|el| el.text().collect::<String>())
        .unwrap_or("[解析文章标题出错]".to_string());

    let release_time = "[解析文章发布时间出错]".to_string();

    let mut passages: Vec<String> = Vec::new();
    for passage in document
        .select(&passages_selector)
        .next()
        .unwrap()
        .child_elements()
    {
        if let Some(text) = passage.select(&text_selector).next() {
            let text = text.text().collect::<String>();
            passages.push(format!(r#"{{"type": "text", "content": "{text}"}}"#));
        } else if let Some(img) = passage.select(&img_selector).next() {
            let src = img.value().attr("src").unwrap().to_string();
            passages.push(format!(
                r#"{{"type": "image", "url": "https://www.jcu.edu.cn{src}"}}"#
            ));
        }
    }

    let passages = passages.join(", ");
    format!(
        r#"{{
        "title": "{title}",
        "releaseTime": "{release_time}",
        "passages": [{passages}]
        }}"#
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let state = AppState {
                settings: Settings {},
                channels: vec![
                    Arc::new(channel::ChannelMainNews::new()),
                    Arc::new(channel::ChannelMainNotice::new()),
                    Arc::new(channel::ChannelYlcNotice::new())
                ],
            };
            app.manage(Mutex::new(state));
            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_informations, get_artical, synchronize_channels])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
