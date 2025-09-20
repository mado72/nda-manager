import { Injectable, signal } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';
import { User, UserRegisterRequest, UserLoginRequest, UserResponse } from '../models/user.model';

@Injectable({ providedIn: 'root' })
export class UserService {
    private apiUrl = environment.apiUrl;

    currentUser = signal<User | null>(null);

    constructor(private http: HttpClient) { }

    register(data: UserRegisterRequest): Observable<UserResponse> {
        console.log(data);
        debugger;
        return this.http.post<UserResponse>(`${this.apiUrl}/api/register`, data);
    }

    login(data: UserLoginRequest): Observable<UserResponse> {
        return this.http.post<UserResponse>(`${this.apiUrl}/api/login`, data);
    }

}
