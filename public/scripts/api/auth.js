import {BASE_AUTH_URL, STATUS, Response} from "./main.js";
import {Account} from "./account.js";

/// login, creation a session and saving the session by setting a cookie
/// :param username username of the user
/// :param password password of the account
/// :return `Account` the account object of the now logged in user
export async function login(username, password) {
    const request = new Request(`${BASE_AUTH_URL}/login`, {
        method: "POST",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
        body: JSON.stringify({username: username, password: password}),
        credentials: 'include',
    });

    let response = await fetch(request);
    if (response.status === STATUS.OK) {
        let raw = response.json();
        return new Response(STATUS.OK, new Account(id, username, password_hash));
    } else {
        return new Response(STATUS.OK, new Account(raw));
    }
}

/// log the current session out
export async function logout() {
    const request = new Request(`${BASE_AUTH_URL}/logout`, {
        method: "PUT",
        credentials: 'include',
    });

    try {
        let response = await fetch(request);
        return response.json();
    } catch (error) {
        return error;
    }
}
