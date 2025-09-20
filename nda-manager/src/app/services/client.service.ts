import { Injectable } from '@angular/core';
import { Observable } from 'rxjs/internal/Observable';
import { UserService } from './user.service';
import { map } from 'rxjs/internal/operators/map';
import { catchError } from 'rxjs/internal/operators/catchError';
import { throwError } from 'rxjs/internal/observable/throwError';
import { of } from 'rxjs';

export interface Client {
  id: string;
  name: string;
  email: string;
  password: string;
  stellar_public_key?: string;
}

@Injectable({
  providedIn: 'root'
})
export class ClientService {
  private clients: Client[] = [];
  private loggedClient: Client | null = null;

  constructor(private userService: UserService) {
    this.loadClients();
    this.loadLoggedClient();
  }

  private loadClients(): void {
    const data = localStorage.getItem('clients');
    this.clients = data ? JSON.parse(data) : [];
  }

  private saveClients(): void {
    localStorage.setItem('clients', JSON.stringify(this.clients));
  }

  private loadLoggedClient(): void {
    const data = localStorage.getItem('loggedClient');
    this.loggedClient = data ? JSON.parse(data) : null;
  }

  private saveLoggedClient(): void {
    if (this.loggedClient) {
      localStorage.setItem('loggedClient', JSON.stringify(this.loggedClient));
    } else {
      localStorage.removeItem('loggedClient');
    }
  }

  registerClient(name: string, email: string, password: string): Observable<Client> {
    const newClient: Client = {
      id: '',
      name,
      email,
      password
    };

    this.clients.push(newClient);
    return of(newClient);

    // return this.userService.register({ username: email, password, user_type: 'client' }).pipe(
    //   map((response) => {
    //     newClient.id = response.stellar_public_key; // Using stellar_public_key as id for simplicity
    //     this.clients.push(newClient);
    //     return newClient;
    //   }),
    //   catchError((error) => {
    //     console.error('Error registering user:', error);
    //     return throwError(() => new Error('Registration failed'));
    //   })
    // );
  }

  authenticateClient(email: string, password: string): boolean {
    const client = this.clients.find(c => c.email === email && c.password === password);
    if (client) {
      this.loggedClient = client;
      this.saveLoggedClient();
      return true;
    }
    return false;
  }

  getLoggedClient(): Client | null {
    return this.loggedClient;
  }

  logout(): void {
    this.loggedClient = null;
    this.saveLoggedClient();
  }
}
