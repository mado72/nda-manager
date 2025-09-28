import { Injectable } from '@angular/core';
import { catchError, map, of, throwError } from 'rxjs';
import { Observable } from 'rxjs/internal/Observable';
import { User, UserRole } from '../models/user.model';
import { UserService } from './user.service';

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
  private loggedClient: Client | null = null;

  constructor(private userService: UserService) {
    this.loadLoggedClient();
  }

  private loadLoggedClient(): void {
    const data = JSON.parse(localStorage.getItem('loggedClient') || '{}') as Client;
    this.authenticateClient(data?.email, data?.password).subscribe({
      next: (client) => {
        this.loggedClient = client;
      },
      error: () => {
        this.loggedClient = null;
      }
    });
  }

  private saveLoggedClient(): void {
    if (this.loggedClient) {
      localStorage.setItem('loggedClient', JSON.stringify(this.loggedClient));
    } else {
      localStorage.removeItem('loggedClient');
    }
  }

  registerClient(name: string, email: string, password: string): Observable<Client> {
    return this.userService.register({ username: email, password, roles: ['client'] }).pipe(
      map((response) => {
        const newClient: Client = {
          id: response.stellar_public_key,
          name,
          email,
          password
        };
        return newClient;
      }),
      catchError((error) => {
        console.error('Error registering user:', error);
        return throwError(() => new Error('Registration failed'));
      })
    );
  }

  authenticateClient(email: string, password: string): Observable<Client> {
    return this.userService.login({ username: email, password }).pipe(
      map((user) => {
        if (user) {
          this.loggedClient = {
            id: user.id,
            name: user.username,
            email: email,
            password: password,
            stellar_public_key: user.stellar_public_key
          };
          return this.loggedClient;
        }
        throw new Error('Invalid credentials');
      }),
      catchError((error) => {
        console.error('Error during user login:', error);
        return throwError(() => new Error('Authentication failed'));
      })
    );
  }

  getLoggedClient(): Client | null {
    return this.loggedClient;
  }

  logout(): void {
    this.loggedClient = null;
    this.saveLoggedClient();
  }
}
