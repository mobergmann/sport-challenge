import {get} from "/scripts/api/account.js"

let res = await get();
if (res.ok) {
    let links = document.querySelectorAll(".dashboard-link");

    for (const elem of links) {
        elem.style.display = "block";
    }
}
