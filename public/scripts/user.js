import {BASE_USER_URL} from "./variables.js";
import {do_request} from "./requests.js"

export async function get_user(username) {
    const request = new Request(`${BASE_USER_URL}/${username}`, {
        method: "GET",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
    });

    return await do_request(request);
}

export async function get_user_by_id(id) {
    const request = new Request(`${BASE_USER_URL}/id/${id}`, {
        method: "GET",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
    });

    return await do_request(request);
}
