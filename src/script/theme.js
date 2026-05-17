import { updateSetting } from "./settings.js";

export const themeDict = {
    "Blue": {
        "--primary-color": "#1e59a8"
    },
    "Pink": {
        "--primary-color": "#fb7299"
    },
    "Red": {
        "--primary-color": "#ff2e4d"
    },
    "Green": {
        "--primary-color": "#1aad19"
    },
    "Purple": {
        "--primary-color": "#6703b1"
    }
}

export function initTheme(themeKey) {
    document.documentElement.style.setProperty("--primary-color", themeDict[themeKey]["--primary-color"]);
}

export function initDarkMode(darkModeKey) {
    if (darkModeKey === "Auto") {
        const prefersDarkScheme = window.matchMedia("(prefers-color-scheme: dark)");
        applyDarkClass(prefersDarkScheme.matches);
        prefersDarkScheme.addEventListener("change", (e) => {
            console.log("Dark mode preference changed:", e.matches);
            applyDarkClass(e.matches);
        });
        console.log("set up auto dark mode with prefers-color-scheme");
    } else {
        applyDarkClass(darkModeKey === "Enabled");
    }
}

function applyDarkClass(enabled) {
    console.log("Applying dark mode:", enabled);
    if (enabled) {
        document.documentElement.classList.add("dark");
    } else {
        document.documentElement.classList.remove("dark");
    }
}