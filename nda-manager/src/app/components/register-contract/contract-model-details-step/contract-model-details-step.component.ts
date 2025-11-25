import { Component, input, output } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ContactInfo, FeeStructureEntry } from '../../../models/contract.model';
import { ContactPersonFormComponent } from '../contact-person-form/contact-person-form.component';
import { ContractPartiesManagementComponent } from '../contract-parties-management/contract-parties-management.component';
import { ContractFeeStructureComponent } from '../contract-fee-structure/contract-fee-structure.component';

@Component({
  selector: 'app-contract-model-details-step',
  standalone: true,
  imports: [
    FormsModule, 
    ContactPersonFormComponent, 
    ContractPartiesManagementComponent,
    ContractFeeStructureComponent
  ],
  templateUrl: './contract-model-details-step.component.html',
  styleUrl: './contract-model-details-step.component.scss'
})
export class ContractModelDetailsStepComponent {
  // Basic fields
  proprietaryCompanyName = input.required<string>();
  scopeOfDiscussion = input.required<string>();
  agreementValue = input.required<string>();
  
  // Contact and parties
  authorizedContactPerson = input.required<ContactInfo>();
  disclosingParties = input.required<ContactInfo[]>();
  receivingParties = input.required<ContactInfo[]>();
  
  // Fee structure
  feeStructure = input.required<FeeStructureEntry[]>();
  
  loading = input<boolean>(false);

  // Outputs
  proprietaryCompanyNameChange = output<string>();
  scopeOfDiscussionChange = output<string>();
  agreementValueChange = output<string>();
  authorizedContactPersonChange = output<ContactInfo>();
  disclosingPartiesChange = output<ContactInfo[]>();
  receivingPartiesChange = output<ContactInfo[]>();
  feeStructureChange = output<FeeStructureEntry[]>();

  onProprietaryCompanyNameChange(value: string) {
    this.proprietaryCompanyNameChange.emit(value);
  }

  onScopeOfDiscussionChange(value: string) {
    this.scopeOfDiscussionChange.emit(value);
  }

  onAgreementValueChange(value: string) {
    this.agreementValueChange.emit(value);
  }

  onAuthorizedContactPersonChange(contact: ContactInfo) {
    this.authorizedContactPersonChange.emit(contact);
  }

  onDisclosingPartiesChange(parties: ContactInfo[]) {
    this.disclosingPartiesChange.emit(parties);
  }

  onReceivingPartiesChange(parties: ContactInfo[]) {
    this.receivingPartiesChange.emit(parties);
  }

  onFeeStructureChange(feeStructure: FeeStructureEntry[]) {
    this.feeStructureChange.emit(feeStructure);
  }
}