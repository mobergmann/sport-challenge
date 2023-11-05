import {get_all_activities} from "./scripts/activity.js";

/// @source: https://stackoverflow.com/a/31810991/11186407
Date.prototype.getWeek = function() {
    let onejan = new Date(this.getFullYear(), 0, 1);
    let today = new Date(this.getFullYear(), this.getMonth(), this.getDate());
    let dayOfYear = ((today - onejan + 86400000) / 86400000);
    return Math.ceil(dayOfYear / 7)
};

Array.prototype.sum = function() {
    let sum = 0;
    for (const element of this) {
        sum += element;
    }
    return sum;
}

Array.prototype.sum = function(key) {
    let sum = 0;
    for (const element of this) {
        sum += element[key];
    }
    return sum;
}

/// download all activities and store them in a map assigning each user a list of the associated activities
/// the map is ordered by the *sum of the amount of the activities* of each user
async function prepare_activities_data() {
    let activities_per_user = new Map();

    for (const activity of await get_all_activities()) {
        // if author not alreay in map, insert the author as a key and an emptpy list
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
    let tmp = [];
    for (const [author_id, activities] of activities_per_user) {
        tmp.push({author_id: author_id, activities: activities});
    }
    tmp = tmp.sort((lhs, rhs) => {
        let lhs_sum = lhs.activities.reduce((a, b) => a.amount + b.amount, 0);
        let rhs_sum = rhs.activities.reduce((a, b) => a.amount + b.amount, 0);

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
    // convert placeholder to map
    let sorted_activities_per_user = new Map();
    for (const i of tmp) {
        sorted_activities_per_user.set(i.author_id, i.activities);
    }

    return sorted_activities_per_user;
}

/// display the activities in a chart
function init_chart(activities_per_user) {
    // display the chart
    const x_axis_labels = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

    // prepare for each user the y-Axis
    let activities_per_day = [];
    for (const [author_id, activities] of activities_per_user) {
        // push ups done at day's index
        let amounts = [0, 0, 0, 0, 0, 0, 0];
        for (let i = 0; i < activities.length; ++i) {
            // only factor in activities of this week
            if (activities[i].start_time.getWeek() !== new Date().getWeek()) {
                continue;
            }

            // sum the activity's amount to its corresponding weekday
            const activity_weekday = activities[i].start_time.getDay();
            amounts[activity_weekday] += activities[i].amount;
        }

        activities_per_day.push({
            // todo: replace with author name
            label: author_id,
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

    // create the chart
    new Chart("graph-comparison-canvas", {
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

function init_log(activities_per_user) {
    // test to see if the browser supports the HTML template element by checking
    // for the presence of the template element's content attribute.
    if (!("content" in document.createElement("template"))) {
        alert("Your browser doesn't support templates. We cannot display the site properly. Try updating it or using a more up to date browser.");
    }

    const parent = document.querySelector("#ref-log-list");
    const template = document.querySelector("#template-log");

    let place = 1;
    for (const [author_id, activities] of activities_per_user) {
        const clone = template.content.cloneNode(true);
        // add the data to the log entry
        clone.querySelector("#template-log-place").innerHTML = place;
        clone.querySelector("#template-log-username").innerHTML = author_id;
        clone.querySelector("#template-log-amount").innerHTML = activities.sum("amount"); // todo: issue
                                                                 // (a, b) => a.amount + b.amount,
        // add the activities to the log entry
        let activities_list = clone.querySelector("#template-log-activities");
        let activity_template = document.querySelector("#template-log-activity");
        for (const activity of activities) {
            const clone = activity_template.content.cloneNode(true);
            clone.querySelector("#template-log-activity-start-time").value = activity.start_time.toISOString().slice(0,16);
            clone.querySelector("#template-log-activity-duration").value = activity.end_time - activity.start_time;
            clone.querySelector("#template-log-activity-amount").value = activity.amount;

            activities_list.appendChild(clone);
        }

        parent.appendChild(clone);
        ++place;
    }
}



async function main() {
    let data = await prepare_activities_data();

    init_chart(data);
    init_log(data);
}

await main();
