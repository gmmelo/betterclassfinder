"use strict";
function displaySearch() {
    try {
        const search = window.location.search;
        const params = new URLSearchParams(search);
        const subject = params.get("subject") ?? "";
        const number = Number(params.get("number")) ?? -1;
        document.title = `Search: ${subject} ${number}`;
        const search_title = document.getElementById("search-title") ?? document.createElement("h1");
        search_title.innerHTML += `${subject} ${number}`;
    }
    catch (err) {
        console.log(`Couldn't display search results! ${err}`);
    }
}
document.addEventListener("DOMContentLoaded", displaySearch);
