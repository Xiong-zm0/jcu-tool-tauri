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

initMainView();
bindBackButton();
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
        const justActivedTrackGroups = trackGroups[activeId];
        if (justActivedTrackGroups.level > 1) {
            justActivedTrackGroups.level -= 1;
            const lastSecondElement = justActivedTrackGroups.trackGroup.children[justActivedTrackGroups.level-1];
            justActivedTrackGroups.trackGroup.lastElementChild.setAttribute("animation", "leave");
            lastSecondElement.setAttribute("animation", "display");
            justActivedTrackGroups.trackGroup.lastElementChild.remove();
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
    const oldLastElement = trackGroups[targetTrack].trackGroup.children[trackGroups[targetTrack].level-1];
    oldLastElement.setAttribute("animation", "covered");

    let page = await pageConstructor();
    trackGroups[targetTrack].trackGroup.appendChild(page);
    trackGroups[targetTrack].level += 1;
    const newLastElement = trackGroups[targetTrack].trackGroup.children[trackGroups[targetTrack].level-1];
    requestAnimationFrame(() => {
        requestAnimationFrame(() => {
            newLastElement.setAttribute("animation", "display");
        });
    });
}