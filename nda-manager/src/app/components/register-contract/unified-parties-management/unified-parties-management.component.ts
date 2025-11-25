import { Component, input, output, signal, computed } from '@angular/core';
import { PartyInfo, PartyType } from '../../../models/contract.model';
import { PartyEditModalComponent } from '../party-edit-modal/party-edit-modal.component';

@Component({
  selector: 'app-unified-parties-management',
  standalone: true,
  imports: [PartyEditModalComponent],
  templateUrl: './unified-parties-management.component.html',
  styleUrl: './unified-parties-management.component.scss'
})
export class UnifiedPartiesManagementComponent {
  parties = input.required<PartyInfo[]>();
  loading = input<boolean>(false);

  partiesChange = output<PartyInfo[]>();

  // Modal state
  isModalOpen = signal<boolean>(false);
  editingPartyIndex = signal<number>(-1);
  editingParty = signal<PartyInfo | undefined>(undefined);
  selectedPartyType = signal<PartyType>('disclosing');

  // Computed grouped parties
  groupedParties = computed(() => {
    const parties = this.parties();
    return {
      disclosing: parties.filter(p => p.partyType === 'disclosing'),
      receiving: parties.filter(p => p.partyType === 'receiving'),
      witness: parties.filter(p => p.partyType === 'witness')
    };
  });

  addParty(partyType: PartyType) {
    const emptyParty: PartyInfo = {
      partyType,
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
    
    this.editingParty.set(emptyParty);
    this.editingPartyIndex.set(-1);
    this.selectedPartyType.set(partyType);
    this.isModalOpen.set(true);
  }

  editParty(party: PartyInfo, index: number) {
    // Calculate the actual index in the full parties array
    const actualIndex = this.findActualIndex(party);
    this.editingParty.set({ ...party });
    this.editingPartyIndex.set(actualIndex);
    this.selectedPartyType.set(party.partyType);
    this.isModalOpen.set(true);
  }

  removeParty(party: PartyInfo) {
    const actualIndex = this.findActualIndex(party);
    if (actualIndex !== -1) {
      const updated = this.parties().filter((_, i) => i !== actualIndex);
      this.partiesChange.emit(updated);
    }
  }

  private findActualIndex(partyToFind: PartyInfo): number {
    return this.parties().findIndex(p => 
      p.name === partyToFind.name && 
      p.officialEmail === partyToFind.officialEmail &&
      p.partyType === partyToFind.partyType
    );
  }

  onModalClose() {
    this.isModalOpen.set(false);
    this.editingParty.set(undefined);
    this.editingPartyIndex.set(-1);
  }

  onModalSave(contact: any) {
    const party: PartyInfo = {
      ...contact,
      partyType: this.selectedPartyType()
    };

    const updated = [...this.parties()];
    const index = this.editingPartyIndex();
    
    if (index === -1) {
      // Adding new party
      updated.push(party);
    } else {
      // Updating existing party
      updated[index] = party;
    }
    
    this.partiesChange.emit(updated);
    this.onModalClose();
  }

  getIdentificationDisplay(party: PartyInfo): string {
    const typeMap = {
      'cpf': 'CPF',
      'cnpj': 'CNPJ',
      'passport': 'Passport',
      'other': 'Other'
    };
    return `${typeMap[party.identification.type]}: ${party.identification.number}`;
  }

  getEntityTypeDisplay(entityType: 'individual' | 'company'): string {
    return entityType === 'individual' ? 'Individual' : 'Company';
  }

  getPartyTypeDisplayName(partyType: PartyType): string {
    const typeMap = {
      'disclosing': 'Disclosing Parties',
      'receiving': 'Receiving Parties',
      'witness': 'Witness Parties'
    };
    return typeMap[partyType];
  }

  getPartyTypeSingular(partyType: PartyType): string {
    const typeMap = {
      'disclosing': 'Disclosing Party',
      'receiving': 'Receiving Party',
      'witness': 'Witness Party'
    };
    return typeMap[partyType];
  }
}