export const BASE_URL = "/v1";
export const BASE_ACCOUNT_URL = `${BASE_URL}/account`;
export const BASE_USERS_URL = `${BASE_URL}/users`;
export const BASE_AUTH_URL = `${BASE_URL}/auth`;
export const BASE_ACTIVITIES_URL = `${BASE_URL}/activities`;


//#region auth

/**
 * Login and create a session
 * @param username username to log in
 * @param password password of the user
 * @returns {Promise<Response>} the user-object of the logged-in user
 */
export async function login(username, password)
{
    const user = {
        name: username,
        password: password,
    };

    const request = new Request(`${BASE_AUTH_URL}/login`, {
        method: "POST",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
        body: JSON.stringify(user),
        credentials: 'include',
    });

    try {
        let response = await fetch(request);
        return response.json();
    } catch (error) {
        return error;
    }
}

/**
 * Logout the current session
 * @returns {Promise<void>}
 */
export async function logout() {
    const request = new Request(`${BASE_AUTH_URL}/logout`, {
        method: "PUT",
        credentials: 'include',
    });

    await fetch(request);
}

//#endregion

//#region account

/**
 * Gets the current logged in account object (including properties like the password_hash).
 * If no user is logged in, then an appropriate http error code is returned
 * @returns {Promise<Response>} the current logged-in user object
 */
export async function get_account() {
    const request = new Request(`${BASE_ACCOUNT_URL}`, {
        method: "GET",
    });

    return (await fetch(request)).json();
}

/**
 * Creates/registers a new user account
 * @param username username
 * @param password password
 * @returns {Promise<Response>} the newly created account
 */
export async function create_account(username, password) {
    const user = {
        name: username,
        password: password,
    };

    const request = new Request(`${BASE_ACCOUNT_URL}`, {
        method: "POST",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
        body: JSON.stringify(user),
    });

    return (await fetch(request)).json();
}

/**
 * Updates the user object, excluding the password. The password has to be set via an extra route.
 * @param username new username
 * @returns {Promise<Response>} updated user object
 * @returns {Promise<Response>}
 */
export async function update_account(username) {
    const user = {
        name: username,
    };

    const request = new Request(`${BASE_ACCOUNT_URL}`, {
        method: "PUT",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
        body: JSON.stringify(user),
    });

    return (await fetch(request)).json();
}

/**
 * !!Permanently!! deletes the current user account
 * @param new_password the password of the current account
 * @returns {Promise<Response>}
 */
export async function delete_account(new_password) {
    const pw = {
        password: new_password,
    };

    const request = new Request(`${BASE_ACCOUNT_URL}`, {
        method: "DELETE",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
        body: JSON.stringify(pw),
    });

    return (await fetch(request)).json();
}

/**
 * Updates the current users account password
 * @returns {Promise<Response>} the updated user object
 */
export async function update_account_password(current_password, new_password) {
    const pw = {
        current_password: current_password,
        password: new_password,
    };

    const request = new Request(`${BASE_ACCOUNT_URL}`, {
        method: "PUT",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
        body: JSON.stringify(pw),
    });

    return (await fetch(request)).json();
}

//#endregion

//#region users

/**
 * Returns a public user object by it username
 * @param username username of the user
 * @returns {Promise<Response>} user object with the username
 */
export async function get_user(username) {
    const request = new Request(`${BASE_USERS_URL}/${username}`, {
        method: "GET",
    });

    return (await fetch(request)).json();
}

/**
 * Returns a public user object by it username
 * @param id id of the user
 * @returns {Promise<Response>} user object with the id
 */
export async function get_user_by_id(id) {
    const request = new Request(`${BASE_USERS_URL}/id/${id}`, {
        method: "GET",
    });

    return (await fetch(request)).json();
}

//#endregion

//#region activities

/**
 * Creates a new activity for the current user
 * @param amount amount done in the activity
 * @param activity_type type of the activity
 * @param start_time start time of the activity
 * @param end_time end time of the activity
 * @returns {Promise<Response>} the newly created activity object
 */
export async function create_activity(amount, activity_type, start_time, end_time) {
    const activity = {
        amount: amount,
        activity_type: activity_type,
        start_time: start_time,
        end_time: end_time,
    };

    const request = new Request(`${BASE_ACTIVITIES_URL}`, {
        method: "POST",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
        body: JSON.stringify(activity),
    });

    return (await fetch(request)).json();
}

/**
 * Get an activity by id
 * @param id id of the activity
 * @returns {Promise<Response>} Activity object with the specified id
 */
export async function get_activity(id) {
    const request = new Request(`${BASE_ACTIVITIES_URL}/${id}`, {
        method: "GET",
    });

    return (await fetch(request)).json();
}

/**
 * Returns a list of activities which took place in the specified time intervall
 * @param from when the activities should have started
 * @param to when the activities should have ended
 * @returns {Promise<Response>} list of activities
 */
export async function get_activities(from, to) {
    const request = new Request(`${BASE_ACTIVITIES_URL}/${from}/${to}`, {
        method: "GET",
    });

    return (await fetch(request)).json();
}

/**
 * Edits an activity.
 * @param id id of the activity
 * @param amount amount done in the activity
 * @param activity_type type of the activity
 * @param start_time start time of the activity
 * @param end_time end time of the activity
 * @returns {Promise<Response>} updated activity
 */
export async function update_activity(id, amount, activity_type, start_time, end_time) {
    const activity = {
        amount: amount,
        activity_type: activity_type,
        start_time: start_time,
        end_time: end_time,
    };

    const request = new Request(`${BASE_ACTIVITIES_URL}/${id}`, {
        method: "PUT",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
        body: JSON.stringify(activity),
    });

    return (await fetch(request)).json();
}

/**
 * Deletes an activity.
 * when the current user is not the author of the activity returns an error code
 * @param id id of the activity
 * @returns {Promise<Response>} deleted activity
 */
export async function delete_activity(id) {
    const request = new Request(`${BASE_ACTIVITIES_URL}/${id}`, {
        method: "DELETE",
    });

    return (await fetch(request)).json();
}

//#endregion
