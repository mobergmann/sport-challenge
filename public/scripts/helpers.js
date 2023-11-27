/// @source: https://stackoverflow.com/a/31810991/11186407
Date.prototype.getWeek = function() {
    let onejan = new Date(this.getFullYear(), 0, 1);
    let today = new Date(this.getFullYear(), this.getMonth(), this.getDate());
    let dayOfYear = ((today - onejan + 86400000) / 86400000);
    return Math.ceil(dayOfYear / 7)
};

/// @source: https://stackoverflow.com/a/5210450/11186407
Date.prototype.getFirstWeekDay = function() {
    // strip time away and set date and time to the beginning of the week-day
    let curr = new Date(this.toDateString());
    curr.setHours(0);
    curr.setMinutes(0);
    curr.setSeconds(0);
    curr.setMilliseconds(0);
    let first = curr.getDate() - curr.getDay(); // First day is the day of the month - the day of the week
    return new Date(curr.setDate(first));
}

/// @source: https://stackoverflow.com/a/5210450/11186407
Date.prototype.getLastWeekDay = function() {
    // strip time away and set date and time to the end of the week-day
    let curr = new Date(this.toDateString());
    curr.setHours(23);
    curr.setMinutes(59);
    curr.setSeconds(59);
    curr.setMilliseconds(999);
    let first = curr.getDate() - curr.getDay(); // First day is the day of the month - the day of the week
    let last = first + 6; // last day is the first day + 6
    return new Date(curr.setDate(last));
}

/// @source: https://stackoverflow.com/a/5210450/11186407
Date.prototype.nextWeek = function() {
    this.setDate(this.getDate() + 7);
}

/// @source: https://stackoverflow.com/a/5210450/11186407
Date.prototype.previousWeek = function() {
    this.setDate(this.getDate() - 7);
}

Date.prototype.dateToHumanReadable = function() {
    return `${this.getDate()}.${this.getMonth() + 1}.${this.getFullYear()}`;
}

Date.prototype.getTimezoneString = function() {
    let tz_plus_minus = "";
    
    let timezone_offset = this.getTimezoneOffset() * (-1);
    if (timezone_offset >= 0) {
        tz_plus_minus += "+";
    } else if (timezone_offset < 0) {
        tz_plus_minus += "-";
    }

    let h = Math.trunc(timezone_offset / 60);
    let m = timezone_offset % 60;
    return `${tz_plus_minus}${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}`;
}

Date.prototype.toRFC3339 = function() {
    let iso = this.toISOString();
    iso = iso.substring(0, iso.length - 1);
    return `${iso}${this.getTimezoneString()}`;
}

/// calculates the sums of the elements of the array 
Array.prototype.sum = function() {
    let sum = 0;
    for (const element of this) {
        sum += element;
    }
    return sum;
}

/// calculates the sums of the elements of the array by using the specified key 
Array.prototype.sum = function(key) {
    let sum = 0;
    for (const element of this) {
        sum += element[key];
    }
    return sum;
}
