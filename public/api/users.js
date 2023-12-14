import {BASE_USERS_URL} from "./main.js";

export class User {
    constructor(id, username) {
        self.id = id;
        self.username = username;
    }
}

/// get the public profile of a user
/// :param username the username of the user
///:return `User`
export async function get(username) {
    const request = new Request(`${BASE_USERS_URL}/${username}`, {
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

/// get the public profile of a user
/// :param id id of the user
///:return `User`
export async function get_id(id) {
    const request = new Request(`${BASE_USERS_URL}/${id}`, {
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
