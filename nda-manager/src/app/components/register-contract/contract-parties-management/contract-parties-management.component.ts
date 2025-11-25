import { Component, input, output } from '@angular/core';
import { ContactInfo } from '../../../models/contract.model';
import { ContactPersonFormComponent } from '../contact-person-form/contact-person-form.component';

@Component({
  selector: 'app-contract-parties-management',
  standalone: true,
  imports: [ContactPersonFormComponent],
  templateUrl: './contract-parties-management.component.html',
  styleUrl: './contract-parties-management.component.scss'
})
export class ContractPartiesManagementComponent {
  parties = input.required<ContactInfo[]>();
  loading = input<boolean>(false);
  title = input.required<string>();
  singularTitle = input.required<string>();

  partiesChange = output<ContactInfo[]>();

  addParty() {
    const emptyContact: ContactInfo = {
      name: '',
      entityType: 'individual',
      companyName: '',
      address: {
        street: '',
        city: '',
        state: '',
        postalCode: '',
        country: ''
      },
      identification: {
        type: 'cpf',
        number: ''
      },
      officialEmail: ''
    };
    
    this.partiesChange.emit([...this.parties(), emptyContact]);
  }

  removeParty(index: number) {
    const updated = this.parties().filter((_, i) => i !== index);
    this.partiesChange.emit(updated);
  }

  updateParty(index: number, contact: ContactInfo) {
    const updated = [...this.parties()];
    updated[index] = contact;
    this.partiesChange.emit(updated);
  }
}