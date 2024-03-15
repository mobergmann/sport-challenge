import {BASE_AUTH_URL, STATUS, Result} from "./main.js";
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
        let value = await response.json();
        return new Result(true, new Account(value.id, value.username, value.display_name, value.password_hash));
    } else {
        let error = await response.text();
        return new Result(false, error);
    }
}

/// log the current session out
export async function logout() {
    const request = new Request(`${BASE_AUTH_URL}/logout`, {
        method: "PUT",
        credentials: 'include',
    });

    let response = await fetch(request);
    if (response.status === STATUS.OK) {
        let value = await response.json();
        return new Result(true, undefined);
//        return new Result(true, new Account(value.id, value.username, value.password_hash));
    } else {
        let error = await response.text();
        return new Result(new Result(false, error));
    }
}
