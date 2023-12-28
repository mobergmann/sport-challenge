import {get} from "/scripts/api/account.js"

let res = await get();
if (res.ok) {
    document.querySelector("#dashboard-link").style.display = "block";
}
