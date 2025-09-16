import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';
import { User, UserRegisterRequest, UserLoginRequest, UserResponse } from '../models/user.model';

@Injectable({ providedIn: 'root' })
export class UserService {
    private apiUrl = environment.apiUrl;

    constructor(private http: HttpClient) { }

    register(data: UserRegisterRequest): Observable<UserResponse> {
        return this.http.post<UserResponse>(`${this.apiUrl}/api/register`, data);
    }

    login(data: UserLoginRequest): Observable<UserResponse> {
        return this.http.post<UserResponse>(`${this.apiUrl}/api/login`, data);
    }
}
