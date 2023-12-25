import {BASE_AUTH_URL} from "./variables.js";
import {do_request} from "./requests.js";

/// sign up a user
export async  function sign_up(username, password) {
    const user = {
        name: username,
        password: password,
    };

    const request = new Request(`${BASE_AUTH_URL}/sign_up`, {
        method: "POST",
        body: JSON.stringify(user),
        headers: new Headers({
            "Content-Type": "application/json",
        }),
    });

    return await fetch(request)
        .then((response) => {
            if (response.status === 201) {
                return response.json();
            } else {
                throw new Error("Something went wrong on API server!");
            }
        })
        .then((response) => {
            console.debug("Sucessfull SignUp");
            console.debug(response);
            return response;
        })
        .catch((error) => {
            console.error(error);
            throw error;
        });
}

/// sign in a user
export async  function sign_in(username, password) {
    const user = {
        name: username,
        password: password,
    };

    const request = new Request(`${BASE_AUTH_URL}/sign_in`, {
        method: "POST",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
        body: JSON.stringify(user),
        credentials: 'include',
    });

    return await do_request(request);
}

/// sign out a user
export async function sign_out() {
    const request = new Request(`${BASE_AUTH_URL}/sign_out`, {
        method: "GET",
        credentials: 'include',
    });

    return await fetch(request)
        .then((response) => {
            if (response.status === 200) {
                return response;
            } else {
                throw new Error("Something went wrong on API server!");
            }
        })
        .then((response) => {
            console.debug("Sucessfull SignOut");
            console.debug(response);
            return response;
        })
        .catch((error) => {
            console.error(error);
            throw error;
        });
}
