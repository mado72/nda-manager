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
import { User, UserType, UserUtils, UserRole } from '../../models/user.model';
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

  currentUser = computed(() => {
    const currentUser = this.userService.currentUser();
    if (currentUser) {
      // Atualizar propriedades baseadas em roles
      this.isClient = UserUtils.isClient(currentUser);
      this.isSupplier = UserUtils.isSupplier(currentUser);
      this.hasMultipleRoles = UserUtils.hasMultipleRoles(currentUser);
      
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

    console.log('👤 User loaded:', currentUser);
    console.log('🔑 Is Client:', this.isClient);
    console.log('🏭 Is Supplier:', this.isSupplier);
    console.log('🔄 Has Multiple Roles:', this.hasMultipleRoles);
    console.log('🔨 Can create contracts:', this.canCreateContracts);
    console.log('🔗 Can share contracts:', this.canShareContracts);
    return currentUser;
  });

  userName = computed(() => {
    const currentUser = this.currentUser();
    return currentUser ? currentUser.username : 'Guest';
  });

  userTypeDisplay = computed(() => {
    const currentUser = this.currentUser();
    return currentUser ? UserUtils.getRoleDescription(currentUser) : 'Guest';
  });

  userTypeIcon = computed(() => {
    const currentUser = this.currentUser();
    if (!currentUser) {
      return 'person';
    }
    
    // Ícone baseado nas roles
    if (UserUtils.hasMultipleRoles(currentUser)) {
      return 'supervisor_account'; // Ícone para múltiplas roles
    }
    
    if (UserUtils.isClient(currentUser)) {
      return 'business';
    }
    
    if (UserUtils.isSupplier(currentUser)) {
      return 'inventory';
    }
    
    return 'person';
  });

  displaySwitchUserTypeLabel = computed(() => {
    const currentUser = this.currentUser();
    if (!currentUser) {
      return 'Switch User Type';
    }
    
    // Para usuários com múltiplas roles
    if (UserUtils.hasMultipleRoles(currentUser)) {
      return 'Gerenciar Roles';
    }
    
    // Para usuários com uma única role
    if (UserUtils.isClient(currentUser)) {
      return 'Adicionar Role: Fornecedor';
    }
    
    if (UserUtils.isSupplier(currentUser)) {
      return 'Adicionar Role: Cliente';
    }
    
    return 'Definir Roles';
  });

  // ✅ NOVO: Computed para classes CSS baseadas em roles
  userRoleBadgeClass = computed(() => {
    const currentUser = this.currentUser();
    if (!currentUser) {
      return 'badge-guest';
    }
    
    // Para usuários com múltiplas roles
    if (UserUtils.hasMultipleRoles(currentUser)) {
      return 'badge-multiple';
    }
    
    // Para usuários com uma única role
    if (UserUtils.isClient(currentUser)) {
      return 'badge-client';
    }
    
    if (UserUtils.isSupplier(currentUser)) {
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
    this.userService.logout().subscribe(() => {
      this.userService.currentUser.set(null);
      this.router.navigate(['/login']);
    });
  }

  // ✅ NOVO: Método para alternar roles do usuário (modo debug)
  switchUserType = () => {
    const currentUser = this.currentUser();
    if (currentUser) {
      let newRoles: UserRole[];
      
      if (UserUtils.hasMultipleRoles(currentUser)) {
        // Se tem múltiplas roles, remover uma
        if (UserUtils.isClient(currentUser)) {
          newRoles = ['supplier'];
        } else {
          newRoles = ['client'];
        }
      } else if (UserUtils.isClient(currentUser)) {
        // Se é só cliente, adicionar supplier
        newRoles = ['client', 'supplier'];
      } else if (UserUtils.isSupplier(currentUser)) {
        // Se é só supplier, adicionar client
        newRoles = ['client', 'supplier'];
      } else {
        // Fallback - definir como cliente
        newRoles = ['client'];
      }
      
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