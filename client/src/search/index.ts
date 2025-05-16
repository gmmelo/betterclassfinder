function displaySearch(): void {
    try {
        const search: string = window.location.search;
        const params: URLSearchParams = new URLSearchParams(search);

        const subject: string = params.get("subject") ?? "";
        const number: number = Number(params.get("number")) ?? -1;

        document.title = `Search: ${subject} ${number}`
        const search_title: HTMLElement = document.getElementById("search-title") ?? document.createElement("h1");
        search_title.innerHTML += `${subject} ${number}`;
    }
    catch(err) {
        console.log(`Couldn't display search results! ${err}`)
    }
}

document.addEventListener("DOMContentLoaded", displaySearch);