import {Activity, create as create_activity} from "./api/activities.js";
import {TIMEZONE_INTS} from "./scripts/helpers.js";

function spawn_timzone(id, parent) {
// @source: https://stackoverflow.com/a/52265733/11186407
    function timezone_dom_select() {
        let select = document.createElement("select");

        for (let i = 0; i < TIMEZONE_INTS.length; ++i) {
            let tz = TIMEZONE_INTS[i];
            let option = document.createElement("option");

            option.value = tz.value;
            option.appendChild(document.createTextNode(tz.label));
            select.appendChild(option);
        }

        return select;
    }

    let select = timezone_dom_select();
    select.id = id;
    select.selectedIndex = 28;
    // todo: automatically select current timezone
    parent.appendChild(select);

    let label = document.createElement("label");
    label.htmlFor = id;
    label.innerHTML = "Timezone";
    parent.appendChild(label);
}

let now = new Date();

spawn_timzone("start_time-timezone", document.getElementById("start_time-timezone-container"));
// set default date to today
document.getElementById("start_time").value = now.toISOString().slice(0,16);

//spawn_timzone("end_time-timezone", document.getElementById("end_time-timezone-container"));
// set default date to today
//document.getElementById("end_time").value = now.toISOString().slice(0,16);

document.getElementById("submit").addEventListener("click", async () => {
    const amount = Number(document.getElementById("amount").value);
    const activity_type = document.getElementById("activity_type").value;

    let start_time = new Date(document.getElementById("start_time").value).toISOString();
    // todo add timezone to end of string
    console.log(start_time);

    /*
    let end_time = new Date(document.getElementById("end_time").value).toISOString();
    // todo add timezone to end of string
    console.log(end_time);
    */
    let end_time = start_time;

    try {
        const activity = Activity
        let result = await create_activity(amount, activity_type, start_time, end_time);
        window.location = "/home.html";
    } catch (error) {
        console.error(error);
        alert("Error while submitting new activity. Please try again.");
    }
});

// todo: maybe redirect to home
