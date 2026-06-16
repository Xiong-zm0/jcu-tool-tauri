import { initTheme, initDarkMode } from "./theme.js";

const invoke = window.__TAURI__.core.invoke;

let settings = null;
loadSettings();

function loadSettings() {
    invoke("load_settings").then((result) => {
        settings = result;
        initTheme(settings.theme);
        initDarkMode(settings.dark_mode);
        console.log("Settings loaded:", settings);
    }).catch((error) => {
        console.error("Failed to load settings:", error);
    });
}

export function updateSetting(key, value) {
    if (settings) {
        settings[key] = value;
        invoke("save_settings", { settings }).then(() => {
            console.log("Settings saved successfully.", settings);
        }).catch((error) => {
            console.error("Failed to save settings:", error);
        });
    } else {
        console.warn("Settings not loaded yet. Cannot update setting.");
    }
}
