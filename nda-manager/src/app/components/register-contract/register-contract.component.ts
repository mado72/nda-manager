import { Component, signal } from '@angular/core';
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
export class RegisterContractComponent {
  title = signal('');
  description = signal('');
  hash = signal('');
  message = signal('');
  loading = signal(false);

  constructor(private contractService: ContractService) {}

  registerContract() {
    if (!this.title() || !this.description() || !this.hash()) {
      this.message.set('Please fill all fields.');
      return;
    }
    this.loading.set(true);
    this.contractService.createContract({
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
      },
      error: err => {
        this.message.set('Error registering contract.');
        this.loading.set(false);
      }
    });
  }
}
