const { invoke } = window.__TAURI__.core;

import { pushNewPage } from "../pageManager.js";
import { constructArtical } from "./artical.js";

export async function loadInformation() {
    const pageNode = document.querySelector(".information-page");
    let informationBunch = await fetchInformation("ancient");
    if (informationBunch.length > 0) {
        const ancientInformationGroup = constructGroup("information-ancient");
        pageNode.appendChild(ancientInformationGroup);
        for (let information of informationBunch) {
            let informationCard = constructInformationCard(information);
            ancientInformationGroup.appendChild(informationCard);
        }

        informationBunch = await fetchInformation("new");
        if (informationBunch.length > 0) {
            pageNode.prepend(constructSplitLine("更早的内容"));
            const newInformationGroup = constructGroup("information-new");
            pageNode.prepend(newInformationGroup);
            for (let information of informationBunch) {
                let informationCard = constructInformationCard(information);
                newInformationGroup.appendChild(informationCard);
            }
        }
    } else {
        informationBunch = await fetchInformation("new");
        if (informationBunch.length > 0) {
            const newInformationGroup = constructGroup("information-new");
            pageNode.prepend(newInformationGroup);
            for (let information of informationBunch) {
                let informationCard = constructInformationCard(information);
                newInformationGroup.appendChild(informationCard);
            }
        } else {
            console.log("网络似乎有点问题");
        }
    }
}

async function fetchInformation(dataEpochs) {
    if (dataEpochs == "new") {
        let informations = await invoke("synchronize_channels");
        console.log(informations);
        return informations;
    } else if (dataEpochs == "ancient") {
        return [
            {
                "title": "校党委常委会专题学习二十届中央纪委五次全会和省纪委十五届六次全会精神66666 6666 666666 666666666",
                "coverCache": "stroge/A2F26E17EF1F823AD3F6A0CC399_FCC9A8E6_47A5CA.jpg",
                "coverUrl": "https://www.jcu.edu.cn/__local/8/A9/B7/A2F26E17EF1F823AD3F6A0CC399_FCC9A8E6_47A5CA.jpg",
                "articalURL": "https://www.jcu.edu.cn/info/1056/50823.htm",
                "department": "景德镇陶瓷大学",
                "releaseTime": 6546546.187,
            }, {
                "title": "我校在全省首届教育系统安全素养大赛中喜获佳绩",
                "coverCache": "stroge/BF2D97B39ABB189D15C51F3EA16_F9E0B9C4_393025.jpg",
                "coverUrl": "https://www.jcu.edu.cn/__local/C/FE/96/BF2D97B39ABB189D15C51F3EA16_F9E0B9C4_393025.jpg",
                "department": "景德镇陶瓷大学",
                "releaseTime": 6546546.187,
            }
        ];
    } else {
        console.error(dataEpochs, "is wrong epochs");
    }
}

function constructGroup(id) {
    let group = document.createElement("div");
    group.setAttribute("class", "information-card-group");
    group.setAttribute("id", id);
    return group;
}

function constructInformationCard(information) {
    let informationCard = document.createElement("button");
    informationCard.setAttribute("class", "information-card card");
    informationCard.addEventListener("click", async () => {
        let articalNodeConstructor = () => constructArtical(information.url);
        pushNewPage(articalNodeConstructor, "#navigation-news");
    });

    let coverCache = document.createElement("div");
    coverCache.setAttribute("class", "information-card__cover");
    let cover = document.createElement("div");
    cover.setAttribute("class", "information-card__cover");

    let coverImg = document.createElement("img");
    coverImg.setAttribute("src", information.cover);
    coverImg.setAttribute("alt", " ");
    coverImg.style.opacity = 0;
    coverImg.onload = (_event) => {
        coverImg.style.opacity = 1;
    };

    let title = document.createElement("div");
    title.setAttribute("class", "information-card__title");
    title.innerText = information.title;
    let department = document.createElement("div");
    department.setAttribute("class", "information-card__department");
    department.innerText = information.signature;
    let timeTip = document.createElement("div");
    timeTip.setAttribute("class", "information-card__timetip");
    timeTip.innerText = information.release_time;

    cover.appendChild(coverImg);
    informationCard.appendChild(coverCache);
    informationCard.appendChild(cover);
    informationCard.appendChild(title);
    informationCard.appendChild(department);
    informationCard.appendChild(timeTip);
    return informationCard;
}

function constructSplitLine(text) {
    let groupSplitLine = document.createElement("div");
    groupSplitLine.setAttribute("class", "group-split-line");

    let lineLeft = document.createElement("div");
    lineLeft.setAttribute("class", "group-split-line__line");
    let lineRight = document.createElement("div");
    lineRight.setAttribute("class", "group-split-line__line");

    let textDiv = document.createElement("div");
    textDiv.class = "group-split-line__text";
    textDiv.innerText = text;

    groupSplitLine.appendChild(lineLeft);
    groupSplitLine.appendChild(textDiv);
    groupSplitLine.appendChild(lineRight);
    return groupSplitLine;
}