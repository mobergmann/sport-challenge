import {BASE_ACCOUNT_URL, STATUS, Result} from "./main.js"

export class Account {
    constructor(id, username, password_hash) {
        this.id = id;
        this.username = username;
        this.password_hash = password_hash;
    }
}

export class NewAccount {
    constructor(username, password) {
        this.username = username;
        this.password = password;
    }
}

export class EditAccount {
    constructor(username) {
        this.username = username;
    }
}

export class EditPassword {
    constructor(current_password, new_password) {
        this.current_password = current_password;
        this.new_password = new_password;
    }
}

export class PasswordValidation {
    constructor(current_password) {
        this.current_password = current_password;
    }
}

/// returns the current account
/// :return `Account`
export async function get() {
    const request = new Request(`${BASE_ACCOUNT_URL}`, {
        method: "GET",
        credentials: 'include',
    });

    let response = await fetch(request);
    if (response.status === STATUS.OK) {
        let value = await response.json();
        return new Result(true, new Account(value.id, value.username, value.password_hash));
    } else {
        let error = await response.text();
        return new Result(false, error);
    }
}

/// create a new account
/// :param account `NewAccount`
/// :return `Account` newly created account
export async function create(account) {
    const request = new Request(`${BASE_ACCOUNT_URL}`, {
        method: "POST",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
        body: JSON.stringify(account),
        credentials: 'include',
    });

    let response = await fetch(request);
    if (response.status === STATUS.CREATED) {
        let value = await response.json();
        return new Result(true, new Account(value.id, value.username, value.password_hash));
    } else {
        let error = await response.text();
        return new Result(false, error);
    }
}

/// edit an account
/// :param account `EditAccount`
/// :return `Account` edited account
export async function edit(account) {
    const request = new Request(`${BASE_ACCOUNT_URL}`, {
        method: "PUT",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
        body: JSON.stringify(account),
        credentials: 'include',
    });

    let response = await fetch(request);
    if (response.status === STATUS.OK) {
        let value = await response.json();
        return new Result(true, new Account(value.id, value.username, value.password_hash));
    } else {
        let error = await response.text();
        return new Result(false, error);
    }
}

/// edit the password of an account
/// :param current_password the accounts current password
/// :param new_password the new password for the account
/// :return `Account` updated account
export async function edit_password(edit_password) {
    const request = new Request(`${BASE_ACCOUNT_URL}/password`, {
        method: "PUT",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
        body: JSON.stringify({
            current_password: edit_password.current_password,
            new_password: edit_password.new_password
        }),
        credentials: 'include',
    });

    let response = await fetch(request);
    if (response.status === STATUS.OK) {
        let value = await response.json();
        return new Result(true, new Account(value.id, value.username, value.password_hash));
    } else {
        let error = await response.text();
        return new Result(false, error);
    }
}

/// deletes the account
/// :param current_password the accounts current password
/// :return `Account` deleted account
export async function remove(current_password) {
    const request = new Request(`${BASE_ACCOUNT_URL}`, {
        method: "DELETE",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
        body: JSON.stringify({current_password: current_password}),
        credentials: 'include',
    });

    let response = await fetch(request);
    if (response.status === STATUS.OK) {
        let value = await response.json();
        return new Result(true, new Account(value.id, value.username, value.password_hash));
    } else {
        let error = await response.text();
        return new Result(false, error);
    }
}
