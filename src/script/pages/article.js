const { invoke } = window.__TAURI__.core;

export async function constructArticle(article) {
    article = await invoke("load_article", {article: article});

    let articleTrackNode = document.createElement("div");
    articleTrackNode.setAttribute("class", "carousel-track");
    articleTrackNode.setAttribute("id", "article-carousel-track");

    let articleNode = document.createElement("div");
    articleNode.setAttribute("class", "article-page card");

    let titleNode = document.createElement("h1");
    titleNode.innerText = article.title;
    articleNode.appendChild(titleNode);

    let releaseTimeNode = document.createElement("h6");
    releaseTimeNode.innerText = article.release_time;
    articleNode.appendChild(releaseTimeNode);

    for (let passage of article.passages) {
        let passageNode;
        if (passage.Text && passage.Text.length > 0) {
            console.log(passage.Text);
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

