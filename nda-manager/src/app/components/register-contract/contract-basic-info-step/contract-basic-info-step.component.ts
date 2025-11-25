import { Component, input, output } from '@angular/core';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-contract-basic-info-step',
  standalone: true,
  imports: [FormsModule],
  templateUrl: './contract-basic-info-step.component.html',
  styleUrl: './contract-basic-info-step.component.scss'
})
export class ContractBasicInfoStepComponent {
  title = input.required<string>();
  description = input.required<string>();
  loading = input<boolean>(false);

  titleChange = output<string>();
  descriptionChange = output<string>();

  onTitleChange(value: string) {
    this.titleChange.emit(value);
  }

  onDescriptionChange(value: string) {
    this.descriptionChange.emit(value);
  }
}