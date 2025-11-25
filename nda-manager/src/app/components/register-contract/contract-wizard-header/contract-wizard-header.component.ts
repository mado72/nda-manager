import { Component, input } from '@angular/core';

@Component({
  selector: 'app-contract-wizard-header',
  standalone: true,
  imports: [],
  templateUrl: './contract-wizard-header.component.html',
  styleUrl: './contract-wizard-header.component.scss'
})
export class ContractWizardHeaderComponent {
  isEdit = input<boolean>(false);
}