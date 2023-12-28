import {BASE_ACTIVITIES_URL, STATUS, Result} from "./main.js";

export const ActivitType = {
    PushUps: "PushUps"
}

export class Activity {
    constructor(id, author_id, amount, activity_type, start_time, end_time) {
        this.id = id;
        this.author_id = author_id;
        this.amount = amount;
        this.activity_type = activity_type;
        this.start_time = start_time;
        this.end_time = end_time;
    }
}

export class NewActivity {
    constructor(amount, activity_type, start_time, end_time) {
        this.amount = amount;
        this.activity_type = activity_type;
        this.start_time = start_time;
        this.end_time = end_time;
    }
}

export class EditActivity {
    constructor(amount, activity_type, start_time, end_time) {
        this.amount = amount;
        this.activity_type = activity_type;
        this.start_time = start_time;
        this.end_time = end_time;
    }
}

/// get a list of activities in a given time interval
/// :param id the id of the activity
/// :return `Activity` returns a single activity
export async function get(id) {
    const request = new Request(`${BASE_ACTIVITIES_URL}/${id}`, {
        method: "GET",
        credentials: 'include',
    });

    let response = await fetch(request);
    if (response.status === STATUS.OK) {
        let value = await response.json();
        return new Result(true, new Activity(value.id, value.author_id, value.amount, value.activity_type, value.start_time, value.end_time));
    } else {
        let error = await response.text();
        return new Result(false, error);
    }
}

/// get a list of activities in a given time interval
/// :param from RFC-3339 compliant string. the timepoint at which the first activity should have started
/// :param to RFC-3339 compliant string. the last timepoint until which the last activity should have ended
/// :return `list[Activity]` returns a list of activites
export async function get_from_to(from, to) {
    const request = new Request(`${BASE_ACTIVITIES_URL}/${from}/${to}`, {
        method: "GET",
        credentials: 'include',
    });

    let response = await fetch(request);
    if (response.status === STATUS.OK) {
        let raw = await response.json();
        let activities = [];
        for (const value of raw) {
            activities.push(new Activity(value.id, value.author_id, value.amount, value.activity_type, value.start_time, value.end_time));
        }
        return new Result(true, activities);
    } else {
        let error = await response.text();
        return new Result(false, error);
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

    let response = await fetch(request);
    if (response.status === STATUS.CREATED) {
        let value = await response.json();
        return new Result(true, new Activity(value.id, value.author_id, value.amount, value.activity_type, value.start_time, value.end_time));
    } else {
        let error = await response.text();
        return new Result(false, error);
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

    let response = await fetch(request);
    if (response.status === STATUS.OK) {
        let value = await response.json();
        return new Result(true, new Activity(value.id, value.author_id, value.amount, value.activity_type, value.start_time, value.end_time));
    } else {
        let error = await response.text();
        return new Result(false, error);
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

    let response = await fetch(request);
    if (response.status === STATUS.OK) {
        let value = await response.json();
        return new Result(true, new Activity(value.id, value.author_id, value.amount, value.activity_type, value.start_time, value.end_time));
    } else {
        let error = await response.text();
        return new Result(false, error);
    }
}
