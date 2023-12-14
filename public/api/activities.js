import {BASE_ACTIVITIES_URL} from "./main.js";

export const ActivitType = {
    PushUps: "PushUps"
}

export class Activity {
    constructor(id, author_id, amount, activity_type, start_time, end_time) {
        self.id = id;
        self.author_id = author_id;
        self.amount = amount;
        self.activity_type = activity_type;
        self.start_time = start_time;
        self.end_time = end_time;
    }
}

export class NewActivity {
    constructor(amount, activity_type, start_time, end_time) {
        self.amount = amount;
        self.activity_type = activity_type;
        self.start_time = start_time;
        self.end_time = end_time;
    }
}

export class EditActivity {
    constructor(amount, activity_type, start_time, end_time) {
        self.amount = amount;
        self.activity_type = activity_type;
        self.start_time = start_time;
        self.end_time = end_time;
    }
}

/// get a list of activities in a given time interval
/// :param id the id of the activity
/// :return `Activity` returns a single activity
export async function get(id) {
    const request = new Request(`${BASE_ACTIVITIES_URL}/${id}`, {
        method: "POST",
        credentials: 'include',
    });

    try {
        let response = await fetch(request);
        return response.json();
    } catch (error) {
        return error;
    }
}

/// get a list of activities in a given time interval
/// :param from RFC-3339 compliant string. the timepoint at which the first activity should have started
/// :param to RFC-3339 compliant string. the last timepoint until which the last activity should have ended
/// :return `list[Activity]` returns a list of activites
export async function get_from_to(from, to) {
    const request = new Request(`${BASE_ACTIVITIES_URL}/${from}/${to}`, {
        method: "POST",
        credentials: 'include',
    });

    try {
        let response = await fetch(request);
        return response.json();
    } catch (error) {
        return error;
    }
}

/// creates a new activity
/// :param activity the activity object of type `NewActivity` with the updated informations of the activity
/// :return `Activity` returns the newly created activity
export async function create(activity) {
    const request = new Request(`${BASE_ACTIVITIES_URL}`, {
        method: "POST",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
        body: JSON.stringify(activity),
        credentials: 'include',
    });

    try {
        let response = await fetch(request);
        return response.json();
    } catch (error) {
        return error;
    }
}

/// edits the information of an activity
/// :param id id if the activity to edit
/// :param activity the activity object of type `EditActivity` with the updated informations of the activvity
/// :return `Activity` returns the updated activity
export async function edit(id, activity) {
    const request = new Request(`${BASE_ACTIVITIES_URL}/${id}`, {
        method: "PUT",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
        body: JSON.stringify(activity),
        credentials: 'include',
    });

    try {
        let response = await fetch(request);
        return response.json();
    } catch (error) {
        return error;
    }
}

/// delete an activity
/// :param id id of the activity to remove
/// :return `Activity` returns the deleted activity
export async function remove(id) {
    const request = new Request(`${BASE_ACTIVITIES_URL}/${id}`, {
        method: "DELETE",
        credentials: 'include',
    });

    try {
        let response = await fetch(request);
        return response.json();
    } catch (error) {
        return error;
    }
}
