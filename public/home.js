import "/scripts/helper.js";
import "/scripts/helpers.js"
import {get_from_to as get_activities} from "/scripts/api/activities.js";
import {get_id as get_user_by_id} from "/scripts/api/users.js";
import {get as get_account} from "/scripts/api/account.js";

// Global Variables
let current_week = new Date();
let global_chart;


/// download all activities and store them in a map assigning each user a list of the associated activities
/// the map is ordered by the *sum of the amount of the activities* of each user
async function prepare_activities_data(from, to) {
    let activities_per_user = new Map();

    let res = await get_activities(from, to);
    if (!res.ok) {
        alert("Error while fetching the activities. Are you logged in?");
    }
    for (const activity of res.value) {
        // if author not already in map, insert the author as a key and an empty list
        if (!activities_per_user.has(activity.author_id)) {
            activities_per_user.set(activity.author_id, []);
        }

        // add the activity to the author
        activities_per_user.get(activity.author_id).push({
            id: activity.id,
            author_id: activity.author_id,
            amount: activity.amount,
            activity_type: activity.activity_type,
            end_time: new Date(activity.end_time),
            start_time: new Date(activity.start_time),
        });
    }

    // for each user sort it's activities by date
    for (const [_, activities] of activities_per_user) {
        activities.sort((lhs, rhs) => {
            // negative if a is less than b,
            // positive if a is greater than b,
            // zero if they are equal.
            if (lhs.start_time < rhs.start_time) {
                return -1;
            } else if (lhs.start_time > rhs.start_time) {
                return 1;
            } else {
                return 0;
            }
        });
    }

    // sort the activities_per_user map so that the person with the most amounts is inserted first
    // convert map to list of tuples
    let tmp = [];
    for (const [key, value] of activities_per_user) {
        tmp.push([key, value]);
    }
    // sort tmp by value sum
    tmp.sort((lhs, rhs) => {
        let lhs_sum = lhs[1].sum("amount");
        let rhs_sum = rhs[1].sum("amount");

        // negative if a is less than b,
        // positive if a is greater than b,
        // zero if they are equal.
        if (lhs_sum < rhs_sum) {
            return -1;
        } else if (lhs_sum > rhs_sum) {
            return 1;
        } else {
            return 0;
        }
    });
    // reverse tmp, so elements are sorted descending
    tmp.reverse();
    // conver tmp back to a map
    let sorted_activities_per_user = new Map();
    for (const i of tmp) {
        sorted_activities_per_user.set(i[0], i[1]);
    }

    return sorted_activities_per_user;
}

/// Download for each user in the activities_per_user map the user object
async function prepare_user_by_id(activities_per_user) {
    let user_by_id = new Map();
    for (const [author_id, _] of activities_per_user) {
        let user = await get_user_by_id(author_id);
        user_by_id.set(author_id, user);
    }
    return user_by_id;
}

/// display the activities in a chart
function init_chart(activities_per_user, user_by_id) {
    // display the chart
    const x_axis_labels = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

    // prepare for each user the y-Axis
    let activities_per_day = [];
    for (const [author_id, activities] of activities_per_user) {
        // push-ups done at day's index
        let amounts = [0, 0, 0, 0, 0, 0, 0];
        for (let i = 0; i < activities.length; ++i) {
            // sum the activity's amount to its corresponding weekday
            const activity_weekday = activities[i].start_time.getDay();
            amounts[activity_weekday] += activities[i].amount;
        }

        activities_per_day.push({
            // todo: replace with author name
            label: user_by_id.get(author_id).name,
            data: amounts,
            fill: false,
            // todo: random color
            borderColor: 'rgb(75, 192, 192)',
            tension: 0.1
        });
    }

    // currently y_axis stores the amount done at each day.
    // we instead  want to store the sum of the activity.amount from 0 to i
    let y_axis = [];
    for (let dataset of activities_per_day) {
        for (let i = 1; i < dataset.data.length; ++i) {
            dataset.data[i] = dataset.data[i] + dataset.data[i-1];
        }
        y_axis.push(dataset);
    }
    // also add the weeks goal to the y-axis
    y_axis.push({
        label: "Goal",
        data: [140, 140, 140, 140, 140, 140, 140],
        fill: false,
        borderColor: 'Red',
        tension: 0.1
    });

    // first destroy char, so we can recreate it if it was already initialized
    if (global_chart) {
        global_chart.destroy();
    }
    // create the chart
    global_chart = new Chart("graph-comparison-canvas", {
        type: "line",
        data: {
            labels: x_axis_labels,
            datasets: y_axis,
        },
        options: {
            legend: {display: true},
            scales: {
                yAxes: [{ticks: {min: 6, max: 16}}],
            }
        }
    });
}

