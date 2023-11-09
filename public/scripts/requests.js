import {BASE_URL} from "./variables.js";

export async function do_request(request, body_expected = true) {
    return await fetch(request)
        .then((response) => {
            if (response.status === 200) {
                if (!body_expected) {
                    return response;
                } else {
                    return response.json();
                }
            } else {
                throw new Error("Something went wrong on API server!");
            }
        })
        .then((response) => {
            return response;
        })
        .catch((error) => {
            console.error(error);
            throw error;
        });
}


export async function ping() {
    const request = new Request(`${BASE_URL}/ping`, {
        method: "GET",
    });

    return await do_request(request, false);
}
