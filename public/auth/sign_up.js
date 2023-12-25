<<<<<<< HEAD
import {create as create_account} from "/scipts/api/account.js";

document.getElementById("submit").addEventListener("click", async () => {
    const username = document.getElementById("username").value;

    // check for password equality
    const password1 = document.getElementById("password1").value;
    const password2 = document.getElementById("password2").value;
    if (password1 !== password2) {
        alert("Passwords do not match!");
        document.getElementById("sign_up-form").reset();
    }

    try {
        await create_account(username, password1);
        window.location = "/auth/sign_in.html";
    } catch (_) {
        alert("SignUp not successful");
=======
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
>>>>>>> main
    }
});
