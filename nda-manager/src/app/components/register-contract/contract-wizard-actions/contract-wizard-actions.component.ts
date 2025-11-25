import { Component, input, output } from '@angular/core';

@Component({
  selector: 'app-contract-wizard-actions',
  standalone: true,
  imports: [],
  templateUrl: './contract-wizard-actions.component.html',
  styleUrl: './contract-wizard-actions.component.scss'
})
export class ContractWizardActionsComponent {
  currentStep = input.required<number>();
  totalSteps = input.required<number>();
  loading = input<boolean>(false);
  canProceed = input<boolean>(false);
  isEdit = input<boolean>(false);

  previousStep = output<void>();
  nextStep = output<void>();
  cancel = output<void>();

  onPreviousStep() {
    this.previousStep.emit();
  }

  onNextStep() {
    this.nextStep.emit();
  }

  onCancel() {
    this.cancel.emit();
  }
}