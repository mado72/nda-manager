import { Injectable, signal } from '@angular/core';
// import { HttpClient } from '@angular/common/http';
import { Observable, of } from 'rxjs';

export interface Contract {
  clientId: string;
  supplierId: string;
  status: string;
  data: any;
  title: string;
  description: string;
  hash: string;
}

@Injectable({ providedIn: 'root' })
export class ContractService {
  private apiUrl = 'http://localhost:8000/api'; // Rust API base URL

  contracts = signal<Contract[]>([]);
  loading = signal<boolean>(false);
  error = signal<string | null>(null);

  // constructor(private http: HttpClient) {}
  constructor() {}


  createContract(contractData: Partial<Contract>): Observable<Contract> {
    debugger;
    this.loading.set(true);
    // O id do contrato será o hash informado
    const newContract: Contract = {
      clientId: contractData.clientId || '',
      supplierId: contractData.supplierId || '',
      status: contractData.status || 'pending',
      data: contractData.data || {},
      title: contractData.title || '',
      description: contractData.description || '',
      hash: contractData.hash || '',
    };
    const current = this.contracts();
    this.contracts.set([...current, newContract]);
    this.loading.set(false);
    return of(newContract);
  }


  getContract(contractId: string): Observable<Contract> {
    this.loading.set(true);
    // Simulação de busca de contrato em memória
    const found = this.contracts().find(c => c.hash === contractId) || null;
    this.loading.set(false);
    return of(found as Contract);
  }


  listContracts(): Observable<Contract[]> {
    this.loading.set(true);
    // Simulação de listagem de contratos em memória
    this.loading.set(false);
    return of(this.contracts());
  }

  updateContract(contractId: string, contractData: Partial<Contract>): Observable<Contract | null> {
    this.loading.set(true);
    const contracts = this.contracts();
    const idx = contracts.findIndex(c => c.hash === contractId);
    if (idx === -1) {
      this.loading.set(false);
      return of(null);
    }
    // O id do contrato será o hash informado (se alterado)
    const updated: Contract = {
      ...contracts[idx],
      ...contractData,
    };
    contracts[idx] = updated;
    this.contracts.set([...contracts]);
    this.loading.set(false);
    return of(updated);
  }
}
