import {login} from "/scripts/api/auth.js";

document.querySelector("#form").addEventListener("submit", async (e) => {
    e.preventDefault();

    const username = document.querySelector("#username").value;
    const password = document.querySelector("#password").value;

    const res = await login(username, password);
    if (res.ok) {
        window.location = "/index.html";
    } else {
        console.error(res.value);
        alert(`Login not successful: ${res.value}`);
    }
});
