import { inject, Injectable, signal } from '@angular/core';
// import { HttpClient } from '@angular/common/http';
import { catchError, Observable, of, tap } from 'rxjs';
import { Contract, ShareRequest, ShareResponse } from '../models/contract.model';
import { environment } from '../../environments/environment';
import { HttpClient } from '@angular/common/http';

@Injectable({ providedIn: 'root' })
export class ContractService {
  private http = inject(HttpClient);
  contracts = signal<Contract[]>([]);
  loading = signal<boolean>(false);
  error = signal<string | null>(null);

  // constructor(private http: HttpClient) {}
  constructor() {}


  createContract(contractData: Partial<Contract>): Observable<Contract> {
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

  
  shareContract(shareData: ShareRequest): Observable<ShareResponse> {
    console.log('🔗 Sharing contract:', shareData);

    const response: ShareResponse = {
      success: true,
      message: 'Contract shared successfully',
      shared_at: new Date().toISOString()
    };

    return of(response);

    // Descomente o código abaixo para fazer a chamada real à API
    //
    
    // return this.http.post<ShareResponse>(`${environment.apiUrl}/processes/share`, shareData).pipe(
    //   tap(response => {
    //     console.log('✅ Contract shared successfully:', response);
    //   }),
    //   catchError(error => {
    //     console.error('❌ Error sharing contract:', error);
    //     throw error;
    //   })
    // );
  }


  getPermissions(): Observable<{ canCreate: boolean; canShare: boolean }> {
    // Simulação de verificação de permissões
    const permissions = {
      canCreate: true,
      canShare: true
    };
    return of(permissions);
  }

}
