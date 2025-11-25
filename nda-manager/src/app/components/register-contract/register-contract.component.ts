import { Component, signal, Input, OnInit, input } from '@angular/core';
import { Router } from '@angular/router';
import { FormsModule } from '@angular/forms';
import { ContractService } from '../../services/contract.service';
import { ClientService } from '../../services/client.service';
import { MessageContainer } from '../message-container/message-container';
import { 
  ContractType, 
  ContractURL, 
  ContractContents, 
  ContractModel, 
  ContactInfo, 
  Address, 
  IdentificationDocument, 
  FeeStructureEntry 
} from '../../models/contract.model';


@Component({
  selector: 'app-register-contract',
  standalone: true,
  imports: [FormsModule, MessageContainer],
  templateUrl: './register-contract.component.html',
  styleUrl: './register-contract.component.scss'
})
export class RegisterContractComponent implements OnInit {
  contractId = input<string | null>(null);
  
  // Wizard State
  currentStep = signal(1);
  totalSteps = 3;
  
  // Basic Contract Info (Step 1)
  title = signal('');
  description = signal('');
  
  // Contract Type Selection (Step 2)
  selectedContractType = signal<ContractType | null>(null);
  
  // Contract Type Data (Step 3)
  // URL Type
  uri = signal('');
  
  // Contents Type
  content = signal('');
  
  // Model Type
  disclosingParties = signal<ContactInfo[]>([this.createEmptyContact()]);
  receivingParties = signal<ContactInfo[]>([this.createEmptyContact()]);
  scopeOfDiscussion = signal('');
  agreementValue = signal('');
  feeStructure = signal<FeeStructureEntry[]>([this.createEmptyFeeEntry()]);
  proprietaryCompanyName = signal('');
  authorizedContactPerson = signal<ContactInfo>(this.createEmptyContact());
  
  message = signal('');
  loading = signal(false);

  constructor(private contractService: ContractService, private router: Router, private clientService: ClientService) {}

  ngOnInit() {
    const contractIdValue = this.contractId();
    if (contractIdValue) {
      this.loading.set(true);
      this.contractService.getContract(contractIdValue).subscribe({
        next: contract => {
          this.title.set(contract.title);
          this.description.set(contract.description);
          this.loadContractData(contract.data);
        },
        error: err => {
          this.message.set('Error loading contract: ' + (err.error?.message || err.message || 'Unknown error'));
        },
        complete: () => {
          this.loading.set(false);
        }
      });
    }
  }

  loadContractData(data: any) {
    if (data?.type) {
      this.selectedContractType.set(data.type);
      this.currentStep.set(3);
      
      switch (data.type) {
        case 'url':
          this.uri.set(data.uri || '');
          break;
        case 'contents':
          this.content.set(data.content || '');
          break;
        case 'model':
          this.disclosingParties.set(data.disclosingParties || [this.createEmptyContact()]);
          this.receivingParties.set(data.receivingParties || [this.createEmptyContact()]);
          this.scopeOfDiscussion.set(data.scopeOfDiscussion || '');
          this.agreementValue.set(data.agreementValue || '');
          this.feeStructure.set(data.feeStructure || [this.createEmptyFeeEntry()]);
          this.proprietaryCompanyName.set(data.proprietaryCompanyName || '');
          this.authorizedContactPerson.set(data.authorizedContactPerson || this.createEmptyContact());
          break;
      }
    } else {
      // Legacy data with uri
      this.selectedContractType.set('url');
      this.uri.set(data?.uri || '');
      this.currentStep.set(3);
    }
  }

  // Wizard Navigation
  nextStep() {
    if (this.canProceedToNextStep()) {
      this.currentStep.set(Math.min(this.currentStep() + 1, this.totalSteps));
      this.message.set('');
    }
  }

  previousStep() {
    this.currentStep.set(Math.max(this.currentStep() - 1, 1));
    this.message.set('');
  }

  canProceedToNextStep(): boolean {
    switch (this.currentStep()) {
      case 1:
        return !!this.title() && !!this.description();
      case 2:
        return !!this.selectedContractType();
      case 3:
        return this.isStep3Valid();
      default:
        return false;
    }
  }

  isStep3Valid(): boolean {
    switch (this.selectedContractType()) {
      case 'url':
        return !!this.uri();
      case 'contents':
        return !!this.content();
      case 'model':
        return this.isModelDataValid();
      default:
        return false;
    }
  }

  isModelDataValid(): boolean {
    return !!(
      this.disclosingParties().length > 0 &&
      this.receivingParties().length > 0 &&
      this.scopeOfDiscussion() &&
      this.agreementValue() &&
      this.proprietaryCompanyName() &&
      this.isContactValid(this.authorizedContactPerson()) &&
      this.disclosingParties().every(contact => this.isContactValid(contact)) &&
      this.receivingParties().every(contact => this.isContactValid(contact))
    );
  }

