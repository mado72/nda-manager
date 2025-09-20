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
import { User, UserType } from '../../models/user.model';
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


  ngOnInit() {
    this.checkScreenSize();
  }

  currentUser = computed(() => {
    const currentUser = this.userService.currentUser();
    if (currentUser) {
      this.contractService.getPermissions().subscribe(permissions => {
        this.canCreateContracts = permissions.canCreate;
        this.canShareContracts = permissions.canShare;
      });
    } else {
      // Fallback se não houver usuário logado
      this.canCreateContracts = false;
      this.canShareContracts = false;
    }

    console.log('👤 User loaded:', currentUser);
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
    return currentUser ? currentUser.user_type : 'Guest';
  });

  userTypeIcon = computed(() => {
    const currentUser = this.currentUser();
    if (!currentUser) {
      return 'person';
    }
    return currentUser.user_type === UserType.client ? 'business' : 'inventory';
  });

  displaySwitchUserTypeLabel = computed(() => {
    const currentUser = this.currentUser();
    if (!currentUser) {
      return 'Switch User Type';
    }
    return currentUser.user_type === UserType.client ? 'Switch to Supplier' : 'Switch to Client';
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

  // ✅ NOVO: Método para debug - trocar tipo de usuário (remover em produção)
  switchUserType = () => {
    const currentUser = this.currentUser();
    if (currentUser) {
      const newType = currentUser.user_type === UserType.client ? UserType.supplier : UserType.client;
      const newUser: User = {
        ...currentUser,
        user_type: newType
      };
      this.userService.currentUser.set(newUser);
      console.log('🔄 Switched user type to:', newType);
    }
  }
}