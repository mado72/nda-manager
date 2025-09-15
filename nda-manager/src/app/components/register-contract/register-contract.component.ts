import { Component, signal, Input, OnInit, input } from '@angular/core';
import { Router } from '@angular/router';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ContractService } from '../../services/contract.service';


@Component({
  selector: 'app-register-contract',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './register-contract.component.html',
  styleUrl: './register-contract.component.scss'
})
export class RegisterContractComponent implements OnInit {
  contractId = input<string | null>(null);
  userId = input<string>('');
  title = signal('');
  description = signal('');
  hash = signal('');
  message = signal('');
  loading = signal(false);

  constructor(private contractService: ContractService, private router: Router) {}

  ngOnInit() {
    const contractIdValue = this.contractId();
    if (contractIdValue) {
      this.loading.set(true);
      this.contractService.getContract(contractIdValue).subscribe({
        next: contract => {
          if (contract) {
            this.title.set(contract.title);
            this.description.set(contract.description);
            this.hash.set(contract.hash);
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

  registerContract() {
    if (!this.title() || !this.description() || !this.hash()) {
      this.message.set('Please fill all fields.');
      return;
    }
    this.loading.set(true);
    const contractIdValue = this.contractId();
    if (contractIdValue) {
      // Atualizar contrato existente
      this.contractService.updateContract(contractIdValue, {
        title: this.title(),
        description: this.description(),
        hash: this.hash()
      }).subscribe({
        next: contract => {
          this.message.set('Contract updated successfully!');
          this.loading.set(false);
          setTimeout(() => {
            this.router.navigate(['/contract-master-detail']);
          }, 1200);
        },
        error: err => {
          this.message.set('Error updating contract.');
          this.loading.set(false);
        }
      });
    } else {
      // Criar novo contrato
      this.contractService.createContract({
        clientId: this.userId(),
        title: this.title(),
        description: this.description(),
        hash: this.hash()
      }).subscribe({
        next: contract => {
          this.message.set('Contract registered successfully!');
          this.title.set('');
          this.description.set('');
          this.hash.set('');
          this.loading.set(false);
          setTimeout(() => {
            this.router.navigate(['/contract-master-detail']);
          }, 1200);
        },
        error: err => {
          this.message.set('Error registering contract.');
          this.loading.set(false);
        }
      });
    }
  }
}
