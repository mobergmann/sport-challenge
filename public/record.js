import {create, NewActivity} from "/scripts/api/activities.js";
import {TIMEZONE_INTS} from "/scripts/helpers.js";

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
set_time_now("start_time");


document.querySelector("#form").addEventListener("submit", async (e) => {
    e.preventDefault();

    const amount = Number(document.querySelector("#amount").value);
    const activity_type = document.querySelector("#activity_type").value;

    let start_time = new Date(document.getElementById("start_time").value);
    let start_time_str = start_time.toISOString();
    // todo add timezone to end of string
    console.log(start_time);

    let duration = Number(document.getElementById("duration").value);
    let end_time = new Date(start_time.getTime() + duration*60000);
    let end_time_str = end_time.toISOString();

    // todo add timezone to end of string
    console.log(end_time);

    let activity = new NewActivity(amount, activity_type, start_time_str, end_time_str);
    let res = await create(activity);
    if (res.ok) {
        const urlParams = new URLSearchParams(window.location.search);
        let target_location = urlParams.get('redirect');
        if (target_location === null) {
            target_location = "/index.html";
        }
        window.location = target_location;
    } else {
        console.error(res.value);
        alert(`Error while submitting new activity: ${res.value}`);
    }
});
