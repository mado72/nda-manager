import { Component, signal } from '@angular/core';
import { RouterModule } from '@angular/router';
import { ListContractsComponent } from '../list-contracts/list-contracts.component';
import { RegisterContractComponent } from '../register-contract/register-contract.component';

@Component({
  selector: 'app-contracts-master-detail',
  standalone: true,
  imports: [RouterModule, ListContractsComponent, RegisterContractComponent],
  templateUrl: './contracts-master-detail.component.html',
  styleUrl: './contracts-master-detail.component.scss'
})
export class ContractsMasterDetailComponent {
  selectedContractId = signal<string | null>(null);

  selectContract(id: string) {
    this.selectedContractId.set(id);
  }

  clearSelection() {
    this.selectedContractId.set(null);
  }
}
