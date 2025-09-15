import { Component } from '@angular/core';
import { RouterModule } from '@angular/router';
import { Router } from '@angular/router';
import { ClientService } from '../../services/client.service';

@Component({
  selector: 'app-menu',
  standalone: true,
  imports: [RouterModule],
  templateUrl: './menu.component.html',
  styleUrl: './menu.component.scss'
})
export class MenuComponent {
  constructor(
    public clientService: ClientService,
    private router: Router
  ) {}

  get userName(): string | null {
    return this.clientService.getLoggedClient()?.name || null;
  }

  logout() {
    this.clientService.logout();
    this.router.navigate(['/login']);
  }
}
