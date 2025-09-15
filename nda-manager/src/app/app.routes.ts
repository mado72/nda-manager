import { Routes } from '@angular/router';

export const routes: Routes = [
    {
        path: '',
        redirectTo: 'login',
        pathMatch: 'full'
    },
    {
        path: 'login',
        loadComponent: () => import('./components/login-user/login-user.component').then(m => m.LoginUserComponent)
    },
    {
        path: 'register',
        loadComponent: () => import('./components/register-user/register-user.component').then(m => m.RegisterUserComponent)
    },
    {
        path: 'register-contract',
        loadComponent: () => import('./components/register-contract/register-contract.component').then(m => m.RegisterContractComponent)
    },
    {
        path: 'list-contracts',
        loadComponent: () => import('./components/list-contracts/list-contracts.component').then(m => m.ListContractsComponent)
    },
    {
        path: 'contracts',
        loadComponent: () => import('./components/contracts-master-detail/contracts-master-detail.component').then(m => m.ContractsMasterDetailComponent)
    }
];
