import { Component, signal, Input, OnInit, input } from '@angular/core';
import { Router } from '@angular/router';
import { FormsModule } from '@angular/forms';
import { CommonModule } from '@angular/common';
import { ContractService } from '../../services/contract.service';
import { ClientService } from '../../services/client.service';


@Component({
  selector: 'app-register-contract',
  standalone: true,
  imports: [FormsModule, CommonModule],
  templateUrl: './register-contract.component.html',
  styleUrl: './register-contract.component.scss'
})
export class RegisterContractComponent implements OnInit {
  contractId = input<string | null>(null);
  title = signal('');
  description = signal('');
  uri = signal('');
  message = signal('');
  loading = signal(false);

  constructor(private contractService: ContractService, private router: Router, private clientService: ClientService) {}

  ngOnInit() {
    const contractIdValue = this.contractId();
    if (contractIdValue) {
      this.loading.set(true);
      this.contractService.getContract(contractIdValue).subscribe({
        next: contract => {
          if (contract) {
            this.title.set(contract.title);
            this.description.set(contract.description);
            this.uri.set(contract.data?.uri || '');
          }
          this.loading.set(false);
        },
        error: err => {
          this.message.set('Error loading contract.');
          this.loading.set(false);
        }
      });
    }
  }

  onSubmit() {
    this.registerContract();
  }

  registerContract() {
    if (!this.title() || !this.description() || !this.uri()) {
      this.message.set('Please fill all fields.');
      return;
    }
    this.loading.set(true);
    const contractIdValue = this.contractId();
    // Criar novo contrato
    this.contractService.createContract({
      clientId: this.clientService.loggedClient()?.id || '',
      title: this.title(),
      description: this.description(),
      data: {
        uri: this.uri()
      }
    }).subscribe({
      next: contract => {
        this.message.set('Contract registered successfully!');
        this.title.set('');
        this.description.set('');
        this.uri.set('');
        this.loading.set(false);
        setTimeout(() => {
          this.router.navigate(['/contracts']);
        }, 1200);
      },
      error: err => {
        this.message.set('Error registering contract.');
        this.loading.set(false);
      }
    });
  }

  onCancel() {
    this.router.navigate(['/contracts']);
  }
}
