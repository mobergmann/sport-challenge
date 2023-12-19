import {BASE_USERS_URL, Response, STATUS} from "./main.js";

export class User {
    constructor(id, username) {
        this.id = id;
        this.username = username;
    }
}

/// get the public profile of a user
/// :param username the username of the user
/// :return `User`
export async function get(username) {
    const request = new Request(`${BASE_USERS_URL}/${username}`, {
        method: "GET",
        credentials: 'include',
    });

    let response = await fetch(request);
    if (response.status === STATUS.OK) {
        let raw = response.body.json();
        response = new Response(response.status, new User(raw, raw));
    } else if (response.status === STATUS.CONFLICT) {

    } else if (response.status === STATUS.NOT_FOUND) {

    }

    try {
        return response.json();
    } catch (error) {
        return error;
    }
}

/// get the public profile of a user
/// :param id id of the user
/// :return `User`
export async function get_id(id) {
    const request = new Request(`${BASE_USERS_URL}/id/${id}`, {
        method: "GET",
        credentials: 'include',
    });

    try {
        let response = await fetch(request);
        return response.json();
    } catch (error) {
        return error;
    }
}
