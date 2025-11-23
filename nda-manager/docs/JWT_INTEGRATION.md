# Integração JWT - Front-End Angular

## 📋 Resumo das Mudanças

Este documento descreve as mudanças implementadas no front-end Angular para integração com o sistema de autenticação JWT do back-end.

## ✅ Alterações Implementadas

### 1. **Models** (`user.model.ts`)

Adicionadas interfaces para suportar autenticação JWT:

```typescript
export interface LoginResponse {
    user: UserResponse;
    access_token: string;
    refresh_token: string;
}

export interface RefreshTokenRequest {
    refresh_token: string;
}

export interface LogoutRequest {
    token: string;
}
```

### 2. **JWT Interceptor** (`jwt.interceptor.ts`)

Novo interceptor HTTP que adiciona automaticamente o token JWT em todas as requisições:

- ✅ Adiciona header `Authorization: Bearer <token>` em requisições autenticadas
- ✅ Exclui endpoints públicos (register, login, auto-login)
- ✅ Funciona automaticamente - sem necessidade de código adicional nos serviços

**Localização:** `src/app/interceptors/jwt.interceptor.ts`

### 3. **Auth Error Interceptor** (`auth-error.interceptor.ts`)

Novo interceptor que trata erros de autenticação:

- ✅ Detecta erros 401 Unauthorized
- ✅ Tenta automaticamente fazer refresh do token
- ✅ Refaz a requisição original com novo token
- ✅ Se refresh falhar, faz logout e redireciona para login

**Localização:** `src/app/interceptors/auth-error.interceptor.ts`

### 4. **UserService** (`user.service.ts`)

Atualizado para gerenciar tokens JWT:

#### Novos Métodos:
```typescript
// Gerenciamento de tokens
getAccessToken(): string | null
getRefreshToken(): string | null
private setAccessToken(token: string): void
private setRefreshToken(token: string): void
private clearTokens(): void
```

#### Métodos Atualizados:

**`login()`**
- Retorna agora `Observable<LoginResponse>` (inclui user + tokens)
- Salva automaticamente access_token e refresh_token no localStorage
- Mantém funcionalidade de "remember me"

**`logout()`**
- Retorna `Observable<any>` em vez de void
- Chama endpoint `/api/users/logout` no backend
- Adiciona token à blacklist
- Limpa sessão local (tokens + dados do usuário)

**`isLoggedIn()`**
- Verifica se usuário existe E se há token válido

#### Novo Método:

**`refreshToken()`**
```typescript
refreshToken(): Observable<LoginResponse>
```
- Usa refresh_token para obter novo access_token
- Atualiza tokens automaticamente
- Usado pelo auth-error.interceptor

### 5. **App Config** (`app.config.ts`)

Registrados os interceptors HTTP:

```typescript
provideHttpClient(
  withInterceptors([jwtInterceptor, authErrorInterceptor])
)
```

### 6. **Login Component** (`login-user.component.ts`)

Atualizado para trabalhar com nova estrutura de resposta:

```typescript
// Antes: Observable<UserResponse>
// Agora: Observable<LoginResponse>

next: (response) => {
    if (response && response.user) {
        // Tokens salvos automaticamente pelo service
        this.successMessage = 'Login successful!';
        // ...
    }
}
```

### 7. **Menu Component** (`menu.component.ts`)

Método `logout()` atualizado para usar Observable:

```typescript
logout = () => {
    this.userService.logout().subscribe({
        next: () => {
            this.router.navigate(['/login']);
        },
        error: () => {
            // Redireciona mesmo com erro
            this.router.navigate(['/login']);
        }
    });
}
```

## 🔄 Fluxo de Autenticação

### Login
```
1. Usuário faz login → POST /api/users/login
2. Backend retorna { user, access_token, refresh_token }
3. UserService salva tokens no localStorage
4. JWT Interceptor adiciona token automaticamente nas próximas requisições
```

### Requisições Autenticadas
```
1. Angular faz requisição HTTP
2. JWT Interceptor adiciona header: Authorization: Bearer <access_token>
3. Backend valida token
4. Resposta retorna normalmente
```

### Token Expirado
```
1. Requisição retorna 401 Unauthorized
2. Auth Error Interceptor detecta erro
3. Tenta refresh automático: POST /api/users/refresh
4. Obtém novos tokens
5. Refaz requisição original com novo token
6. Se refresh falhar → Logout automático
```

### Logout
```
1. Usuário clica em logout
2. Frontend chama POST /api/users/logout
3. Backend adiciona token à blacklist
4. Frontend limpa tokens e dados do usuário
5. Redirecionamento para /login
```

