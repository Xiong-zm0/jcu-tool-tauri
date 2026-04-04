mod cache;
mod settings;
mod article;
mod channel;

use std::sync::{Arc, Mutex};

use tauri::{self, Manager};

use settings::Settings;
use article::Article;

use crate::channel::ChannelType;


struct AppState {
    settings: Settings,
    channels: Vec<Arc<channel::Channel>>,
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

#[tauri::command]
async fn load_article(article: Article) -> Result<Article, String> {
    let article = channel::Channel::load_article(article).await;
    Ok(article)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let state = AppState {
                settings: Settings {},
                channels: vec![
                    Arc::new(channel::Channel::new(ChannelType::MainNews)),
                    Arc::new(channel::Channel::new(ChannelType::MainNotice)),
                ],
            };
            app.manage(Mutex::new(state));
            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_article, synchronize_channels])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
