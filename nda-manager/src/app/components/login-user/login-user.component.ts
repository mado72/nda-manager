import { Component } from '@angular/core';
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
  user = {
    email: '',
    password: ''
  };
  
  isLoading = false;
  errorMessage = '';
  successMessage = '';

  constructor(private clientService: ClientService, private router: Router) {}

  goToRegister() {
    this.router.navigate(['/register']);
  }

  onSubmit() {
    if (!this.user.email || !this.user.password) {
      this.errorMessage = 'Please enter email and password.';
      return;
    }

    this.isLoading = true;
    this.errorMessage = '';
    this.successMessage = '';

    this.clientService.authenticateClient(this.user.email, this.user.password).subscribe({
      next: (user) => {
        this.isLoading = false;
        if (user) {
          this.successMessage = 'Login successful!';
          setTimeout(() => {
            this.router.navigate(['/']);
          }, 1000);
        } else {
          this.errorMessage = 'Invalid credentials.';
        }
      },
      error: () => {
        this.isLoading = false;
        this.errorMessage = 'An error occurred. Please try again.';
      }
    });
  }
}
