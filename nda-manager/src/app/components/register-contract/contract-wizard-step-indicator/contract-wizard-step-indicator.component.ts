import { Component, input } from '@angular/core';

@Component({
  selector: 'app-contract-wizard-step-indicator',
  standalone: true,
  imports: [],
  templateUrl: './contract-wizard-step-indicator.component.html',
  styleUrl: './contract-wizard-step-indicator.component.scss'
})
export class ContractWizardStepIndicatorComponent {
  currentStep = input.required<number>();
}