function init_log(activities_per_user, user_by_id) {
    // test to see if the browser supports the HTML template element by checking
    // for the presence of the template element's content attribute.
    if (!("content" in document.createElement("template"))) {
        alert("Your browser doesn't support templates. We cannot display the site properly. Try updating it or using a more up to date browser.");
    }

    const parent = document.querySelector("#ref-log-list");
    parent.innerHTML = null; // clear the log
    const template = document.querySelector("#template-log");

    let place = 1;
    for (const [author_id, activities] of activities_per_user) {
        const clone = template.content.cloneNode(true);

        // add collapsable functionality to buttons
        let button_down = clone.querySelector("#logentry-i1");
        button_down.id = `logentry-down-${author_id}`;
        let button_up = clone.querySelector("#logentry-i3");
        button_up.id = `logentry-up-${author_id}`;
        let activities_container = clone.querySelector("#logentry-i2");
        activities_container.id = `logentry-container-${author_id}`;
        button_down.onclick = () => {
            button_down.style.display = "none";
            activities_container.style.display = "block";
        };
        button_up.onclick = () => {
            button_down.style.display = "block";
            activities_container.style.display = "none";
        };

        // add the data to the log entry
        clone.querySelector("#template-log-place").innerHTML = place;
        clone.querySelector("#template-log-username").innerHTML = user_by_id.get(author_id).name;
        clone.querySelector("#template-log-amount").innerHTML = activities.sum("amount");
        // add the activities to the log entry
        let activities_list = clone.querySelector("#template-log-activities");
        let activity_template = document.querySelector("#template-log-activity");
        for (const activity of activities) {
            const clone = activity_template.content.cloneNode(true);
            clone.querySelector("#template-log-activity-starttime").value = activity.start_time.toISOString().slice(0,16);
            clone.querySelector("#template-log-activity-duration").innerHTML = activity.end_time - activity.start_time;
            clone.querySelector("#template-log-activity-amount").innerHTML = activity.amount;

            activities_list.appendChild(clone);
        }

        parent.appendChild(clone);
        ++place;
    }
}

async function update_frontend() {
    document.querySelector("#current_year").innerHTML = current_week.getFullYear().toString();
    document.querySelector("#current_week").innerHTML = current_week.getWeek().toString();

    const from = current_week.getFirstWeekDay().toRFC3339();
    const to = current_week.getLastWeekDay().toRFC3339();
    const activities_per_user = await prepare_activities_data(from, to);

    const user_by_id = await prepare_user_by_id(activities_per_user);

    init_chart(activities_per_user, user_by_id);
    init_log(activities_per_user, user_by_id);
}

async function main() {
    let res = await get_account();
    if (!res.ok) {
        alert("You are not signed in. Sign in first.");
        window.location = "/auth/login.html";
    }

    await update_frontend();

    document.querySelector("#button-load_previous_week").onclick = async () => {
        current_week.previousWeek();
        await update_frontend();
    };
    document.querySelector("#button-load_next_week").onclick = async () => {
        current_week.nextWeek();
        await update_frontend();
    };
}

await main();
