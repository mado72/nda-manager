export interface ProcessAccess {
    id: string;
    process_id: string;
    supplier_id: string;
    accessed_at: string;
}

export interface ProcessAccessWithDetails {
    id: string;
    process_id: string;
    supplier_id: string;
    accessed_at: string;
    process_title: string;
    supplier_username: string;
}

export interface AccessProcessRequest {
    process_id: string;
    supplier_public_key: string;
    supplier_username: string;
}

export interface ProcessAccessResponse {
    process_id: string;
    title: string;
    description: string;
    content: string;
    accessed_at: string;
}
