import { Component, input, output } from '@angular/core';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-contract-contents-details-step',
  standalone: true,
  imports: [FormsModule],
  templateUrl: './contract-contents-details-step.component.html',
  styleUrl: './contract-contents-details-step.component.scss'
})
export class ContractContentsDetailsStepComponent {
  content = input.required<string>();
  loading = input<boolean>(false);

  contentChange = output<string>();

  onContentChange(value: string) {
    this.contentChange.emit(value);
  }
}