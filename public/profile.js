import {EditAccount, EditPassword, edit, edit_password} from "/scripts/api/account.js";

document.querySelector("#form-account").addEventListener("submit", async (e) => {
    e.preventDefault();

    const username = document.querySelector("#username").value;

    const account_update = new EditAccount(username);
    const res = await edit(account_update);
    if (res.ok) {
        window.location = "/home.html";
    } else {
        console.error(res.value);
        alert(`Action not successful: ${res.value}`);
    }
});

document.querySelector("#form-credential").addEventListener("submit", async (e) => {
    e.preventDefault();

    const current_password = document.querySelector("#current-password").value;
    const new_password1 = document.querySelector("#new-password1").value;
    const new_password2 = document.querySelector("#new-password2").value;

    if (new_password1 !== new_password2) {
        alert("Passwords didn't match. Enter them again and retry.");
        return;
    }

    const password_update = new EditPassword(current_password, new_password1);
    const res = await edit_password(password_update);
    if (res.ok) {
        window.location = "/home.html";
    } else {
        console.error(res.value);
        alert(`Action not successful: ${res.value}`);
    }
});
