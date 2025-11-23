import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { FormsModule } from '@angular/forms';
import { RouterModule } from '@angular/router';
import { UserService } from '../../services/user.service';

@Component({
  selector: 'app-login-user',
  standalone: true,
  imports: [FormsModule, RouterModule],
  templateUrl: './login-user.component.html',
  styleUrl: './login-user.component.scss'
})
export class LoginUserComponent implements OnInit {
  user = {
    email: '',
    password: ''
  };
  
  rememberMe = false; // Controls whether user credentials are preserved in localStorage
  isLoading = false;
  errorMessage = '';
  successMessage = '';

  constructor(private userService: UserService, private router: Router) {}

  ngOnInit() {
    // Tenta fazer auto-login quando o componente inicializa
    this.tryAutoLogin();
  }

  private tryAutoLogin() {
    // Só tenta auto-login se não houver usuário logado
    if (!this.userService.isLoggedIn()) {
      this.userService.tryAutoLogin().subscribe({
        next: (user) => {
          if (user) {
            this.successMessage = 'Login automático realizado com sucesso!';
            setTimeout(() => {
              this.router.navigate(['/']);
            }, 1000);
          }
        },
        error: () => {
          // Erro no auto-login é silencioso - usuário pode fazer login manual
          console.log('Auto-login não disponível');
        }
      });
    }
  }

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

    this.userService.login({ username: this.user.email, password: this.user.password }, this.rememberMe).subscribe({
      next: (response) => {
        this.isLoading = false;
        if (response && response.user) {
          // Tokens are automatically saved by the service
          this.successMessage = 'Login successful!';
          console.log('Login successful:', response.user);
          setTimeout(() => {
            this.router.navigate(['/']);
          }, 1000);
        } else {
          this.errorMessage = 'Invalid credentials.';
        }
      },
      error: (error) => {
        this.isLoading = false;
        console.error('Login failed:', error);
        this.errorMessage = error.error?.message || 'An error occurred. Please try again.';
      }
    });
  }
}
