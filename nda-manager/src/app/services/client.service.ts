import { computed, inject, Injectable } from '@angular/core';
import { catchError, map, throwError } from 'rxjs';
import { Observable } from 'rxjs/internal/Observable';
import { UserRole } from '../models/user.model';
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

  private userService = inject(UserService);

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

  registerClient(name: string, email: string, password: string): Observable<void> {
    return this.userService.register({ username: email, name, password, roles: ['client'] }).pipe(
      map((response) => {
        if (response && response.id) {
          console.log('✅ User registered:', response);
        }
        else {
          throw new Error('Registration failed');
        }
      }),
      catchError((error) => {
        console.error('Error registering user:', error);
        return throwError(() => new Error('Registration failed'));
      })
    );
  }

}
