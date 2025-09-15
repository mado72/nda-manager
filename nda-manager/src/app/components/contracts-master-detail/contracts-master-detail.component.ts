import { Component, signal } from '@angular/core';
import { RouterModule, ActivatedRoute } from '@angular/router';
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
  mode = signal<'list' | 'add' | 'edit'>('list');
  contractId = signal<string | null>(null);

  constructor(private route: ActivatedRoute) {
    this.route.paramMap.subscribe(params => {
      const contractId = params.get('contractId');
      const url = this.route.snapshot.url.map(u => u.path);
      if (url.length === 1 && url[0] === 'contracts') {
        this.mode.set('list');
        this.contractId.set(null);
      } else if (url.length === 2 && url[1] === 'add') {
        this.mode.set('add');
        this.contractId.set(null);
      } else if (url.length === 3 && url[1] === 'edit' && contractId) {
        this.mode.set('edit');
        this.contractId.set(contractId);
      } else {
        this.mode.set('list');
        this.contractId.set(null);
      }
    });
  }
}
