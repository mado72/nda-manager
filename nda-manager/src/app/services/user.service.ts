import { HttpClient } from '@angular/common/http';
import { Injectable, signal } from '@angular/core';
import { Observable, of, tap } from 'rxjs';
import { environment } from '../../environments/environment';
import { User, UserLoginRequest, UserRegisterRequest, UserResponse, UserRole } from '../models/user.model';

@Injectable({ providedIn: 'root' })
export class UserService {
    private apiUrl = environment.apiUrl;

    currentUser = signal<User | null>(null);

    constructor(private http: HttpClient) { 
    }

    register(data: UserRegisterRequest): Observable<UserResponse> {
        return this.http.post<UserResponse>(`${this.apiUrl}/api/users/register`, data);
    }

    login(data: UserLoginRequest): Observable<UserResponse> {
        return this.http.post<UserResponse>(`${this.apiUrl}/api/users/login`, data)
            .pipe(
                tap(response => {
                    if (response && response.id) {
                        // Converter UserResponse para User
                        const user: User = {
                            id: response.id,
                            username: response.username,
                            name: response.name,
                            stellar_public_key: response.stellar_public_key,
                            roles: response.roles as UserRole[],
                            created_at: response.created_at
                        };
                        this.setCurrentUser(user);
                    }
                })
            );
    }

    logout() {
        this.currentUser.set(null);
    }

    private setCurrentUser(user: User): void {
        this.currentUser.set(user);
    }

    isLoggedIn(): boolean {
        return this.currentUser() !== null;
    }

}
