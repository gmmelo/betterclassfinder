type Class = {
    "CLAS":
        {
            "CLASSNBR": string,
            "SUBJECT": string,
            "CATALOGNBR": string,
            "TITLE": string,
            "INSTRUCTORSLIST": Array<string> | null,
            "ENRLTOT": string,
            "ENRLCAP": string
        }
}

type SearchResults = {
    "classes": Array<Class>;
}

function get_class_html(class_json: Class["CLAS"]): string {
    let inner_content: string;
    try {
        inner_content = `<p>Class: ${class_json["SUBJECT"]} ${class_json["CATALOGNBR"]}</p>`
            + `<p>Title: ${class_json["TITLE"]}</p>`
            + `<p>Instructor: ${class_json["INSTRUCTORSLIST"] == null ? "Not yet defined" : class_json["INSTRUCTORSLIST"][0]}</p>`
            + `<p>Seats taken: ${class_json["ENRLTOT"]}/${class_json["ENRLCAP"]}</p>`
            + `<p>#${class_json["CLASSNBR"]}</p>`;
    }
    catch(err) {
        console.log(`Error creating class listing: ${err}`);
        console.log(class_json);
        inner_content = "<p>Failed displaying this class!</p>";
    }
    const full_string: string = `<div class="class-listing">${inner_content}</div>`; 
    return full_string;
}

async function displaySearch(): Promise<void> {
    try {
        const search: string = window.location.search;
        const params: URLSearchParams = new URLSearchParams(search);

        const subject: string = params.get("subject") ?? "";
        const number: number = Number(params.get("number")) ?? -1;

        document.title = `Search: ${subject} ${number}`
        const search_title: HTMLElement = document.getElementById("search-title") ?? document.createElement("h1");
        search_title.innerHTML += `${subject} ${number}`;

        const url: string = `https://eadvs-cscc-catalog-api.apps.asu.edu/catalog-microservices/api/v1/search/classes?&refine=Y&campusOrOnlineSelection=A&catalogNbr=${number}&honors=F&promod=F&searchType=all&subject=${subject}&term=2257`;
        const response: SearchResults = await (await fetch(url, {headers: {"authorization": "Bearer null"}})).json();

        const search_results: HTMLElement = document.getElementById("search-results") ?? document.createElement("p");
        const class_list: Array<Class> = response["classes"];

        search_results.innerHTML = "";
        for (let i: number = 0; i < class_list.length; i++) {
            search_results.innerHTML += get_class_html(class_list[i]["CLAS"]);
        }
    }
    catch(err) {
        console.log(`Couldn't display search results! ${err}`);
    }
}

document.addEventListener("DOMContentLoaded", displaySearch);