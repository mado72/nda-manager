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
    officialEmail: ''
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
    this.editingContact.update(contact => ({
      ...contact,
      [field]: value
    }));
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

  onIdentificationChange(field: keyof PartyInfo['identification'], value: string) {
    this.editingContact.update(contact => ({
      ...contact,
      identification: {
        ...contact.identification,
        [field]: value
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
      contact.identification.type &&
      contact.identification.number &&
      contact.officialEmail
    );
  }

  get editContact() {
    return this.editingContact();
  }
}