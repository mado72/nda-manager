import { Routes } from '@angular/router';
import { authGuard } from './guards/auth.guard';
import { unauthGuard } from './guards/unauth.guard';

export const routes: Routes = [
    {
        path: '',
        redirectTo: 'home',
        pathMatch: 'full'
    },
    {
        path: 'home',
        loadComponent: () => import('./components/home/home.component').then(m => m.HomeComponent),
        canActivate: [authGuard]
    },
    {
        path: 'contracts',
        loadComponent: () => import('./components/contracts-master-detail/contracts-master-detail.component').then(m => m.ContractsMasterDetailComponent),
        canActivate: [authGuard]
    },
    {
        path: 'contracts/add',
        loadComponent: () => import('./components/contracts-master-detail/contracts-master-detail.component').then(m => m.ContractsMasterDetailComponent),
        canActivate: [authGuard]
    },
    {
        path: 'contracts/share',
        loadComponent: () => import('./components/share-contract/share-contract.component').then(m => m.ShareContractComponent),
        canActivate: [authGuard]
    },
    {
        path: 'contracts/edit/:contractId',
        loadComponent: () => import('./components/contracts-master-detail/contracts-master-detail.component').then(m => m.ContractsMasterDetailComponent),
        canActivate: [authGuard]
    },
    {
        path: 'login',
        loadComponent: () => import('./components/login-user/login-user.component').then(m => m.LoginUserComponent),
        canActivate: [unauthGuard]
    },
    {
        path: 'register',
        loadComponent: () => import('./components/register-user/register-user.component').then(m => m.RegisterUserComponent),
        canActivate: [unauthGuard]
    }
];
