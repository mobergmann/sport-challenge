import {BASE_ACTIVITIES_URL, STATUS, Result} from "./main.js";

export class Like {
    constructor(athlete_id, activity_id) {
        this.athlete_id = athlete_id;
        this.activity_id = activity_id;
    }
}

/// get a list of likes of a given activity
/// :param activity_id the id of the activity
/// :return `Vec<Like>` returns list of likes
export async function get(activity_id) {
    const request = new Request(`${BASE_ACTIVITIES_URL}/${activity_id}/likes`, {
        method: "GET",
        credentials: 'include',
    });

    let response = await fetch(request);
    if (response.status === STATUS.OK) {
        let raw = await response.json();
        let likes = [];
        for (const value of raw) {
            likes.push(new Like(value.athlete_id, value.activity_id));
        }
        return new Result(true, likes);
    } else {
        let error = await response.text();
        return new Result(false, error);
    }
}

/// creates a new activity
/// :param activity the activity object of type `NewActivity` with the updated informations of the activity
/// :return `Activity` returns the newly created activity
export async function create(activity_id) {
    const request = new Request(`${BASE_ACTIVITIES_URL}/${activity_id}/likes`, {
        method: "POST",
        credentials: 'include',
    });

    let response = await fetch(request);
    if (response.status === STATUS.OK) {
        let raw = await response.json();
        let likes = [];
        for (const value of raw) {
            likes.push(new Like(value.athlete_id, value.activity_id));
        }
        return new Result(true, likes);
    } else {
        let error = await response.text();
        return new Result(false, error);
    }
}

/// delete an activity
/// :param id id of the activity to remove
/// :return `Activity` returns the deleted activity
export async function remove(activity_id) {
    const request = new Request(`${BASE_ACTIVITIES_URL}/${activity_id}/likes`, {
        method: "DELETE",
        credentials: 'include',
    });

    let response = await fetch(request);
    if (response.status === STATUS.OK) {
        let raw = await response.json();
        let likes = [];
        for (const value of raw) {
            likes.push(new Like(value.athlete_id, value.activity_id));
        }
        return new Result(true, likes);
    } else {
        let error = await response.text();
        return new Result(false, error);
    }
}
