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
    document.querySelectorAll("#navigation-back")
        .forEach((element) => {
            element.addEventListener("click", () => {
                const justActivedTrackGroups = trackGroups[activeId];
                if (justActivedTrackGroups.level > 1) {
                    justActivedTrackGroups.level -= 1;
                    const lastSecondElement = justActivedTrackGroups.trackGroup.children[justActivedTrackGroups.level-1];
                    justActivedTrackGroups.trackGroup.lastElementChild.setAttribute("animation", "leave");
                    lastSecondElement.setAttribute("animation", "display");
                    justActivedTrackGroups.trackGroup.lastElementChild.remove();
                }
            })
        })
}

function bindTrackManagerButton() {
    for (let buttonId of Object.keys(trackGroups)) {
        document.querySelectorAll(buttonId).forEach((element) => {
            element.setAttribute("aria-pressed", "false");
            element.addEventListener("click", () => changePageStake(buttonId));
        })
    }

    document
        .querySelectorAll("#navigation-news")
        .forEach((element) => {
            element.setAttribute("aria-pressed", "true");
        })
}

function changePageStake(buttonId) {
    trackGroups[activeId].trackGroup.setAttribute("hidden", "");
    document.querySelectorAll(activeId).forEach((element) => {
        element.setAttribute("aria-pressed", "false")
    });
    activeId = buttonId;
    trackGroups[activeId].trackGroup.removeAttribute("hidden");
    document.querySelectorAll(activeId).forEach((element) => {
        element.setAttribute("aria-pressed", "true");
    });
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