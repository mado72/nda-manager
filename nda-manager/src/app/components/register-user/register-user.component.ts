import { Component, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';
import { ClientService } from '../../services/client.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-register-user',
  standalone: true,
  imports: [FormsModule, RouterModule],
  templateUrl: './register-user.component.html',
  styleUrl: './register-user.component.scss'
})
export class RegisterUserComponent {
  name = signal('');
  email = signal('');
  password = signal('');
  message = signal('');
  loading = signal(false);

  constructor(private clientService: ClientService, private router: Router) {}

  onSubmit() {
    this.register();
  }

  register() {
    if (!this.name() || !this.email() || !this.password()) {
      this.message.set('Please fill all fields.');
      return;
    }

    this.loading.set(true);
    this.message.set('');

    this.clientService.registerClient(
      this.name(),
      this.email(),
      this.password()
    ).subscribe({
      next: _ => {
        this.message.set(`User registered: ${this.name()}`);
        this.loading.set(false);
        // Reset form
        this.name.set('');
        this.email.set('');
        this.password.set('');
        
        // Redirect to login after 2 seconds
        setTimeout(() => {
          this.router.navigate(['/login']);
        }, 500);
      },
      error: err => {
        this.message.set('Error registering user. Please try again.');
        this.loading.set(false);
      }
    });
  }
}
