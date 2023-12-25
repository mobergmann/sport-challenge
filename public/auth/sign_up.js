import {sign_up} from "/scripts/auth.js";

document.querySelector("#form").addEventListener("submit", async (e) => {
    e.preventDefault();

    const username = document.querySelector("#username").value;
    const password1 = document.querySelector("#password1").value;
    const password2 = document.querySelector("#password2").value;

    if (password1 !== password2) {
        alert("Passwords didn't match. Enter them again and retry.");
        return;
    }

    try {
        const user = await sign_up(username, password1);
        window.location = "/auth/login.html";
    } catch (error) {
        console.error(error);
        alert("SignUp not successful. Please try again.");
    }
});
