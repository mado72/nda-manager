
export interface Contract {
  id?: string;
  clientId: string;
  partnerId: string;
  status: string;
  data: any;
  title: string;
  description: string;
}

export interface ShareRequest {
  process_id: string;
  partner_public_key: string;
  client_username: string;
}

export interface ShareResponse {
  success: boolean;
  message: string;
  shared_at?: string;
}
