import { inject } from '@angular/core';
import { CanActivateFn, Router } from '@angular/router';
import { UserService } from '../services/user.service';

export const authGuard: CanActivateFn = (route, state) => {
  const userService = inject(UserService);
  const router = inject(Router);
  
  const currentUser = userService.currentUser();
  
  // Se o usuário está logado, pode acessar a rota
  if (currentUser) {
    return true;
  }
  
  // Se não está logado, redireciona para login
  router.navigate(['/login']);
  return false;
};