export function loadSetting() {
    const pageNode = document.querySelector(".setting-page");
    pageNode.appendChild(constructSection("显示", [
        constructButton("主题色", ""),
        constructButton("安全区", ""),
        constructButton("详细设置", ""),
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

function constructButton(name, type) {
    const buttonNode = document.createElement("button");
    buttonNode.setAttribute("class", "setting-section__item");

    const titleNode = document.createElement("div");
    titleNode.setAttribute("class", "item__title");
    titleNode.innerText = name;

    buttonNode.appendChild(titleNode);

    return buttonNode;
}