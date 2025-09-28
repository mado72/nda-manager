import { HttpClient } from '@angular/common/http';
import { Injectable, signal } from '@angular/core';
import { Observable, of, tap } from 'rxjs';
import { environment } from '../../environments/environment';
import { User, UserLoginRequest, UserRegisterRequest, UserResponse, UserRole } from '../models/user.model';

@Injectable({ providedIn: 'root' })
export class UserService {
    private apiUrl = environment.apiUrl;
    private readonly STORAGE_KEY = 'nda_current_user';

    currentUser = signal<User | null>(null);

    constructor(private http: HttpClient) { 
        this.loadUserFromStorage();
    }

    register(data: UserRegisterRequest): Observable<UserResponse> {
        console.log(data);
        debugger;
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
                            stellar_public_key: response.stellar_public_key,
                            roles: response.roles as UserRole[],
                            created_at: response.created_at
                        };
                        this.setCurrentUser(user);
                    }
                })
            );
    }

    logout(): Observable<void> {
        this.currentUser.set(null);
        localStorage.removeItem(this.STORAGE_KEY);
        return of(undefined);
    }

    setCurrentUser(user: User): void {
        this.currentUser.set(user);
        localStorage.setItem(this.STORAGE_KEY, JSON.stringify(user));
    }

    private loadUserFromStorage(): void {
        const userData = localStorage.getItem(this.STORAGE_KEY);
        if (userData) {
            try {
                const user = JSON.parse(userData);
                this.currentUser.set(user);
            } catch (error) {
                console.error('Error parsing user data from storage:', error);
                localStorage.removeItem(this.STORAGE_KEY);
            }
        }
    }

    isLoggedIn(): boolean {
        return this.currentUser() !== null;
    }

}
