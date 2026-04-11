import { pushNewPage } from "../pageManager.js";

export function loadSetting() {
    constructSettingPage();
}

function constructSettingPage() {
    const pageNode = document.querySelector(".setting-page");
    pageNode.appendChild(constructSection("显示", [
        constructButton("主题色", "", () => constructDisplaySetting()),
        constructButton("安全区", "", () => constructDisplaySetting()),
        constructButton("详细设置", "", () => constructDisplaySetting()),
    ]));
    pageNode.appendChild(constructSection("存储", [
        constructButton("启用缓存", ""),
        constructButton("详细设置", ""),
    ]));
    pageNode.appendChild(constructSection("新闻页", [
        constructButton("高亮关键词", ""),
        constructButton("替换关键词", ""),
        constructButton("详细设置", ""),
    ]));
    pageNode.appendChild(constructSection("搜索页", [
        constructButton("详细设置", ""),
    ]));
}

function constructDisplaySetting() {
    pushNewPage(displaySettingConstructor, "#navigation-setting");
}

function displaySettingConstructor() {
    const displayPageTrackNode = document.createElement("div");
    displayPageTrackNode.setAttribute("class", "carousel-track");
    displayPageTrackNode.setAttribute("id", "dispaly-setting-track")

    const displayPageNode = document.createElement("div");
    displayPageNode.setAttribute("class", "setting-page");

    function setThemeColor(color) {
        document.documentElement.style.setProperty("--theme-color", color);
    }

    displayPageNode.appendChild(constructSection("夜间模式", [
        constructButton("跟随系统", "", ),
        constructButton("明亮", "", ),
        constructButton("暗色", "", ),
    ]))

    displayPageNode.appendChild(constructSection("主题色", [
        constructButton("陶瓷蓝", "", () => setThemeColor("#1e59a8")),
        constructButton("哔哩粉", "", () => setThemeColor("#fb7299")),
        constructButton("书小红", "", () => setThemeColor("#ff2e4d")),
        constructButton("巨信绿", "", () => setThemeColor("#1aad19")),
        constructButton("清华紫", "", () => setThemeColor("#6703b1")),
        constructButton("自定义", "", ),
    ]));

    displayPageNode.appendChild(constructSection("安全区", [
        constructButton("顶部", "", ),
        constructButton("底部", "", ),
        constructButton("左侧", "", ),
        constructButton("右侧", "", ),
    ]));

    displayPageTrackNode.appendChild(displayPageNode);
    return displayPageTrackNode;
}

function constructSection(sectionTitle, items) {
    const settingNode = document.createElement("div");
    settingNode.setAttribute("class", "setting-section card");

    const sectionTitleNode = document.createElement("div");
    sectionTitleNode.setAttribute("class", "setting-section__title");
    sectionTitleNode.innerText = sectionTitle;

    settingNode.appendChild(sectionTitleNode);
    settingNode.append(...items);

    return settingNode;
}

function constructButton(name, type, callback) {
    const buttonNode = document.createElement("button");
    buttonNode.setAttribute("class", "setting-section__item");
    buttonNode.addEventListener("click", () => callback());

    const titleNode = document.createElement("div");
    titleNode.setAttribute("class", "item__title");
    titleNode.innerText = name;

    buttonNode.appendChild(titleNode);

    return buttonNode;
}