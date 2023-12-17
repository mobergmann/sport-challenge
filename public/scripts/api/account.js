import {BASE_ACCOUNT_URL} from "./main.js"

export class Account {
    constructor(id, username, password_hash) {
        self.id = id;
        self.username = username;
        self.password_hash = password_hash;
    }
}

export class NewAccount {
    constructor(username, password) {
        self.username = username;
        self.password = password;
    }
}

export class EditAccount {
    constructor(username) {
        self.username = username;
    }
}

export class EditPassword {
    constructor(current_password, new_password) {
        self.current_password = current_password;
        self.new_password = new_password;
    }
}

export class PasswordValidation {
    constructor(current_password) {
        self.current_password = current_password;
    }
}

/// returns the current account
/// :return `Account`
export async function get() {
    const request = new Request(`${BASE_ACCOUNT_URL}`, {
        method: "GET",
        credentials: 'include',
    });

    try {
        let response = await fetch(request);
        return response.json();
    } catch (error) {
        return error;
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

    try {
        let response = await fetch(request);
        return response.json();
    } catch (error) {
        return error;
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

    try {
        let response = await fetch(request);
        return response.json();
    } catch (error) {
        return error;
    }
}

/// edit the password of an account
/// :param current_password the accounts current password
/// :param new_password the new password for the account
/// :return `Account` updated account
export async function edit_password(current_password, new_password) {
    const request = new Request(`${BASE_ACCOUNT_URL}`, {
        method: "PUT",
        headers: new Headers({
            "Content-Type": "application/json",
        }),
        body: JSON.stringify({
            current_password: current_password,
            new_password: new_password
        }),
        credentials: 'include',
    });

    try {
        let response = await fetch(request);
        return response.json();
    } catch (error) {
        return error;
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

    try {
        let response = await fetch(request);
        return response.json();
    } catch (error) {
        return error;
    }
}
