import {NewAccount, create} from "/scripts/api/account.js";

document.querySelector("#form").addEventListener("submit", async (e) => {
    e.preventDefault();

    const username = document.querySelector("#username").value;
    const display_name = document.querySelector("#display-name").value;
    const password1 = document.querySelector("#password1").value;
    const password2 = document.querySelector("#password2").value;

    if (password1 !== password2) {
        alert("Passwords didn't match. Enter them again and retry.");
        return;
    }

    const account = new NewAccount(username, display_name, password1);
    const res = await create(account);
    if (res.ok) {
        window.location = "/index.html";
    } else {
        console.error(res.value);
        alert(`SignUp not successful: ${res.value}`);
    }
});
