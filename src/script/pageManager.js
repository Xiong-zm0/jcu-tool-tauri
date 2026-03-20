import { loadInformation } from "./pages/information.js";
import { loadSetting } from "./pages/setting.js";

let trackGroups = {
    "#navigation-news": {
        level: 1,
        trackGroup: document.querySelector("#news-track-group"),
    },
    "#navigation-search": {
        level: 1,
        trackGroup: document.querySelector("#search-track-group"),
    },
    "#navigation-star": {
        level: 1,
        trackGroup: document.querySelector("#star-track-group"),
    },
    "#navigation-setting": {
        level: 1,
        trackGroup: document.querySelector("#setting-track-group"),
    },
}

let activeId = "#navigation-news";

bindBackButton();
initMainView();
bindTrackManagerButton();

function initMainView() {
    for (let trackGroup of Object.values(trackGroups)) {
        trackGroup.trackGroup.setAttribute("hidden", "");
    }
    trackGroups[activeId].trackGroup.removeAttribute("hidden");
    loadInformation();
}

function bindBackButton() {
    let backButton = document.querySelector("#navigation-back");
    backButton.addEventListener("click", () => {
        if (trackGroups[activeId].level > 1) {
            trackGroups[activeId].level -= 1;
            const lastSecondElement = trackGroups[activeId].trackGroup.children[trackGroups[activeId].level-1];
            trackGroups[activeId].trackGroup.lastElementChild.setAttribute("animation", "right");
            setTimeout(() => {
                lastSecondElement.setAttribute("animation", "center");
            }, 200);
            setTimeout(() => {
                trackGroups[activeId].trackGroup.lastElementChild.remove();
            }, 500);
        }
    })
}

function bindTrackManagerButton() {
    for (let buttonId of Object.keys(trackGroups)) {
        let button = document.querySelector(buttonId);
        button.setAttribute("aria-pressed", "false");
        button.addEventListener("click", () => changePageStake(buttonId))
    }

    document
        .querySelector("#navigation-news")
        .setAttribute("aria-pressed", "true");
}

function changePageStake(buttonId) {
    console.log("pressed");
    trackGroups[activeId].trackGroup.setAttribute("hidden", "");
    document.querySelector(activeId).setAttribute("aria-pressed", "false");
    activeId = buttonId;
    trackGroups[activeId].trackGroup.removeAttribute("hidden");
    document.querySelector(activeId).setAttribute("aria-pressed", "true");
}

export async function pushNewPage(pageConstructor, targetTrack) {
    trackGroups[targetTrack].trackGroup.lastElementChild.setAttribute("animation", "left");
    let page = await pageConstructor();
    trackGroups[targetTrack].trackGroup.appendChild(page);
    setTimeout(() => {
        trackGroups[targetTrack].trackGroup.lastElementChild.setAttribute("animation", "center");
    }, 200);
    trackGroups[targetTrack].level += 1;
}