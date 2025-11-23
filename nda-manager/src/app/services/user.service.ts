import { HttpClient } from '@angular/common/http';
import { Injectable, signal } from '@angular/core';
import { Observable, of, tap } from 'rxjs';
import { environment } from '../../environments/environment';
import { 
    AutoLoginRequest, 
    User, 
    UserLoginRequest, 
    UserRegisterRequest, 
    UserResponse, 
    UserRole,
    LoginResponse,
    RefreshTokenRequest,
    LogoutRequest
} from '../models/user.model';

@Injectable({ providedIn: 'root' })
export class UserService {
    private apiUrl = environment.apiUrl;
    private accessTokenKey = 'access_token';
    private refreshTokenKey = 'refresh_token';

    currentUser = signal<User | null>(null);

    constructor(private http: HttpClient) { 
    }

    register(data: UserRegisterRequest): Observable<UserResponse> {
        return this.http.post<UserResponse>(`${this.apiUrl}/api/users/register`, data);
    }

    login(data: UserLoginRequest, preserve?: boolean): Observable<LoginResponse> {
        return this.http.post<LoginResponse>(`${this.apiUrl}/api/users/login`, data)
            .pipe(
                tap(response => {
                    if (response && response.user) {
                        // Salvar tokens
                        this.setAccessToken(response.access_token);
                        this.setRefreshToken(response.refresh_token);

                        // Converter UserResponse para User
                        const user: User = {
                            id: response.user.id,
                            username: response.user.username,
                            name: response.user.name,
                            stellar_public_key: response.user.stellar_public_key,
                            roles: response.user.roles as UserRole[],
                            created_at: response.user.created_at
                        };
                        this.setCurrentUser(user);

                        // Salvar dados no localStorage para auto-login futuro
                        if (preserve) {
                            this.saveUserToLocalStorage(user);
                        }
                    }
                })
            );
    }

    refreshToken(): Observable<LoginResponse> {
        const refreshToken = this.getRefreshToken();
        if (!refreshToken) {
            throw new Error('No refresh token available');
        }

        const data: RefreshTokenRequest = { refresh_token: refreshToken };
        return this.http.post<LoginResponse>(`${this.apiUrl}/api/users/refresh`, data)
            .pipe(
                tap(response => {
                    if (response && response.user) {
                        // Atualizar tokens
                        this.setAccessToken(response.access_token);
                        this.setRefreshToken(response.refresh_token);

                        // Atualizar usuário
                        const user: User = {
                            id: response.user.id,
                            username: response.user.username,
                            name: response.user.name,
                            stellar_public_key: response.user.stellar_public_key,
                            roles: response.user.roles as UserRole[],
                            created_at: response.user.created_at
                        };
                        this.setCurrentUser(user);
                    }
                })
            );
    }

    logout(): Observable<any> {
        const token = this.getAccessToken();
        
        if (token) {
            const data: LogoutRequest = { token };
            return this.http.post(`${this.apiUrl}/api/users/logout`, data)
                .pipe(
                    tap(() => {
                        this.clearSession();
                    })
                );
        }

        this.clearSession();
        return of(null);
    }

    private clearSession(): void {
        this.currentUser.set(null);
        this.clearTokens();
        this.clearUserFromLocalStorage();
    }

    private setCurrentUser(user: User): void {
        this.currentUser.set(user);
    }

    isLoggedIn(): boolean {
        return this.currentUser() !== null && this.getAccessToken() !== null;
    }

    // Token management methods
    getAccessToken(): string | null {
        return localStorage.getItem(this.accessTokenKey);
    }

    private setAccessToken(token: string): void {
        localStorage.setItem(this.accessTokenKey, token);
    }

    getRefreshToken(): string | null {
        return localStorage.getItem(this.refreshTokenKey);
    }

    private setRefreshToken(token: string): void {
        localStorage.setItem(this.refreshTokenKey, token);
    }

    private clearTokens(): void {
        localStorage.removeItem(this.accessTokenKey);
        localStorage.removeItem(this.refreshTokenKey);
    }

    /**
     * Realiza login automático usando dados do localStorage
     * @param data Dados de auto-login com user_name e user_id
     * @returns Observable com resposta do usuário
     */
    autoLogin(data: AutoLoginRequest): Observable<UserResponse> {
        return this.http.post<UserResponse>(`${this.apiUrl}/api/users/auto-login`, data)
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

    /**
     * Tenta fazer auto-login usando dados salvos no localStorage
     * @returns Observable com resposta do usuário ou null se dados não existem
     */
    tryAutoLogin(): Observable<UserResponse | null> {
        const user_name = localStorage.getItem('user_name');
        const user_id = localStorage.getItem('user_id');

        if (!user_name || !user_id) {
            return of(null);
        }

        const autoLoginData: AutoLoginRequest = {
            user_name,
            user_id
        };

        return this.autoLogin(autoLoginData);
    }

    /**
     * Salva dados do usuário no localStorage para auto-login futuro
     * @param user Dados do usuário para salvar
     */
    saveUserToLocalStorage(user: User | UserResponse): void {
        localStorage.setItem('user_name', user.username);
        localStorage.setItem('user_id', user.id);
    }

    /**
     * Remove dados do usuário do localStorage
     */
    clearUserFromLocalStorage(): void {
        localStorage.removeItem('user_name');
        localStorage.removeItem('user_id');
    }

}
