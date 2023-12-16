import {create as create_account} from "../api/account.js";

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
    }
});
