use std::path::PathBuf;
use tauri::path::BaseDirectory;

use crate::article::Article;

struct CacheManager {
    cache_path: PathBuf,
    max_article: usize,
    max_image: usize,
}

impl CacheManager {
    fn new(max_article: usize, max_image: usize) -> Self {
        Self {
            cache_path: BaseDirectory::AppCache.variable().into(),
            max_article,
            max_image,
        }
    }

    fn cache_article(&self, article: &Article) -> String {
        todo!();
    }

    fn cache_image(&self, image_url: &str) -> String {
        todo!();
    }
}
