import { Component, signal, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ContractService, Contract } from '../../services/contract.service';

@Component({
  selector: 'app-list-contracts',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './list-contracts.component.html',
  styleUrl: './list-contracts.component.scss'
})
export class ListContractsComponent {
  contracts = signal<Contract[]>([]);
  loading = signal<boolean>(false);
  error = signal<string | null>(null);

  constructor(private contractService: ContractService) {
    this.fetchContracts();
  }

  fetchContracts() {
    this.loading.set(true);
    this.contractService.listContracts().subscribe({
      next: (data) => {
        this.contracts.set(data);
        this.loading.set(false);
      },
      error: (err) => {
        this.error.set('Erro ao buscar contratos');
        this.loading.set(false);
      }
    });
  }
}
