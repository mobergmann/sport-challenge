import {new_activity} from "/scripts/activity.js";
import {TIMEZONE_INTS} from "/scripts/variables.js";

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

    let label = document.createElement("label");
    label.htmlFor = id;
    label.innerHTML = "Timezone";
    parent.appendChild(label);

    let select = timezone_dom_select();
    select.id = id;
    select.classList.add("w100", "input");
    let offset = Math.trunc(Number((new Date().getTimezoneOffset() * (-1)) / 60));
    const offset_string = offset.toString(); //(offset > 0) ? `+${offset.toString()}` : offset.toString();
    let index = -1;
    for (const i of TIMEZONE_INTS) {
        ++index;
        if (i.value === offset_string) {
            break;
        }
    }
    select.selectedIndex = index;
    select.style.margin = ".5rem";

    // todo: automatically select current timezone
    parent.appendChild(select);
}

function set_time_now(html_id) {
    // set default date to today
    let now = new Date();
    document.getElementById(html_id).value = now.toISOString().slice(0,16);
}

window.set_time_now = set_time_now;

spawn_timzone("start_time-timezone", document.getElementById("start_time-timezone-container"));
spawn_timzone("end_time-timezone", document.getElementById("end_time-timezone-container"));

document.querySelector("#form").addEventListener("submit", async (e) => {
    e.preventDefault();

    const amount = Number(document.querySelector("#amount").value);
    const activity_type = document.querySelector("#activity_type").value;

    let start_time = new Date(document.getElementById("start_time").value).toISOString();
    // todo add timezone to end of string
    console.log(start_time);

    let end_time = new Date(document.getElementById("end_time").value).toISOString();
    // todo add timezone to end of string
    console.log(end_time);

    try {
        const activity = Activity
        let result = await create_activity(amount, activity_type, start_time, end_time);
        window.location = "/home.html";
    } catch (error) {
        console.error(error);
        alert("Error while submitting new activity. Please try again.");
    }
});
