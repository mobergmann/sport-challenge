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
export function get() {
}

/// create a new account
/// :param account `NewAccount`
/// :return `Account` newly created account
export function create(account) {
}

/// edit an account
/// :param account `EditAccount`
/// :return `Account` edited account
export function edit(account) {
}

/// edit the password of an account
/// :param current_password the accounts current password
/// :param new_password the new password for the account
/// :return `Account` updated account
export function edit_password(current_password, new_password) {
}

/// deletes the account
/// :param current_password the accounts current password
/// :return `Account` deleted account
export function remove(current_password) {
}
