import { inject, Injectable, signal } from '@angular/core';
// import { HttpClient } from '@angular/common/http';
import { Observable, of } from 'rxjs';
import { catchError, finalize, map } from 'rxjs/operators';
import { Contract, ShareRequest, ShareResponse } from '../models/contract.model';
import { ClientService } from './client.service';
import { ProcessService } from './process.service';

@Injectable({ providedIn: 'root' })
export class ContractService {
  private clientService = inject(ClientService);
  private processService = inject(ProcessService);
  contracts = signal<Contract[]>([]);
  loading = signal<boolean>(false);
  error = signal<string | null>(null);

  // constructor(private http: HttpClient) {}
  constructor() {}


  createContract(contractData: Partial<Contract>): Observable<Contract | null> {
    this.loading.set(true);

    const client = this.clientService.loggedClient();
    if (!client) {
      this.error.set('No logged client found');
      this.loading.set(false);
      return of(null as any);
    }

    if (!contractData.title || !contractData.description) {
      this.error.set('Missing required contract fields');
      this.loading.set(false);
      return of(null);
    }

    const newContract: Contract = {
      clientId: client.id,
      supplierId: contractData.supplierId || '',
      status: contractData.status || 'pending',
      data: { ...(contractData.data || {}),
        created_at: new Date().toISOString()
      },
      title: contractData.title,
      description: contractData.description,
    };

    return this.processService.createProcess({
      title: newContract.title,
      description: newContract.description,
      client_id: newContract.clientId,
      confidential_content: JSON.stringify(newContract || {}),
    }).pipe(
      map((processResponse: any) => {
        console.log('✅ Process created:', processResponse);
        this.error.set(null);
        newContract.status = processResponse.status;
        newContract.id = processResponse.id;

        const current = this.contracts();
        this.contracts.set([...current, newContract]);
        return newContract;
      }),
      catchError((err: any) => {
        console.error('❌ Error creating process:', err);
        this.error.set('Error creating process');
        return of(null);
      }),
      finalize(() => {
        this.loading.set(false);
      })
    );
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
