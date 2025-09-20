import { Component, OnInit, OnDestroy, ViewChild, HostListener } from '@angular/core';
import { RouterModule, Router } from '@angular/router'; // ✅ Adicionar Router
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatMenuModule } from '@angular/material/menu';
import { MatSidenavModule } from '@angular/material/sidenav';
import { MatListModule } from '@angular/material/list';
import { MatDividerModule } from '@angular/material/divider';
import { MatSidenav } from '@angular/material/sidenav';
import { ContractService } from '../../services/contract.service'; // ✅ Adicionar
import { User, UserType } from '../../models/user.model';

@Component({
  selector: 'app-menu',
  standalone: true,
  imports: [
    RouterModule,
    MatToolbarModule,
    MatButtonModule,
    MatIconModule,
    MatMenuModule,
    MatSidenavModule,
    MatListModule,
    MatDividerModule
  ],
  templateUrl: './menu.component.html',
  styleUrls: ['./menu.component.scss']
})
export class MenuComponent implements OnInit, OnDestroy {
  @ViewChild('sidenav') sidenav!: MatSidenav;
  
  // ✅ ATUALIZAR: Usar dados dinâmicos do usuário
  userName = 'João Silva'; // Será atualizado dinamicamente
  isMobile = false;
  
  // ✅ NOVO: Propriedades para controle de permissões
  currentUser: User | null = null;
  canCreateContracts = false;
  canShareContracts = false;

  constructor(
    private router: Router, // ✅ Adicionar
    private contractService: ContractService // ✅ Adicionar
  ) {}

  ngOnInit() {
    this.checkScreenSize();
    this.loadUserInfo(); // ✅ Carregar informações do usuário
  }

  ngOnDestroy() {}

  @HostListener('window:resize', ['$event'])
  onResize(event: any) {
    this.checkScreenSize();
  }

  private checkScreenSize() {
    this.isMobile = window.innerWidth < 768;
  }

  // ✅ NOVO: Carregar informações do usuário
  loadUserInfo() {
    const user = this.contractService.getCurrentUser();
    this.currentUser = user;
    
    if (user) {
      this.userName = user.username;
      this.canCreateContracts = this.contractService.canCreateContracts();
      this.canShareContracts = this.contractService.canShareContracts();
      
      console.log('👤 User loaded:', user);
      console.log('🔨 Can create contracts:', this.canCreateContracts);
      console.log('🔗 Can share contracts:', this.canShareContracts);
    } else {
      // Fallback se não houver usuário logado
      this.userName = 'Guest';
      this.canCreateContracts = false;
      this.canShareContracts = false;
    }
  }

  // ✅ NOVO: Obter tipo de usuário para exibição
  getUserTypeDisplay(): string {
    if (!this.currentUser) {
      return 'Guest';
    }
    return this.currentUser.user_type === UserType.client ? 'Client' : 'Supplier';
  }

  // ✅ NOVO: Obter ícone do tipo de usuário
  getUserTypeIcon(): string {
    if (!this.currentUser) {
      return 'person';
    }
    return this.currentUser.user_type === UserType.client ? 'business' : 'inventory';
  }

  toggleSidenav() {
    if (this.sidenav) {
      this.sidenav.toggle();
    }
  }

  logout() {
    console.log('🚪 Logging out...');
    localStorage.removeItem('currentUser');
    this.router.navigate(['/login']);
  }

  displaySwitchUserTypeLabel(): string {
    if (!this.currentUser) {
      return 'Switch User Type';
    }
    return this.currentUser.user_type === UserType.client ? 'Switch to Supplier' : 'Switch to Client';
  }

  // ✅ NOVO: Método para debug - trocar tipo de usuário (remover em produção)
  switchUserType() {
    if (this.currentUser) {
      const newType = this.currentUser.user_type === UserType.client ? UserType.supplier : UserType.client;
      const newUser: User = {
        ...this.currentUser,
        user_type: newType
      };
      this.contractService.setCurrentUser(newUser);
      this.loadUserInfo();
      console.log('🔄 Switched user type to:', newType);
    }
  }
}