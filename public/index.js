import {ping} from "/scripts/requests.js"

try {
    await ping();
    document.querySelector("#already_loggedin").style.display = "block";
}
catch (error) {}
