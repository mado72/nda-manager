import { computed, Injectable, Signal, signal } from '@angular/core';
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
  isClient(): boolean;
  isSupplier(): boolean;
}

@Injectable({
  providedIn: 'root'
})
export class ClientService {
  loggedClient = computed<Client | null>(() => {
    const currentUser = this.userService.currentUser();
    if (currentUser && currentUser.roles.includes('client' as UserRole)) {
      return {
        id: currentUser.id,
        name: currentUser.name,
        email: currentUser.username,
        password: '', // Password is not stored for security reasons
        stellar_public_key: currentUser.stellar_public_key,
        isClient: () => currentUser.roles.includes('client' as UserRole),
        isSupplier: () => currentUser.roles.includes('supplier' as UserRole)
      };
    }
    return null;
  });

  constructor(private userService: UserService) {
    this.loadLoggedClient();
  }

  private loadLoggedClient(): void {
    const data = JSON.parse(localStorage.getItem('loggedClient') || '{}') as Client;
    this.authenticateClient(data?.email, data?.password).subscribe();
  }

  private saveLoggedClient(preserve: boolean): void {
    if (preserve && this.loggedClient) {
      localStorage.setItem('loggedClient', JSON.stringify(this.loggedClient));
    } else {
      localStorage.removeItem('loggedClient');
    }
  }

  registerClient(name: string, email: string, password: string): Observable<Client> {
    return this.userService.register({ username: email, name, password, roles: ['client'] }).pipe(
      map((_) => {
        return this.loggedClient() as Client;
      }),
      catchError((error) => {
        console.error('Error registering user:', error);
        return throwError(() => new Error('Registration failed'));
      })
    );
  }

  authenticateClient(email: string, password: string, preserve?: boolean): Observable<Client> {
    return this.userService.login({ username: email, password }).pipe(
      map((user) => {
        if (user) {
          this.saveLoggedClient(preserve || false);
          return this.loggedClient() as Client;
        }
        throw new Error('Invalid credentials');
      }),
      catchError((error) => {
        console.error('Error during user login:', error);
        return throwError(() => new Error('Authentication failed'));
      })
    );
  }


  logout(): void {
    this.userService.logout();
  }
}
