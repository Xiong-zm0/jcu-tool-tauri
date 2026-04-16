const invoke = window.__TAURI__.core.invoke;


let settings = null;
loadSettings();

function loadSettings() {
    invoke("load_settings").then((result) => {
        settings = result;
        console.log("Settings loaded:", settings);
    }).catch((error) => {
        console.error("Failed to load settings:", error);
    });
}

function saveSettings() {
    if (settings) {
        invoke("save_settings", { settings }).then(() => {
            console.log("Settings saved successfully.");
        }).catch((error) => {
            console.error("Failed to save settings:", error);
        });
    } else {
        console.warn("No settings to save.");
    }
}