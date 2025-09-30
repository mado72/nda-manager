export interface ProcessAccess {
    id: string;
    process_id: string;
    partner_id: string;
    accessed_at: string;
}

export interface ProcessAccessWithDetails {
    id: string | null;
    process_id: string;
    partner_id: string | null;
    accessed_at: string | null;
    process_title: string;
    process_description: string;
    process_status: string;
    partner_username: string | null;
}

export interface AccessProcessRequest {
    process_id: string;
    partner_public_key: string;
    partner_username: string;
}

export interface ProcessAccessResponse {
    process_id: string;
    title: string;
    description: string;
    content: string;
    accessed_at: string;
}
