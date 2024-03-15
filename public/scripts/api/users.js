import {BASE_USERS_URL, STATUS, Result} from "./main.js";

export class User {
    constructor(id, username, display_name) {
        this.id = id;
        this.username = username;
        this.display_name = display_name;
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
        let value = await response.json();
        return new Result(true, new User(value.id, value.username, value.display_name));
    } else {
        let error = await response.text();
        return new Result(false, error);
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

    let response = await fetch(request);
    if (response.status === STATUS.OK) {
        let value = await response.json();
        return new Result(true, new User(value.id, value.username, value.display_name));
    } else {
        let error = await response.text();
        return new Result(false, error);
    }
}
