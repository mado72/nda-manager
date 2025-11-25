import { Component, input, output } from '@angular/core';
import { ContractType } from '../../../models/contract.model';

@Component({
  selector: 'app-contract-type-selection-step',
  standalone: true,
  imports: [],
  templateUrl: './contract-type-selection-step.component.html',
  styleUrl: './contract-type-selection-step.component.scss'
})
export class ContractTypeSelectionStepComponent {
  selectedContractType = input<ContractType | null>(null);
  loading = input<boolean>(false);

  contractTypeChange = output<ContractType>();

  onSelectContractType(type: ContractType) {
    this.contractTypeChange.emit(type);
  }
}