import { Injectable } from '@angular/core';

export interface Client {
  id: number;
  name: string;
  email: string;
  password: string;
}

@Injectable({
  providedIn: 'root'
})
export class ClientService {
  private clients: Client[] = [];
  private loggedClient: Client | null = null;

  constructor() {
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

  registerClient(name: string, email: string, password: string): Client {
    const newClient: Client = {
      id: Date.now(),
      name,
      email,
      password
    };
    this.clients.push(newClient);
    this.saveClients();
    return newClient;
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
