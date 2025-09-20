import { Component, signal } from '@angular/core';
import { Router } from '@angular/router';
import { FormsModule } from '@angular/forms';
import { RouterModule } from '@angular/router';
import { ClientService } from '../../services/client.service';

@Component({
  selector: 'app-login-user',
  standalone: true,
  imports: [FormsModule, RouterModule],
  templateUrl: './login-user.component.html',
  styleUrl: './login-user.component.scss'
})
export class LoginUserComponent {
  email = signal('');
  password = signal('');
  message = signal('');
  loggedIn = signal(false);

  constructor(private clientService: ClientService, private router: Router) {}
  goToRegister() {
    this.router.navigate(['/register']);
  }

  login() {
    if (!this.email() || !this.password()) {
      this.message.set('Please enter email and password.');
      return;
    }
    this.clientService.authenticateClient(this.email(), this.password()).subscribe({
      next: (user) => {
        if (user) {
          this.loggedIn.set(true);
          this.message.set('Login successful!');
          setTimeout(() => {
            this.router.navigate(['/']);
          }, 500);
        } else {
          this.message.set('Invalid credentials.');
        }
      },
      error: () => {
        this.message.set('An error occurred. Please try again.');
      }
    });
  }
}
