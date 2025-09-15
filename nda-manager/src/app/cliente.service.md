// Exemplo de uso do ClienteService em um componente Angular
import { Component } from '@angular/core';
import { ClienteService } from './cliente.service';

@Component({
  selector: 'app-cadastro-cliente',
  template: `
    <form (ngSubmit)="cadastrar()">
      <input [(ngModel)]="nome" name="nome" placeholder="Nome" required />
      <input [(ngModel)]="email" name="email" placeholder="Email" required />
      <input [(ngModel)]="senha" name="senha" type="password" placeholder="Senha" required />
      <button type="submit">Cadastrar</button>
    </form>
    <div *ngIf="mensagem">{{ mensagem }}</div>
  `
})
export class CadastroClienteComponent {
  nome = '';
  email = '';
  senha = '';
  mensagem = '';

  constructor(private clienteService: ClienteService) {}

  cadastrar() {
    const cliente = this.clienteService.cadastrarCliente(this.nome, this.email, this.senha);
    this.mensagem = `Cliente cadastrado: ${cliente.nome}`;
  }
}

// Para autenticação, utilize:
// this.clienteService.autenticarCliente(email, senha);
// Retorna true se autenticado, false caso contrário.