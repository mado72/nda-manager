import { HttpInterceptorFn, HttpErrorResponse } from '@angular/common/http';
import { inject } from '@angular/core';
import { catchError, switchMap, throwError } from 'rxjs';
import { UserService } from '../services/user.service';
import { Router } from '@angular/router';

/**
 * Auth Error Interceptor
 * 
 * Intercepts HTTP errors and handles authentication failures:
 * - 401 Unauthorized: Attempts to refresh the access token
 * - If refresh fails: Logs out and redirects to login
 */
export const authErrorInterceptor: HttpInterceptorFn = (req, next) => {
    const userService = inject(UserService);
    const router = inject(Router);

    return next(req).pipe(
        catchError((error: HttpErrorResponse) => {
            // Handle 401 Unauthorized errors
            if (error.status === 401 && !req.url.includes('/api/users/refresh')) {
                // Try to refresh the token
                return userService.refreshToken().pipe(
                    switchMap(() => {
                        // Retry the original request with the new token
                        const accessToken = userService.getAccessToken();
                        const clonedReq = req.clone({
                            setHeaders: {
                                Authorization: `Bearer ${accessToken}`
                            }
                        });
                        return next(clonedReq);
                    }),
                    catchError(refreshError => {
                        // If refresh fails, logout and redirect to login
                        userService.logout().subscribe({
                            complete: () => {
                                router.navigate(['/login']);
                            }
                        });
                        return throwError(() => refreshError);
                    })
                );
            }

            // For other errors, just pass them through
            return throwError(() => error);
        })
    );
};
