import { Component, input, output, signal, OnInit, OnChanges } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { PartyInfo } from '../../../models/contract.model';

@Component({
  selector: 'app-party-edit-modal',
  standalone: true,
  imports: [FormsModule],
  templateUrl: './party-edit-modal.component.html',
  styleUrl: './party-edit-modal.component.scss'
})
export class PartyEditModalComponent implements OnInit, OnChanges {
  isOpen = input<boolean>(false);
  contact = input<PartyInfo>();
  loading = input<boolean>(false);
  title = input<string>('Edit Party');

  close = output<void>();
  save = output<PartyInfo>();

  private editingContact = signal<PartyInfo>({
    partyType: 'disclosing',
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
    officialEmail: '',
    companyEIN: '',
    representativeDocument: {
      type: 'cpf',
      number: ''
    }
  });

  ngOnInit() {
    if (this.contact()) {
      this.editingContact.set({ ...this.contact()! });
    }
  }

  ngOnChanges() {
    if (this.contact()) {
      this.editingContact.set({ ...this.contact()! });
    }
  }

  onClose() {
    this.close.emit();
  }

  onSave() {
    if (this.isValidForm()) {
      this.save.emit(this.editingContact());
      this.onClose();
    }
  }

  onFieldChange(field: keyof PartyInfo, value: any) {
    this.editingContact.update(contact => {
      const updatedContact = {
        ...contact,
        [field]: value
      };
      
      // Initialize fields when entity type changes
      if (field === 'entityType') {
        if (value === 'company') {
          // Initialize representative document if not exists
          if (!updatedContact.representativeDocument) {
            updatedContact.representativeDocument = {
              type: 'cpf',
              number: ''
            };
          }
          // Initialize company EIN if not exists
          if (!updatedContact.companyEIN) {
            updatedContact.companyEIN = '';
          }
        } else {
          // Clear company-specific fields for individuals
          updatedContact.companyEIN = undefined;
          updatedContact.representativeDocument = undefined;
        }
      }
      
      return updatedContact;
    });
  }

  onAddressChange(field: keyof PartyInfo['address'], value: string) {
    this.editingContact.update(contact => ({
      ...contact,
      address: {
        ...contact.address,
        [field]: value
      }
    }));
  }



  onRepresentativeDocumentChange(field: 'type' | 'number', value: string) {
    this.editingContact.update(contact => ({
      ...contact,
      representativeDocument: {
        type: field === 'type' ? value as any : contact.representativeDocument?.type || 'cpf',
        number: field === 'number' ? value : contact.representativeDocument?.number || ''
      }
    }));
  }

  private isValidForm(): boolean {
    const contact = this.editingContact();
    return !!(
      contact.partyType &&
      contact.name &&
      contact.entityType &&
      (contact.entityType === 'individual' || contact.companyName) &&
      contact.address.street &&
      contact.address.city &&
      contact.address.state &&
      contact.address.postalCode &&
      contact.address.country &&
      // For companies, ensure company EIN and representative document are filled
      (contact.entityType !== 'company' || (
        contact.companyEIN && 
        contact.companyEIN.trim() &&
        contact.representativeDocument &&
        contact.representativeDocument.type &&
        contact.representativeDocument.number &&
        contact.representativeDocument.number.trim()
      )) &&
      contact.officialEmail
    );
  }

  get editContact() {
    return this.editingContact();
  }
}