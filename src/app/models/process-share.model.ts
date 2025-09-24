export interface ProcessShare {
    id: string;
    process_id: string;
    supplier_public_key: string;
    stellar_transaction_hash: string;
    shared_at: string;
}

export interface ShareProcessRequest {
    process_id: string;
    supplier_public_key: string;
    client_username: string;
}
