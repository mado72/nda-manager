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
  PartyInfo, 
  PartyType,
  Address, 
  IdentificationDocument, 
  FeeStructureEntry 
} from '../../models/contract.model';

// Wizard components
import { ContractWizardHeaderComponent } from './contract-wizard-header/contract-wizard-header.component';
import { ContractWizardStepIndicatorComponent } from './contract-wizard-step-indicator/contract-wizard-step-indicator.component';
import { ContractBasicInfoStepComponent } from './contract-basic-info-step/contract-basic-info-step.component';
import { ContractTypeSelectionStepComponent } from './contract-type-selection-step/contract-type-selection-step.component';
import { ContractUrlDetailsStepComponent } from './contract-url-details-step/contract-url-details-step.component';
import { ContractContentsDetailsStepComponent } from './contract-contents-details-step/contract-contents-details-step.component';
import { ContractModelDetailsStepComponent } from './contract-model-details-step/contract-model-details-step.component';
import { ContractWizardActionsComponent } from './contract-wizard-actions/contract-wizard-actions.component';


@Component({
  selector: 'app-register-contract',
  standalone: true,
  imports: [
    FormsModule, 
    MessageContainer,
    ContractWizardHeaderComponent,
    ContractWizardStepIndicatorComponent,
    ContractBasicInfoStepComponent,
    ContractTypeSelectionStepComponent,
    ContractUrlDetailsStepComponent,
    ContractContentsDetailsStepComponent,
    ContractModelDetailsStepComponent,
    ContractWizardActionsComponent
  ],
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
  parties = signal<PartyInfo[]>([this.createEmptyParty('disclosing'), this.createEmptyParty('receiving')]);
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
          // Handle backward compatibility
          if (data.parties) {
            this.parties.set(data.parties);
          } else {
            // Convert old structure to new
            const parties: PartyInfo[] = [];
            if (data.disclosingParties) {
              parties.push(...data.disclosingParties.map((p: ContactInfo) => ({ ...p, partyType: 'disclosing' as PartyType })));
            }
            if (data.receivingParties) {
              parties.push(...data.receivingParties.map((p: ContactInfo) => ({ ...p, partyType: 'receiving' as PartyType })));
            }
            this.parties.set(parties.length > 0 ? parties : [this.createEmptyParty('disclosing'), this.createEmptyParty('receiving')]);
          }
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
    const parties = this.parties();
    const disclosingParties = parties.filter(p => p.partyType === 'disclosing');
    const receivingParties = parties.filter(p => p.partyType === 'receiving');
    
    return !!(
      disclosingParties.length > 0 &&
      receivingParties.length > 0 &&
      this.scopeOfDiscussion() &&
      this.agreementValue() &&
      this.proprietaryCompanyName() &&
      this.isContactValid(this.authorizedContactPerson()) &&
      parties.every(party => this.isPartyValid(party))
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

  isPartyValid(party: PartyInfo): boolean {
    return !!(
      party.partyType &&
      party.name &&
      party.entityType &&
      party.address.street &&
      party.address.city &&
      party.address.state &&
      party.address.postalCode &&
      party.address.country &&
      party.identification.type &&
      party.identification.number &&
      party.officialEmail
    );
  }

  // Contract Type Selection
  selectContractType(type: ContractType) {
    this.selectedContractType.set(type);
  }

  // Helper Methods for Model Type
  createEmptyParty(partyType: PartyType): PartyInfo {
    return {
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
  }

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
          parties: this.parties(),
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
    this.parties.set([this.createEmptyParty('disclosing'), this.createEmptyParty('receiving')]);
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



  updateFeeEntry(index: number, field: keyof FeeStructureEntry, value: any) {
    const entries = [...this.feeStructure()];
    entries[index] = { ...entries[index], [field]: value };
    this.feeStructure.set(entries);
  }
}
