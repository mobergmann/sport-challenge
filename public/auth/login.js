import {login} from "../api/auth.js";

document.getElementById("submit").addEventListener("click", async () => {
    const username = document.getElementById("username").value;
    const password = document.getElementById("password").value;

    try {
        const user = await login(username, password);
        window.location = "/home.html";
    } catch (error) {
        console.error(error);
        alert("SignIn not successful. Try again");
    }
});
