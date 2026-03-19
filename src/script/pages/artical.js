const { invoke } = window.__TAURI__.core;

export async function constructArtical(src) {
    let artical = await getArtical(src);

    let articalTrackNode = document.createElement("div");
    articalTrackNode.setAttribute("class", "carousel-track");
    articalTrackNode.setAttribute("id", "artical-carousel-track");

    let articalNode = document.createElement("div");
    articalNode.setAttribute("class", "artical-page card");

    let titleNode = document.createElement("h1");
    titleNode.innerText = artical.title;
    articalNode.appendChild(titleNode);

    let releaseTimeNode = document.createElement("h6");
    releaseTimeNode.innerText = artical.releaseTime;
    articalNode.appendChild(releaseTimeNode);

    for (let passage of artical.passages) {
        let passageNode;
        if (passage.type === "text") {
            passageNode = document.createElement("p");
            passageNode.innerText = passage.content;
        } else if (passage.type === "image") {
            passageNode = document.createElement("img");
            passageNode.setAttribute("src", passage.url);
        } else {

        }
        articalNode.appendChild(passageNode);
    }

    articalTrackNode.appendChild(articalNode);

    return articalTrackNode;
}

async function getArtical(url) {
    let resolt = await invoke("get_artical", {url: url});
    console.log(JSON.parse(resolt));
    return JSON.parse(resolt);
    resolt = {
        "title": "校党委书记蒲守智检查指导新学期教学工作",
        "releaseTime": "2026-03-02",
        "passages": [
            {
                "type": "text",
                "content": "本网讯（党委宣传部）春启新程，教启新篇。3月2日上午，校党委书记蒲守智，党委常委、副校长陈云霞，党委常委、副校长黄勇带队深入学校湘湖校区、新厂校区教学一线，实地检查指导新学期教学工作。",
            },{
                "type": "image",
                "url": "https://www.jcu.edu.cn/__local/0/C7/26/98326CB55E3E7B8818FFA85F2D5_D7444E6A_3B5C79.jpg",
            },{
                "type": "image",
                "url": "https://www.jcu.edu.cn/__local/E/06/46/4185734894C41313E54288BD9B3_82AABDBC_90ABEF.jpg",
            },{
                "type": "image",
                "url": "https://www.jcu.edu.cn/__local/C/05/6B/585EE56E6E7878EBEA974D812E0_5DB45AD1_5CF0D.png",
            },{
                "type": "image",
                "url": "https://www.jcu.edu.cn/__local/5/FE/50/C6A2EF6A07E8B97CDC375F91567_B409BB34_2BBA6B.jpg",
            },{
                "type": "image",
                "url": "https://www.jcu.edu.cn/__local/1/5E/BB/A5FBCDB7236EF3A47B6653DB5CC_0D9B6567_397A2E.jpg",
            },{
                "type": "text",
                "content": "蒲守智强调，课堂教学是人才培养的主阵地，各部门要提高政治站位，强化责任担当，抓实抓细教学各环节管理，及时排查解决教学运行中的各类问题。他要求，授课教师要坚守育人初心，深耕教学一线，不断创新教学方法，提升课堂教学实效；各职能部门要加强协同联动，抓好教学设施保障和逐步更新、校园安全防护等工作，全力为教学工作保驾护航，为2026年学校扎实推进“内涵质量提升年”建设开好局、起好步。"
            },{
                "type": "text",
                "content": "党委（校长）办公室、党委学生工作部（党委研究生工作部、学生工作处）、研究生院、教务处（教育评估与评估中心）、保卫处、后勤管理处、现代教育技术中心等相关部门负责人随同检查。"
            },{
                "type": "text",
                "content": "（责任编辑：刘欢 审稿：兰茜 刘欢）"
            }
        ]
    }
    return resolt;
}