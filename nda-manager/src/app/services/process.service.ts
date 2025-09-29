import { Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';
import { Process, ProcessResponse, CreateProcessRequest } from '../models/process.model';
import { ProcessShare } from '../models/process-share.model';
import { ProcessAccessResponse, ProcessAccessWithDetails } from '../models/process-access.model';

@Injectable({ providedIn: 'root' })
export class ProcessService {
    private apiUrl = environment.apiUrl;

    constructor(private http: HttpClient) { }

    createProcess(data: CreateProcessRequest): Observable<ProcessResponse> {
        return this.http.post<ProcessResponse>(`${this.apiUrl}/api/processes`, data);
    }

    shareProcess(data: any): Observable<ProcessShare> {
        return this.http.post<ProcessShare>(`${this.apiUrl}/api/process/share`, data);
    }

    accessProcess(data: any): Observable<ProcessAccessResponse> {
        return this.http.post<ProcessAccessResponse>(`${this.apiUrl}/api/process/access`, data);
    }

    listProcesses(client_id: string): Observable<ProcessResponse[]> {
        const params = new HttpParams().set('client_id', client_id);
        return this.http.get<ProcessResponse[]>(`${this.apiUrl}/processes`, { params });
    }

    getNotifications(client_id: string): Observable<ProcessAccessWithDetails[]> {
        const params = new HttpParams().set('client_id', client_id);
        return this.http.get<ProcessAccessWithDetails[]>(`${this.apiUrl}/notifications`, { params });
    }
}
