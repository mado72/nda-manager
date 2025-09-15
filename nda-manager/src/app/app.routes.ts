import { Routes } from '@angular/router';

export const routes: Routes = [
    {
        path: '',
        redirectTo: 'login',
        pathMatch: 'full'
    },
    {
        path: 'contracts',
        loadComponent: () => import('./components/contracts-master-detail/contracts-master-detail.component').then(m => m.ContractsMasterDetailComponent)
    },
    {
        path: 'contracts/add',
        loadComponent: () => import('./components/contracts-master-detail/contracts-master-detail.component').then(m => m.ContractsMasterDetailComponent)
    },
    {
        path: 'contracts/edit/:contractId',
        loadComponent: () => import('./components/contracts-master-detail/contracts-master-detail.component').then(m => m.ContractsMasterDetailComponent)
    },
    {
        path: 'login',
        loadComponent: () => import('./components/login-user/login-user.component').then(m => m.LoginUserComponent)
    },
    {
        path: 'register',
        loadComponent: () => import('./components/register-user/register-user.component').then(m => m.RegisterUserComponent)
    }
];
