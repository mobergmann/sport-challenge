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
