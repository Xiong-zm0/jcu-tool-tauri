const { invoke } = window.__TAURI__.core;

export async function constructArticle(article, resetTime) {
    // if infoCard set a information card node, will try to sync
    // new information on the card.
    article = await invoke("load_article", {article: article});

    let articleTrackNode = document.createElement("div");
    articleTrackNode.setAttribute("class", "carousel-track");
    articleTrackNode.setAttribute("id", "article-carousel-track");

    let articleNode = document.createElement("div");
    articleNode.setAttribute("class", "article-page card");

    let titleNode = document.createElement("h1");
    titleNode.innerText = article.title;
    articleNode.appendChild(titleNode);

    let timeString;
    if (article.release_time == null) {
        timeString = "-";
    } else {
        const beijingTime = new Date(article.release_time * 1000);
        const year = beijingTime.getFullYear();
        const month = beijingTime.getMonth() + 1;
        const day = beijingTime.getDate();
        timeString = `${year}/${month}/${day}`;
    }
    let releaseTimeNode = document.createElement("h6");
    releaseTimeNode.innerText = timeString;
    articleNode.appendChild(releaseTimeNode);
    if (resetTime) {
        resetTime(timeString);
    }

    for (let passage of article.passages) {
        let passageNode;
        if (passage.Text && passage.Text.length > 0) {
            passageNode = document.createElement("p");
            constructTextPassage(passage, passageNode);
        } else if (passage.Image != null) {
            passageNode = document.createElement("img");
            passageNode.setAttribute("src", passage.Image.url);
            passageNode.setAttribute("alt", " ");
        }
        articleNode.appendChild(passageNode);
    }
    articleTrackNode.appendChild(articleNode);

    return articleTrackNode;
}

function constructTextPassage(passage, passageNode) {
    for (let textSegment of passage.Text) {
        if (textSegment.style.includes("Right")) {
            passageNode.setAttribute("class", "align-right");
        }

        if (textSegment.style.includes("Bold")) {
            let boldNode = document.createElement("strong");
            boldNode.innerText = textSegment.content;
            passageNode.appendChild(boldNode);
        } else {
            let textNode = document.createElement("span");
            textNode.innerText = textSegment.content;
            passageNode.appendChild(textNode);
        }
    }
}

