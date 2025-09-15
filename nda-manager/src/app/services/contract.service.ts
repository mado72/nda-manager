import { Injectable, signal } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

export interface Contract {
  id: string;
  clientId: string;
  supplierId: string;
  status: string;
  data: any;
}

@Injectable({ providedIn: 'root' })
export class ContractService {
  private apiUrl = 'http://localhost:8000/api'; // Rust API base URL

  contracts = signal<Contract[]>([]);
  loading = signal<boolean>(false);
  error = signal<string | null>(null);

  constructor(private http: HttpClient) {}

  createContract(contractData: Partial<Contract>): Observable<Contract> {
    this.loading.set(true);
    return this.http.post<Contract>(`${this.apiUrl}/contracts`, contractData);
  }

  getContract(contractId: string): Observable<Contract> {
    this.loading.set(true);
    return this.http.get<Contract>(`${this.apiUrl}/contracts/${contractId}`);
  }

  listContracts(): Observable<Contract[]> {
    this.loading.set(true);
    return this.http.get<Contract[]>(`${this.apiUrl}/contracts`);
  }
}
