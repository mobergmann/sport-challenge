import {sign_in} from "/scripts/auth.js";

document.querySelector("#form").addEventListener("submit", async (e) => {
    e.preventDefault();

    const username = document.querySelector("#username").value;
    const password = document.querySelector("#password").value;

    try {
        const user = await sign_in(username, password);
        window.location = "/home.html";
    } catch (error) {
        console.error(error);
        alert("Login not successful. Please try again");
    }
});
