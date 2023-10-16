import {BASE_ACTIVITIES_URL} from "./variables.js";

async function do_request(request) {
    return await fetch(request)
        .then((response) => {
            if (response.status === 200) {
                return response.json();
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

export async function get_activity(id) {
    const request = new Request(`${BASE_ACTIVITIES_URL}/${id}`, {
        method: "GET",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
    });

    return await do_request(request);
}

export async function get_all_activities() {
    const request = new Request(`${BASE_ACTIVITIES_URL}`, {
        method: "GET",
    });

    return await do_request(request);
}

export async function new_activity(amount, activity_type, start_time, end_time) {
    const activity = {
        amount: amount,
        activity_type: activity_type,
        start_time: start_time,
        end_time: end_time,
    };

    const request = new Request(`${BASE_ACTIVITIES_URL}`, {
        method: "POST",
        body: JSON.stringify(activity),
        headers: new Headers({
            "Content-Type": "application/json",
        }),
    });
    return await do_request(request);
}

export function edit_activity(amount, activity_type, start_time, end_time) {
    //let amount = document.getElementById("edit_activity-amount").value;
    //let activity_type = document.getElementById("edit_activity-activity_type").value;
    //let start_time = document.getElementById("edit_activity-start_time").value;
    //let end_time = document.getElementById("edit_activity-end_time").value;
}

export function delete_activity(id) {
    //let id = document.getElementById("delete_activity-id").value;
}
