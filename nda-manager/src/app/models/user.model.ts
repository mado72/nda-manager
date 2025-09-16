export interface User {
    id: string;
    username: string;
    stellar_public_key: string;
    user_type: string; // "client" ou "supplier"
    created_at: string; // ISO date string
}

export interface UserRegisterRequest {
    username: string;
    password: string;
    user_type: string;
}

export interface UserLoginRequest {
    username: string;
    password: string;
}

export interface UserResponse {
    id: string;
    username: string;
    stellar_public_key: string;
    user_type: string;
    created_at: string;
}
