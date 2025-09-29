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
    description: string;
    status: string;
    created_at: string;
}

export interface CreateProcessRequest {
    title: string;
    description: string;
    confidential_content: string;
    client_id: string;
}
