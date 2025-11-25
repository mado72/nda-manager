import { Component, input, output } from '@angular/core';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-contract-url-details-step',
  standalone: true,
  imports: [FormsModule],
  templateUrl: './contract-url-details-step.component.html',
  styleUrl: './contract-url-details-step.component.scss'
})
export class ContractUrlDetailsStepComponent {
  uri = input.required<string>();
  loading = input<boolean>(false);

  uriChange = output<string>();

  onUriChange(value: string) {
    this.uriChange.emit(value);
  }
}