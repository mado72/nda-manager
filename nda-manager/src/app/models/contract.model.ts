
export interface Contract {
  id?: string;
  clientId: string;
  supplierId: string;
  status: string;
  data: any;
  title: string;
  description: string;
}

export interface ShareRequest {
  process_id: string;
  supplier_public_key: string;
  client_username: string;
}

export interface ShareResponse {
  success: boolean;
  message: string;
  shared_at?: string;
}
