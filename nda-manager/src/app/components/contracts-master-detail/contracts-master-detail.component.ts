import { Component, OnInit, signal } from '@angular/core';
import { RouterModule, ActivatedRoute, Router } from '@angular/router';
import { ListContractsComponent } from '../list-contracts/list-contracts.component';
import { RegisterContractComponent } from '../register-contract/register-contract.component';

@Component({
  selector: 'app-contracts-master-detail',
  standalone: true,
  imports: [RouterModule, ListContractsComponent, RegisterContractComponent],
  templateUrl: './contracts-master-detail.component.html',
  styleUrl: './contracts-master-detail.component.scss'
})
export class ContractsMasterDetailComponent implements OnInit {
  currentView = signal<'list' | 'add'>('list');

  constructor(
    private route: ActivatedRoute,
    private router: Router
  ) {}

  ngOnInit() {
    const { url } = this.router;
    console.log('🔍 Current URL:', url);

    if (url.includes('/contracts/add')) {
      this.currentView.set('add');
    } else {
      this.currentView.set('list');
    }

    console.log('🎯 Current view:', this.currentView());
  }
}
