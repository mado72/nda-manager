import { Component, signal, OnInit, OnDestroy } from '@angular/core';
import { RouterModule } from '@angular/router';
import { CommonModule, JsonPipe, SlicePipe } from '@angular/common';
import { Subscription } from 'rxjs';
import { ContractService } from '../../services/contract.service';
import { Contract } from '../../models/contract.model';

@Component({
  selector: 'app-list-contracts',
  standalone: true,
  imports: [RouterModule, JsonPipe, SlicePipe],
  templateUrl: './list-contracts.component.html',
  styleUrl: './list-contracts.component.scss'
})
export class ListContractsComponent implements OnInit, OnDestroy {
  contracts = signal<Contract[]>([]);
  loading = signal<boolean>(false);
  error = signal<string | null>(null);
  
  private subscription: Subscription = new Subscription();

  constructor(private contractService: ContractService) {}

  ngOnInit() {
    this.fetchContracts();
  }

  ngOnDestroy() {
    this.subscription.unsubscribe();
  }

  fetchContracts() {
    this.loading.set(true);
    this.error.set(null);
    
    this.contractService.listContracts().subscribe({
      next: (data) => {
        this.contracts.set(data || []);
        this.loading.set(false);
      },
      error: (err) => {
        this.error.set('Error loading contracts. Please try again.');
        this.loading.set(false);
        console.error('Error fetching contracts:', err);
      }
    });
  }

  copyContractInfo(contract: Contract) {
    const info = `
Contract Information:
- Data: ${JSON.stringify(contract.data)}
- Title: ${contract.title}
- Status: ${contract.status}
- Client ID: ${contract.clientId}
- Supplier ID: ${contract.supplierId}
    `.trim();

    navigator.clipboard.writeText(info).then(() => {
      console.log('Contract info copied to clipboard');
      // TODO: Show success toast
    }).catch(err => {
      console.error('Failed to copy contract info:', err);
    });
  }
}
