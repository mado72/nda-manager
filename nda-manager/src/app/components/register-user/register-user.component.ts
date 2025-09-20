import { Component, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ClientService } from '../../services/client.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-register-user',
  standalone: true,
  imports: [FormsModule],
  templateUrl: './register-user.component.html',
  styleUrl: './register-user.component.scss'
})
export class RegisterUserComponent {
  name = signal('');
  email = signal('');
  password = signal('');
  message = signal('');

  constructor(private clientService: ClientService, private router: Router) {}

  register() {
    if (!this.name() || !this.email() || !this.password()) {
      this.message.set('Please fill all fields.');
      return;
    }
    this.clientService.registerClient(
      this.name(),
      this.email(),
      this.password()
    ).subscribe(client => {
      this.message.set(`User registered: ${client.name}`);
      this.name.set('');
      this.email.set('');
      this.password.set('');
    })
  }

}