  isContactValid(contact: ContactInfo): boolean {
    return !!(
      contact.name &&
      contact.entityType &&
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

  // Contract Type Selection
  selectContractType(type: ContractType) {
    this.selectedContractType.set(type);
  }

  // Helper Methods for Model Type
  createEmptyContact(): ContactInfo {
    return {
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
  }

  createEmptyFeeEntry(): FeeStructureEntry {
    return {
      partyName: '',
      role: 'receiving',
      feePercentage: 0,
      fixedFee: 0,
      description: ''
    };
  }

  addDisclosingParty() {
    this.disclosingParties.set([...this.disclosingParties(), this.createEmptyContact()]);
  }

  removeDisclosingParty(index: number) {
    const parties = this.disclosingParties();
    if (parties.length > 1) {
      parties.splice(index, 1);
      this.disclosingParties.set([...parties]);
    }
  }

  addReceivingParty() {
    this.receivingParties.set([...this.receivingParties(), this.createEmptyContact()]);
  }

  removeReceivingParty(index: number) {
    const parties = this.receivingParties();
    if (parties.length > 1) {
      parties.splice(index, 1);
      this.receivingParties.set([...parties]);
    }
  }

  addFeeStructureEntry() {
    this.feeStructure.set([...this.feeStructure(), this.createEmptyFeeEntry()]);
  }

  removeFeeStructureEntry(index: number) {
    const entries = this.feeStructure();
    if (entries.length > 1) {
      entries.splice(index, 1);
      this.feeStructure.set([...entries]);
    }
  }

  // Form Submission
  onSubmit() {
    if (this.currentStep() < this.totalSteps) {
      this.nextStep();
    } else {
      this.registerContract();
    }
  }

  registerContract() {
    if (!this.canProceedToNextStep()) {
      this.message.set('Please fill all required fields.');
      return;
    }

    this.loading.set(true);
    
    let contractData: ContractURL | ContractContents | ContractModel;
    
    switch (this.selectedContractType()) {
      case 'url':
        contractData = {
          type: 'url',
          uri: this.uri()
        };
        break;
      case 'contents':
        contractData = {
          type: 'contents',
          content: this.content()
        };
        break;
      case 'model':
        contractData = {
          type: 'model',
          disclosingParties: this.disclosingParties(),
          receivingParties: this.receivingParties(),
          scopeOfDiscussion: this.scopeOfDiscussion(),
          agreementValue: this.agreementValue(),
          feeStructure: this.feeStructure(),
          proprietaryCompanyName: this.proprietaryCompanyName(),
          authorizedContactPerson: this.authorizedContactPerson()
        };
        break;
      default:
        this.loading.set(false);
        this.message.set('Please select a contract type.');
        return;
    }

    this.contractService.createContract({
      clientId: this.clientService.loggedClient()?.id || '',
      title: this.title(),
      description: this.description(),
      data: contractData
    }).subscribe({
      next: contract => {
        this.message.set('Contract registered successfully!');
        this.resetForm();
        setTimeout(() => {
          this.router.navigate(['/contracts']);
        }, 1200);
      },
      error: err => {
        this.message.set('Error registering contract: ' + (err.error?.message || err.message || 'Unknown error'));
      },
      complete: () => { 
        this.loading.set(false); 
      }
    });
  }

  resetForm() {
    this.currentStep.set(1);
    this.title.set('');
    this.description.set('');
    this.selectedContractType.set(null);
    this.uri.set('');
    this.content.set('');
    this.disclosingParties.set([this.createEmptyContact()]);
    this.receivingParties.set([this.createEmptyContact()]);
    this.scopeOfDiscussion.set('');
    this.agreementValue.set('');
    this.feeStructure.set([this.createEmptyFeeEntry()]);
    this.proprietaryCompanyName.set('');
    this.authorizedContactPerson.set(this.createEmptyContact());
  }

  onCancel() {
    this.router.navigate(['/contracts']);
  }

  // Helper methods for updating nested objects in templates
  updateAuthorizedContact(field: keyof ContactInfo, value: any) {
    const current = this.authorizedContactPerson();
    this.authorizedContactPerson.set({ ...current, [field]: value });
  }

  updateAuthorizedContactAddress(field: keyof Address, value: string) {
    const current = this.authorizedContactPerson();
    this.authorizedContactPerson.set({
      ...current,
      address: { ...current.address, [field]: value }
    });
  }

  updateAuthorizedContactIdentification(field: keyof IdentificationDocument, value: any) {
    const current = this.authorizedContactPerson();
    this.authorizedContactPerson.set({
      ...current,
      identification: { ...current.identification, [field]: value }
    });
  }

  updateDisclosingParty(index: number, field: keyof ContactInfo, value: any) {
    const parties = [...this.disclosingParties()];
    parties[index] = { ...parties[index], [field]: value };
    this.disclosingParties.set(parties);
  }

  updateReceivingParty(index: number, field: keyof ContactInfo, value: any) {
    const parties = [...this.receivingParties()];
    parties[index] = { ...parties[index], [field]: value };
    this.receivingParties.set(parties);
  }

  updateFeeEntry(index: number, field: keyof FeeStructureEntry, value: any) {
    const entries = [...this.feeStructure()];
    entries[index] = { ...entries[index], [field]: value };
    this.feeStructure.set(entries);
  }
}
