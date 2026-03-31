use std::path::PathBuf;
use tauri::path::BaseDirectory;

use crate::article::Article;

struct CacheManager {
    cache_path: PathBuf,
    max_artical: usize,
    max_image: usize,
}

impl CacheManager {
    fn new(max_artical: usize, max_image: usize) -> Self {
        Self {
            cache_path: BaseDirectory::AppCache.variable().into(),
            max_artical,
            max_image,
        }
    }

    fn cache_artical(&self, artical: &Article) -> String {
        todo!();
    }

    fn cache_image(&self, image_url: &str) -> String {
        todo!();
    }
}
