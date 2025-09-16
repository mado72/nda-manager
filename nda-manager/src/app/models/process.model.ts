export interface Process {
    id: string;
    client_id: string;
    title: string;
    encrypted_content: string;
    encryption_key: string;
    status: string;
    created_at: string;
}

export interface ProcessResponse {
    id: string;
    title: string;
    status: string;
    created_at: string;
}

export interface CreateProcessRequest {
    title: string;
    confidential_content: string;
    client_username: string;
}
