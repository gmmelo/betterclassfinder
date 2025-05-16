"use strict";
function get_class_html(class_json) {
    let inner_content;
    try {
        inner_content = `<p>Class: ${class_json["SUBJECT"]} ${class_json["CATALOGNBR"]}</p>`
            + `<p>Title: ${class_json["TITLE"]}</p>`
            + `<p>Instructor: ${class_json["INSTRUCTORSLIST"] == null ? "Not yet defined" : class_json["INSTRUCTORSLIST"][0]}</p>`
            + `<p>Seats taken: ${class_json["ENRLTOT"]}/${class_json["ENRLCAP"]}</p>`
            + `<p>#${class_json["CLASSNBR"]}</p>`;
    }
    catch (err) {
        console.log(`Error creating class listing: ${err}`);
        console.log(class_json);
        inner_content = "<p>Failed displaying this class!</p>";
    }
    const full_string = `<div class="class-listing">${inner_content}</div>`;
    return full_string;
}
async function displaySearch() {
    try {
        const search = window.location.search;
        const params = new URLSearchParams(search);
        const subject = params.get("subject") ?? "";
        const number = Number(params.get("number")) ?? -1;
        document.title = `Search: ${subject} ${number}`;
        const search_title = document.getElementById("search-title") ?? document.createElement("h1");
        search_title.innerHTML += `${subject} ${number}`;
        const url = `https://eadvs-cscc-catalog-api.apps.asu.edu/catalog-microservices/api/v1/search/classes?&refine=Y&campusOrOnlineSelection=A&catalogNbr=${number}&honors=F&promod=F&searchType=all&subject=${subject}&term=2257`;
        const response = await (await fetch(url, { headers: { "authorization": "Bearer null" } })).json();
        const search_results = document.getElementById("search-results") ?? document.createElement("p");
        const class_list = response["classes"];
        search_results.innerHTML = "";
        for (let i = 0; i < class_list.length; i++) {
            search_results.innerHTML += get_class_html(class_list[i]["CLAS"]);
        }
    }
    catch (err) {
        console.log(`Couldn't display search results! ${err}`);
    }
}
document.addEventListener("DOMContentLoaded", displaySearch);
