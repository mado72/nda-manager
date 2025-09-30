import { inject } from '@angular/core';
import { CanActivateFn, Router } from '@angular/router';
import { map, catchError, of } from 'rxjs';
import { UserService } from '../services/user.service';

export const authGuard: CanActivateFn = (route, state) => {
  const userService = inject(UserService);
  const router = inject(Router);
  
  const currentUser = userService.currentUser();
  
  // Se o usuário está logado, pode acessar a rota
  if (currentUser) {
    return true;
  }
  
  // Tenta fazer auto-login antes de redirecionar
  return userService.tryAutoLogin().pipe(
    map(autoLoginResponse => {
      if (autoLoginResponse) {
        // Auto-login bem-sucedido, permite acesso
        return true;
      }
      // Auto-login falhou, redireciona para login
      router.navigate(['/login']);
      return false;
    }),
    catchError(() => {
      // Erro no auto-login, redireciona para login
      router.navigate(['/login']);
      return of(false);
    })
  );
};