
export interface Contract {
  id?: string;
  clientId: string;
  partnerId: string;
  status: string;
  data: ContractURL | ContractContents | ContractModel;
  title: string;
  description: string;
  created_at: string;
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

// Contract Types
export type ContractType = 'url' | 'contents' | 'model';

export interface ContractURL {
  type: 'url';
  uri: string;
}

export interface ContractContents {
  type: 'contents';
  content: string;
}

export interface ContractModel {
  type: 'model';
  parties: PartyInfo[];
  scopeOfDiscussion: string;
  agreementValue: string;
  feeStructure: FeeStructureEntry[];
  proprietaryCompanyName: string;
  authorizedContactPerson: ContactInfo;
}

export type PartyType = 'disclosing' | 'receiving' | 'witness';

export interface PartyInfo extends ContactInfo {
  partyType: PartyType;
}

export interface ContactInfo {
  name: string;
  entityType: 'individual' | 'company';
  companyName?: string;
  address: Address;
  identification: IdentificationDocument;
  officialEmail: string;
  // Company representative fields
  companyEIN?: string; // EIN of the company being represented
  representativeDocument?: IdentificationDocument; // Document of the person representing the company
}

export interface Address {
  street: string;
  city: string;
  state: string;
  postalCode: string;
  country: string;
}

export interface IdentificationDocument {
  type: 'cpf' | 'cnpj' | 'passport' | 'other';
  number: string;
}

export interface FeeStructureEntry {
  partyName: string;
  role: 'disclosing' | 'receiving';
  feePercentage?: number;
  fixedFee?: number;
  description: string;
}
