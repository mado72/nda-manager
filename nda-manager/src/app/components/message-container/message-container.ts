import { Component, input, computed } from '@angular/core';
import { CommonModule } from '@angular/common';

export type MessageType = 'success' | 'error' | 'info';

@Component({
  selector: 'app-message-container',
  imports: [CommonModule],
  templateUrl: './message-container.html',
  styleUrl: './message-container.scss'
})
export class MessageContainer {
  // Inputs
  message = input.required<string>();
  type = input<MessageType>('info');
  
  // Computed properties for CSS classes and icon logic
  containerClasses = computed(() => {
    const baseClass = 'message-container';
    const typeClass = `${this.effectiveType()}-message`;
    return `${baseClass} ${typeClass}`;
  });

  // Determine message type automatically if not explicitly set
  autoDetectedType = computed((): MessageType => {
    const msg = this.message().toLowerCase();
    if (msg.includes('successfully') || msg.includes('success')) {
      return 'success';
    } else if (msg.includes('error') || msg.includes('please') || msg.includes('fail')) {
      return 'error';
    }
    return 'info';
  });

  // Use explicit type if provided, otherwise auto-detect
  effectiveType = computed(() => {
    return this.type() === 'info' ? this.autoDetectedType() : this.type();
  });

  // Icon path based on message type
  iconPath = computed(() => {
    switch (this.effectiveType()) {
      case 'success':
        return 'M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z';
      case 'error':
        return 'M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z';
      default: // info
        return 'M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z';
    }
  });
}
