import { Injectable, signal } from '@angular/core';
// import { HttpClient } from '@angular/common/http';
import { Observable, of } from 'rxjs';

export interface Contract {
  id: string;
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
    this.loading.set(true);
    // Simulação de criação de contrato em memória
    const newContract: Contract = {
      id: (Math.random() * 1000000).toFixed(0),
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
    const found = this.contracts().find(c => c.id === contractId) || null;
    this.loading.set(false);
    return of(found as Contract);
  }


  listContracts(): Observable<Contract[]> {
    this.loading.set(true);
    // Simulação de listagem de contratos em memória
    this.loading.set(false);
    return of(this.contracts());
  }
}
