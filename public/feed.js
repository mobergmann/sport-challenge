import "/scripts/helper.js";
import "/scripts/helpers.js"
import {get_from_to as get_activities} from "/scripts/api/activities.js";
import {get_id as get_user_by_id} from "/scripts/api/users.js";
import {get as get_account} from "/scripts/api/account.js";

async function main() {
    let res = await get_account();
    if (!res.ok) {
        alert("You are not signed in. Sign in first.");
        window.location = "/auth/login.html";
    }

    // test to see if the browser supports the HTML template element by checking
    // for the presence of the template element's content attribute.
    if (!("content" in document.createElement("template"))) {
        alert("Your browser doesn't support templates. We cannot display the site properly. Try updating it or using a more up to date browser.");
        return;
    }

    let current_week = new Date();
    let feed_to_time = current_week.toRFC3339();
    let feed_from_time = new Date(new Date().setDate(current_week.getDate() - 7)).toRFC3339();

    const post_list = document.querySelector("#feed-list");
    const post_template = document.querySelector("#post-template");

    let activities = await get_activities(feed_from_time, feed_to_time);
    if (!activities.ok) {
        alert("Error while fetching Activities:\n" + activities.value);
        return;
    }

    // prepare the user by id map
    let user_by_id = new Map();
    for (const activity of activities.value) {
        if (user_by_id.has(activity.author_id)) {
            continue;
        }

        let user = await get_user_by_id(activity.author_id);
        if (!user.ok) {
            alert("Error while fetching Activities:\n" + activities.value);
        }
        user_by_id.set(activity.author_id, user.value);
    }

    for (const activity of activities.value) {
        let clone = post_template.content.cloneNode(true);

        const athlete_link = `/athletes/${activity.author_id}`;
        const author_name = user_by_id.get(activity.author_id).username;
        let duration = "";
        {
            const diff = new Date(activity.end_time) - new Date(activity.start_time);
            if (diff == 0) {
                duration = "no duration";
            }
            else {
                const ss = Math.floor(diff / 1000) % 60;
                const mm = Math.floor(diff / 1000 / 60) % 60;
                const hh = Math.floor(diff / 1000 / 60 / 60);

                // append all values after the first non zero value
                let tmp = [hh, mm, ss];
                for (let i = 0; i < tmp.length; ++i) {
                    if (tmp[i] > 0) {
                        for (let j = i; j < tmp.length; ++j) {
                            duration += `${String(tmp[j]).padStart(2, '0')}:`;
                        }
                        break;
                    }
                }
                duration = duration.substring(0, duration.length - 1); // remove last ':'
            }
        }

        clone.querySelector(".post-name").innerHTML = author_name;
        clone.querySelector(".post-athlete-link").href = athlete_link;
        clone.querySelector(".post-activity-type").innerHTML = activity.activity_type;
        clone.querySelector(".post-start-time").innerHTML = activity.start_time;
        clone.querySelector(".post-duration").innerHTML = duration;
        clone.querySelector(".post-distance").innerHTML = activity.amount;

        post_list.append(clone);
    }
}

await main();
