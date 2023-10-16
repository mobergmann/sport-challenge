import {get_all_activities} from "./scripts/activity.js";

/// @source: https://stackoverflow.com/a/31810991/11186407
Date.prototype.getWeek = function() {
    let onejan = new Date(this.getFullYear(), 0, 1);
    let today = new Date(this.getFullYear(), this.getMonth(), this.getDate());
    let dayOfYear = ((today - onejan + 86400000) / 86400000);
    return Math.ceil(dayOfYear / 7)
};

let activities_per_user = new Map();
{
    for (const activity of await get_all_activities()) {
        if (! activities_per_user.has(activity.author_id)) {
            activities_per_user.set(activity.author_id, []);
        }
        activities_per_user.get(activity.author_id).push({
            id: activity.id,
            author_id: activity.author_id,
            amount: activity.amount,
            activity_type: activity.activity_type,
            end_time: new Date(activity.end_time),
            start_time: new Date(activity.start_time),
        });
    }

    // sort activities of each user by date
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
}

console.debug(activities_per_user);

// display the chart
{
    const x_axis_labels = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    
    // prepare for each user the y-Axis
    let push_up_per_day = [];
    for (const [author_id, activities] of activities_per_user) {
        // push ups done at day's index
        let amounts = [0, 0, 0, 0, 0, 0, 0];
        for (let i = 0; i < activities.length; ++i) {
            // only factor in activities of this week
            if (activities[i].start_time.getWeek() !== new Date().getWeek()) {
                continue;
            }
            
            // sum the activity's amount to its corresponding weekday
            if (activities[i].start_time.getDay() === 1) {
                // Monday
                amounts[0] += activities[i].amount;
            } else if (activities[i].start_time.getDay() === 2) {
                // Tuesday
                amounts[1] += activities[i].amount;
            } else if (activities[i].start_time.getDay() === 3) {
                // Wednesday
                amounts[2] += activities[i].amount;
            } else if (activities[i].start_time.getDay() === 4) {
                // Thursday
                amounts[3] += activities[i].amount;
            } else if (activities[i].start_time.getDay() === 5) {
                // Friday
                amounts[4] += activities[i].amount;
            } else if (activities[i].start_time.getDay() === 6) {
                // Saturday
                amounts[5] += activities[i].amount;
            } else if (activities[i].start_time.getDay() === 7) {
                // Sunday
                amounts[6] += activities[i].amount;
            }
        }
        
        push_up_per_day.push({
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
    for (let dataset of push_up_per_day) {
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
