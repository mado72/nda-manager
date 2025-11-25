import { Component, input, output, signal } from '@angular/core';
import { ContactInfo } from '../../../models/contract.model';
import { PartyEditModalComponent } from '../party-edit-modal/party-edit-modal.component';

@Component({
  selector: 'app-contract-parties-management',
  standalone: true,
  imports: [PartyEditModalComponent],
  templateUrl: './contract-parties-management.component.html',
  styleUrl: './contract-parties-management.component.scss'
})
export class ContractPartiesManagementComponent {
  parties = input.required<ContactInfo[]>();
  loading = input<boolean>(false);
  title = input.required<string>();
  singularTitle = input.required<string>();

  partiesChange = output<ContactInfo[]>();

  // Modal state
  isModalOpen = signal<boolean>(false);
  editingPartyIndex = signal<number>(-1);
  editingParty = signal<ContactInfo | undefined>(undefined);

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
    
    this.editingParty.set(emptyContact);
    this.editingPartyIndex.set(-1); // -1 indicates new party
    this.isModalOpen.set(true);
  }

  editParty(index: number) {
    this.editingParty.set({ ...this.parties()[index] });
    this.editingPartyIndex.set(index);
    this.isModalOpen.set(true);
  }

  removeParty(index: number) {
    const updated = this.parties().filter((_, i) => i !== index);
    this.partiesChange.emit(updated);
  }

  onModalClose() {
    this.isModalOpen.set(false);
    this.editingParty.set(undefined);
    this.editingPartyIndex.set(-1);
  }

  onModalSave(contact: ContactInfo) {
    const updated = [...this.parties()];
    const index = this.editingPartyIndex();
    
    if (index === -1) {
      // Adding new party
      updated.push(contact);
    } else {
      // Updating existing party
      updated[index] = contact;
    }
    
    this.partiesChange.emit(updated);
    this.onModalClose();
  }

  getIdentificationDisplay(contact: ContactInfo): string {
    const typeMap = {
      'cpf': 'CPF',
      'cnpj': 'CNPJ',
      'passport': 'Passport',
      'other': 'Other'
    };
    return `${typeMap[contact.identification.type]}: ${contact.identification.number}`;
  }

  getEntityTypeDisplay(entityType: 'individual' | 'company'): string {
    return entityType === 'individual' ? 'Individual' : 'Company';
  }
}