export const BASE_URL = "/v1";
export const BASE_ACCOUNT_URL = `${BASE_URL}/account`;
export const BASE_USERS_URL = `${BASE_URL}/users`;
export const BASE_AUTH_URL = `${BASE_URL}/auth`;
export const BASE_ACTIVITIES_URL = `${BASE_URL}/activities`;

export const STATUS = {
    OK: 200,
    UNAUTHORIZED: 401,
    FORBIDDEN: 403,
    NOT_FOUND: 404,
    CONFLICT: 409,
    INTERNAL_SERVER_ERROR: 500,
};

export class Response {
    constructor(status, body) {
        this.status = status;
        this.body = body;
    }
}
