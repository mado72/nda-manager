import { Component, OnInit, OnDestroy, ViewChild, HostListener, inject, computed } from '@angular/core';
import { RouterModule, Router } from '@angular/router'; // ✅ Adicionar Router
import { JsonPipe, NgTemplateOutlet } from '@angular/common'; // ✅ Adicionar NgTemplateOutlet
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatMenuModule } from '@angular/material/menu';
import { MatSidenavModule } from '@angular/material/sidenav';
import { MatListModule } from '@angular/material/list';
import { MatDividerModule } from '@angular/material/divider';
import { MatSidenav } from '@angular/material/sidenav';
import { ContractService } from '../../services/contract.service'; // ✅ Adicionar
import { ClientService } from '../../services/client.service';
import { User, UserRole, UserUtils } from '../../models/user.model';
import { UserService } from '../../services/user.service';


@Component({
  selector: 'app-menu',
  standalone: true,
  imports: [
    RouterModule,
    NgTemplateOutlet, // ✅ Adicionar NgTemplateOutlet
    MatToolbarModule,
    MatButtonModule,
    MatIconModule,
    MatMenuModule,
    MatSidenavModule,
    MatListModule,
    MatDividerModule,
    JsonPipe
  ],
  templateUrl: './menu.component.html',
  styleUrls: ['./menu.component.scss']
})
export class MenuComponent implements OnInit, OnDestroy {

  private router = inject(Router);
  private clientService = inject(ClientService);
  private userService = inject(UserService);
  private contractService = inject(ContractService);

  @ViewChild('sidenav') sidenav!: MatSidenav;
  
  // ✅ ATUALIZAR: Usar dados dinâmicos do usuário
  isMobile = false;
  
  // ✅ NOVO: Propriedades para controle de permissões
  canCreateContracts = false;
  canShareContracts = false;
  
  // ✅ NOVO: Propriedades para o novo sistema de roles
  isClient = false;
  isSupplier = false;
  hasMultipleRoles = false;

  ngOnInit() {
    this.checkScreenSize();
  }

  currentClient = computed(() => {
    const currentClient = this.clientService.loggedClient();
    if (currentClient) {
      // Atualizar propriedades baseadas em roles
      this.isClient = currentClient.isClient();
      this.isSupplier = currentClient.isSupplier();
      this.hasMultipleRoles = this.isClient && this.isSupplier;
      
      // Atualizar permissões baseadas em roles
      this.canCreateContracts = this.isClient;
      this.canShareContracts = this.isSupplier;
      
      this.contractService.getPermissions().subscribe(permissions => {
        this.canCreateContracts = permissions.canCreate && this.isClient;
        this.canShareContracts = permissions.canShare && this.isSupplier;
      });
    } else {
      // Fallback se não houver usuário logado
      this.isClient = false;
      this.isSupplier = false;
      this.hasMultipleRoles = false;
      this.canCreateContracts = false;
      this.canShareContracts = false;
    }

    console.log('👤 Client loaded:', currentClient);
    console.log('🔑 Is Client:', this.isClient);
    console.log('🏭 Is Supplier:', this.isSupplier);
    console.log('🔄 Has Multiple Roles:', this.hasMultipleRoles);
    console.log('🔨 Can create contracts:', this.canCreateContracts);
    console.log('🔗 Can share contracts:', this.canShareContracts);
    return currentClient;
  });

  userName = computed(() => {
    const currentUser = this.currentClient();
    return currentUser ? currentUser.name : 'Guest';
  });

  userTypeDisplay = computed(() => {
    const currentUser = this.currentClient();
    return currentUser ? this.isClient ? 'Client' : 'Supplier' : 'Guest';
  });

  userTypeIcon = computed(() => {
    const currentUser = this.currentClient();
    if (!currentUser) {
      return 'person';
    }
    
    // Ícone baseado nas roles
    if (this.isClient) {
      return 'supervisor_account'; // Ícone para múltiplas roles
    }
    
    if (this.isClient) {
      return 'business';
    }
    
    if (this.isSupplier) {
      return 'inventory';
    }
    
    return 'person';
  });

  displaySwitchUserTypeLabel = computed(() => {
    const currentUser = this.currentClient();
    if (!currentUser) {
      return 'Switch User Type';
    }
    
    // Para usuários com múltiplas roles
    if (this.isClient && this.isSupplier) {
      return 'Gerenciar Roles';
    }
    
    // Para usuários com uma única role
    if (this.isClient) {
      return 'Adicionar Role: Fornecedor';
    }
    
    if (this.isSupplier) {
      return 'Adicionar Role: Cliente';
    }
    
    return 'Definir Roles';
  });

  // ✅ NOVO: Computed para classes CSS baseadas em roles
  userRoleBadgeClass = computed(() => {
    const currentUser = this.currentClient();
    if (!currentUser) {
      return 'badge-guest';
    }
    
    // Para usuários com múltiplas roles
    if (this.isClient && this.isSupplier) {
      return 'badge-multiple';
    }
    
    // Para usuários com uma única role
    if (this.isClient) {
      return 'badge-client';
    }
    
    if (this.isSupplier) {
      return 'badge-supplier';
    }
    
    return 'badge-unknown';
  });

  ngOnDestroy() {}

  @HostListener('window:resize', ['$event'])
  onResize(event: any) {
    this.checkScreenSize();
  }

  private checkScreenSize() {
    this.isMobile = window.innerWidth < 768;
  }

  toggleSidenav() {
    if (this.sidenav) {
      this.sidenav.toggle();
    }
  }

  logout = () => {
    console.log('🚪 Logging out...');
    this.userService.logout();
    this.router.navigate(['/login']);
  }

  // ✅ NOVO: Método para alternar roles do usuário (modo debug)
  switchUserType = () => {
    const currentUser = this.currentClient();
    if (currentUser) {
      let newRoles: UserRole[];
      
      if (this.isClient && this.isSupplier) {
        // Se tem múltiplas roles, remover uma
        if (this.isClient) {
          newRoles = ['partner'];
        } else {
          newRoles = ['client'];
        }
      } else if (this.isClient) {
        // Se é só cliente, adicionar partner
        newRoles = ['client', 'partner'];
      } else if (this.isSupplier) {
        // Se é só partner, adicionar client
        newRoles = ['client', 'partner'];
      } else {
        // Fallback - definir como cliente
        newRoles = ['client'];
      }
      
      const currentUser = this.userService.currentUser();
      if (currentUser) {

        const newUser: User = {
          ...currentUser,
          roles: newRoles
        };
        
        this.userService.currentUser.set(newUser);
        console.log('🔄 Switched user roles to:', newRoles);
        console.log('📝 New role description:', UserUtils.getRoleDescription(newUser));
      }
    }
  }
}