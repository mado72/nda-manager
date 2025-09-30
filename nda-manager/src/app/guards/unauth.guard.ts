import { inject } from '@angular/core';
import { CanActivateFn, Router } from '@angular/router';
import { UserService } from '../services/user.service';

export const unauthGuard: CanActivateFn = (route, state) => {
  const userService = inject(UserService);
  const router = inject(Router);
  
  const currentUser = userService.currentUser();
  
  // Se o usuário não está logado, pode acessar as páginas de login/registro
  if (!currentUser) {
    return true;
  }
  
  // Se está logado, redireciona para a página principal
  router.navigate(['/']);
  return false;
};