const BASE_URL = "http://127.0.0.1:3000/v1/";
const BASE_AUTH_URL = BASE_URL + "auth/";
const BASE_ACTIVITIES_URL = BASE_URL + "activities/";

//#region auth

function sign_in() {
    const user = {
        name: document.getElementById("sign_in-username").value,
        password: document.getElementById("sign_in-password").value,
    };

    const request = new Request(BASE_AUTH_URL + "sign_in", {
        method: "POST",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
        body: JSON.stringify(user),
        credentials: 'include',
    });

    fetch(request)
        .then((response) => {
            if (response.status === 200) {
                return;
            } else {
                throw new Error("Something went wrong on API server!");
            }
        })
        .then(() => {
            console.debug("Sucessfull SignIn");
        })
        .catch((error) => {
            console.error(error);
        });
}

function sign_out() {
    const request = new Request(BASE_AUTH_URL + "sign_out", {
        method: "GET",
        credentials: 'include',
    });

    fetch(request)
        .then((response) => {
            if (response.status === 200) {
                return;
            } else {
                throw new Error("Something went wrong on API server!");
            }
        })
        .then(() => {
            console.debug("Sucessfull SignOut");
        })
        .catch((error) => {
            console.error(error);
        });
}

function sign_up() {
    // check for password equality
    let pw_1 = document.getElementById("sign_up-password").value;
    let pw_2 = document.getElementById("sign_up-password2").value;
    if (pw_1 !== pw_2) {
        alert("Passwords do not match!");
        document.getElementById("sign_up-form").reset();
    }

    const user = {
        name: document.getElementById("sign_up-username").value,
        password: pw_1,
    };

    const request = new Request(BASE_AUTH_URL + "sign_up", {
        method: "POST",
        body: JSON.stringify(user),
        headers: new Headers({
            "Content-Type": "application/json",
       }),
    });

    fetch(request)
        .then((response) => {
            if (response.status === 201) {
                return response.json();
            } else {
                throw new Error("Something went wrong on API server!");
            }
        })
        .then((response) => {
            console.debug("Sucessfull SignUp");
            console.debug(response);
        })
        .catch((error) => {
            console.error(error);
        });
}

////#endregion

//#region activities

function get_activity() {
    let id = document.getElementById("activity-id").value;

    const request = new Request(BASE_ACTIVITIES_URL + String(id), {
        method: "GET",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
    });

    fetch(request)
        .then((response) => {
            if (response.status === 200) {
                return response.json();
            } else {
                throw new Error("Something went wrong on API server!");
            }
        })
        .then((response) => {
            console.debug(response);
        })
        .catch((error) => {
            console.error(error);
        });
}

function get_all_activities() {
    const request = new Request(BASE_ACTIVITIES_URL, {
        method: "GET",
    });

    fetch(request)
        .then((response) => {
            if (response.status === 200) {
                return response.json();
            } else {
                throw new Error("Something went wrong on API server!");
            }
        })
        .then((response) => {
            console.debug(response);
        })
        .catch((error) => {
            console.error(error);
        });

}

function new_activity() {
    let amount = document.getElementById("new_activity-amount").value;
    let activity_type = document.getElementById("new_activity-activity_type").value;
    let start_time = document.getElementById("new_activity-start_time").value;
    let end_time = document.getElementById("new_activity-end_time").value;
}

function edit_activity() {
    let amount = document.getElementById("edit_activity-amount").value;
    let activity_type = document.getElementById("edit_activity-activity_type").value;
    let start_time = document.getElementById("edit_activity-start_time").value;
    let end_time = document.getElementById("edit_activity-end_time").value;
}

function delete_activity() {
    let id = document.getElementById("delete_activity-id").value;
}

////#endregion