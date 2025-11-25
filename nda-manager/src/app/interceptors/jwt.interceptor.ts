import { HttpInterceptorFn } from '@angular/common/http';
import { inject } from '@angular/core';
import { UserService } from '../services/user.service';

/**
 * JWT Interceptor
 * 
 * Automatically adds the JWT access token to all HTTP requests
 * (except public endpoints like login, register, etc.)
 */
export const jwtInterceptor: HttpInterceptorFn = (req, next) => {
    const userService = inject(UserService);
    const accessToken = userService.getAccessToken();

    // List of public endpoints that don't require authentication
    const publicEndpoints = [
        '/api/users/register',
        '/api/users/login',
        '/api/users/auto-login'
    ];

    // Check if this is a public endpoint
    const isPublicEndpoint = publicEndpoints.some(endpoint => req.url.includes(endpoint));

    // Add Authorization header if token exists and endpoint is not public
    if (accessToken && !isPublicEndpoint) {
        req = req.clone({
            setHeaders: {
                Authorization: `Bearer ${accessToken}`
            }
        });
    }

    return next(req);
};
