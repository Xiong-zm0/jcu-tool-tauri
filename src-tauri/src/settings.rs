use tauri_plugin_store::{self, StoreExt};

use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    theme: Theme,
    dark_mode: DarkMode,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Theme {
    Blue,
    Green,
    Red,
    Pink,
    Purple,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DarkMode {
    Auto,
    Enabled,
    Disabled,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::Blue,
            dark_mode: DarkMode::Auto,
        }
    }
}

impl Settings {
    pub fn load(app: &tauri::App) -> Self {
        println!("Loading settings...");
        let mut settings = Self::default();
        if let Ok(store) = app.store("settings.json") {
            println!("Settings store loaded successfully.");
            if let Some(theme) = store.get("theme") {
                println!("Loaded theme: {}", theme);
                let theme = match theme.as_str() {
                    Some("Blue") => Theme::Blue,
                    Some("Green") => Theme::Green,
                    Some("Red") => Theme::Red,
                    Some("Pink") => Theme::Pink,
                    Some("Purple") => Theme::Purple,
                    _ => Theme::Blue,
                }; 
                settings.set_theme(theme);
            }
            if let Some(dark_mode) = store.get("dark_mode") {
                println!("Loaded dark mode: {}", dark_mode);
                let dark_mode = match dark_mode.as_str() {
                    Some("Auto") => DarkMode::Auto,
                    Some("Enabled") => DarkMode::Enabled,
                    Some("Disabled") => DarkMode::Disabled,
                    _ => DarkMode::Auto,
                };
                settings.set_dark_mode(dark_mode);
            }
        }
        settings
    }

    pub fn store(&self, app: &tauri::AppHandle) {
        println!("Storing settings...");
        if let Ok(mut store) = app.store("settings.json") {
            println!("Settings store loaded successfully for storing.");
            store.set("theme", match self.theme {
                Theme::Blue => "Blue",
                Theme::Green => "Green",
                Theme::Red => "Red",
                Theme::Pink => "Pink",
                Theme::Purple => "Purple",
            });
            store.set("dark_mode", match self.dark_mode {
                DarkMode::Auto => "Auto",
                DarkMode::Enabled => "Enabled",
                DarkMode::Disabled => "Disabled",
            });
            println!("Settings stored successfully.");
        } else {
            println!("Failed to load settings store for storing.");
        }
    }

    fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    fn set_dark_mode(&mut self, dark_mode: DarkMode) {
        self.dark_mode = dark_mode;
    }
}