## 📁 Estrutura de Arquivos

```
src/app/
├── interceptors/               # NOVO
│   ├── jwt.interceptor.ts
│   └── auth-error.interceptor.ts
├── models/
│   └── user.model.ts          # ATUALIZADO
├── services/
│   └── user.service.ts        # ATUALIZADO
├── components/
│   ├── login-user/
│   │   └── login-user.component.ts  # ATUALIZADO
│   └── menu/
│       └── menu.component.ts        # ATUALIZADO
└── app.config.ts              # ATUALIZADO
```

## 🔒 Segurança

### Armazenamento de Tokens
- **Access Token**: Armazenado em `localStorage` (chave: `access_token`)
- **Refresh Token**: Armazenado em `localStorage` (chave: `refresh_token`)
- **Lifetime**: 
  - Access Token: 15 minutos
  - Refresh Token: 7 dias

### Proteção Automática
- ✅ Todos os endpoints protegidos recebem token automaticamente
- ✅ Tokens expirados são renovados automaticamente
- ✅ Falhas de autenticação resultam em logout automático
- ✅ Tokens revogados (após logout) não podem ser reutilizados

## 🧪 Como Testar

### 1. Login
```bash
# Inicie o backend
cd nda-backend
cargo run

# Inicie o frontend
cd nda-manager
ng serve

# Acesse http://localhost:4200
# Faça login com credenciais válidas
```

### 2. Verificar Tokens
```javascript
// No console do navegador (F12)
console.log('Access Token:', localStorage.getItem('access_token'));
console.log('Refresh Token:', localStorage.getItem('refresh_token'));
```

### 3. Testar Requisições Autenticadas
```typescript
// Abra Network tab no DevTools
// Faça qualquer operação (criar contrato, listar processos)
// Verifique header "Authorization: Bearer ..." nas requisições
```

### 4. Testar Refresh Automático
```javascript
// No console do navegador
// Limpe o access_token (simulando expiração)
localStorage.setItem('access_token', 'token-invalido');

// Faça uma requisição que exige autenticação
// O interceptor tentará refresh automaticamente
```

### 5. Testar Logout
```typescript
// Clique no botão de logout
// Verifique que:
// 1. Tokens são removidos do localStorage
// 2. Requisição POST foi enviada para /api/users/logout
// 3. Redirecionamento para /login ocorreu
```

## 🚨 Problemas Comuns

### Token não está sendo enviado
- Verifique se o endpoint não está na lista `publicEndpoints` do JWT interceptor
- Confirme que `getAccessToken()` retorna um valor válido

### Refresh infinito
- Verifique se o endpoint `/api/users/refresh` está excluído no auth-error.interceptor
- Confirme que refresh_token é válido e não expirou

### Logout não funciona
- Verifique se o método `logout()` retorna Observable
- Confirme que está usando `.subscribe()` no componente

### CORS errors
- Backend deve ter CORS configurado para aceitar header Authorization
- Verifique configuração no backend (já está OK no main.rs)

## 📝 Próximos Passos

### Melhorias Sugeridas
- [ ] Implementar exibição de mensagens de erro JWT para o usuário
- [ ] Adicionar loading indicator durante refresh de token
- [ ] Implementar timeout de sessão por inatividade
- [ ] Adicionar opção "Lembrar-me" para refresh token persistente
- [ ] Implementar storage mais seguro (httpOnly cookies)

### Funcionalidades Futuras
- [ ] Two-factor authentication (2FA)
- [ ] OAuth2 integration (Google, Microsoft)
- [ ] Session management (listar sessões ativas)
- [ ] Device fingerprinting para segurança adicional

## ✅ Checklist de Integração

- [x] Models atualizados com interfaces JWT
- [x] JWT Interceptor criado e configurado
- [x] Auth Error Interceptor criado e configurado
- [x] UserService atualizado com gerenciamento de tokens
- [x] Interceptors registrados no app.config
- [x] Login component atualizado
- [x] Menu component logout atualizado
- [x] Compilação bem-sucedida
- [x] Nenhum erro TypeScript

## 🎯 Conclusão

O front-end Angular está agora totalmente integrado com o sistema de autenticação JWT do back-end:

✅ **Autenticação automática** via interceptors  
✅ **Refresh automático** de tokens expirados  
✅ **Logout seguro** com blacklist  
✅ **Tratamento de erros** robusto  
✅ **Código limpo** e manutenível  

**Status:** Pronto para uso em desenvolvimento! 🚀

---

**Última atualização:** 23 de novembro de 2025  
**Versão do Angular:** 20  
**Backend:** Rust + Axum + JWT
