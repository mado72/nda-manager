import { Component, input, output } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ContactInfo } from '../../../models/contract.model';

@Component({
  selector: 'app-contact-person-form',
  standalone: true,
  imports: [FormsModule],
  templateUrl: './contact-person-form.component.html',
  styleUrl: './contact-person-form.component.scss'
})
export class ContactPersonFormComponent {
  contact = input.required<ContactInfo>();
  loading = input<boolean>(false);
  showRemove = input<boolean>(false);
  title = input<string>('Contact Person');

  contactChange = output<ContactInfo>();
  remove = output<void>();

  onFieldChange(field: string, value: string) {
    const updated = { ...this.contact(), [field]: value };
    this.contactChange.emit(updated);
  }

  onAddressChange(field: string, value: string) {
    const updated = { 
      ...this.contact(), 
      address: { ...this.contact().address, [field]: value }
    };
    this.contactChange.emit(updated);
  }

  onIdentificationChange(field: string, value: string) {
    const updated = { 
      ...this.contact(), 
      identification: { ...this.contact().identification, [field]: value }
    };
    this.contactChange.emit(updated);
  }

  onPersonalIdentificationChange(field: string, value: string) {
    if (this.contact().entityType === 'company') {
      // For companies, update the representativeDocument
      const currentDoc = this.contact().representativeDocument || { type: 'cpf' as const, number: '' };
      const updated = { 
        ...this.contact(), 
        representativeDocument: { ...currentDoc, [field]: value }
      };
      this.contactChange.emit(updated);
    } else {
      // For individuals, update the main identification
      const updated = { 
        ...this.contact(), 
        identification: { ...this.contact().identification, [field]: value }
      };
      this.contactChange.emit(updated);
    }
  }

  onRemove() {
    this.remove.emit();
  }
}