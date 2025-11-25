import { Component, input, output } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { FeeStructureEntry } from '../../../models/contract.model';

@Component({
  selector: 'app-contract-fee-structure',
  standalone: true,
  imports: [FormsModule],
  templateUrl: './contract-fee-structure.component.html',
  styleUrl: './contract-fee-structure.component.scss'
})
export class ContractFeeStructureComponent {
  feeStructure = input.required<FeeStructureEntry[]>();
  loading = input<boolean>(false);

  feeStructureChange = output<FeeStructureEntry[]>();

  addFeeEntry() {
    const emptyEntry: FeeStructureEntry = {
      partyName: '',
      role: 'disclosing',
      feePercentage: 0,
      fixedFee: 0,
      description: ''
    };
    
    this.feeStructureChange.emit([...this.feeStructure(), emptyEntry]);
  }

  removeFeeEntry(index: number) {
    const updated = this.feeStructure().filter((_, i) => i !== index);
    this.feeStructureChange.emit(updated);
  }

  updateFeeEntry(index: number, field: keyof FeeStructureEntry, value: any) {
    const updated = [...this.feeStructure()];
    updated[index] = { ...updated[index], [field]: value };
    this.feeStructureChange.emit(updated);
  }
}