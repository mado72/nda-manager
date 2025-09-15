import { Component, signal } from '@angular/core';
import { RouterModule } from '@angular/router';
import { Contract, ContractService } from '../../services/contract.service';
import { JsonPipe } from '@angular/common';

@Component({
  selector: 'app-list-contracts',
  standalone: true,
  imports: [RouterModule, JsonPipe],
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
