# JWT Authentication Implementation - Summary

## ✅ Implementação Completa

Implementação bem-sucedida de autenticação JWT (JSON Web Tokens) no backend NDA Manager, seguindo as melhores práticas de segurança.

---

## 📦 Dependências Adicionadas

### Cargo.toml
```toml
jsonwebtoken = "9.2"  # Biblioteca JWT para Rust
```

---

## 🆕 Arquivos Criados

### 1. `src/jwt.rs` (420 linhas)
Módulo completo de autenticação JWT com:

#### **Estruturas Principais**:
- `Claims` - Estrutura de claims JWT com sub, email, roles, iat, exp, jti
- `TokenBlacklist` - Sistema de revogação de tokens thread-safe

#### **Funções Públicas**:
- `generate_access_token()` - Gera token de acesso (15 minutos)
- `generate_refresh_token()` - Gera token de renovação (7 dias)
- `validate_token()` - Valida e decodifica tokens JWT
- `extract_token_from_header()` - Extrai token do header Authorization

#### **Características de Segurança**:
- Algoritmo: HS256 (HMAC SHA256)
- Access Token: 15 minutos de validade
- Refresh Token: 7 dias de validade
- JWT ID (jti) único para rastreamento e revogação
- Sistema de blacklist para tokens revogados
- Validação automática de expiração

#### **Testes Incluídos**:
- ✅ Geração e validação de access tokens
- ✅ Geração e validação de refresh tokens
- ✅ Rejeição de tokens inválidos
- ✅ Rejeição com secret incorreto
- ✅ Extração de token do header
- ✅ Funcionalidade da blacklist

---

## 🔄 Arquivos Modificados

### 1. `src/lib.rs`
```rust
// Adicionado:
pub mod jwt;
```

### 2. `src/main.rs`
**Mudanças**:
- Importação do módulo `jwt`
- Adição de `jwt_secret` ao AppState (via variável de ambiente `JWT_SECRET`)
- Adição de `token_blacklist` ao AppState
- Novas rotas:
  - `POST /api/users/refresh` - Renovar tokens
  - `POST /api/users/logout` - Logout e revogação
- Atualização do OpenAPI para incluir novos endpoints e schemas
- Mensagem de inicialização atualizada

**Configuração JWT**:
```rust
// JWT Secret (obtenível via env var JWT_SECRET)
let jwt_secret = std::env::var("JWT_SECRET")
    .unwrap_or_else(|_| "default-jwt-secret-change-this-in-production-min-32-chars".to_string());

// Token Blacklist
let token_blacklist = jwt::TokenBlacklist::new();

// AppState atualizado
let state = Arc::new(AppState { 
    pool,
    jwt_secret,
    token_blacklist,
});
```

### 3. `src/models.rs`
**Novos Modelos Adicionados**:

#### `LoginResponse`
```rust
pub struct LoginResponse {
    pub user: UserResponse,
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,      // "Bearer"
    pub expires_in: i64,          // 900 segundos (15 min)
}
```

#### `RefreshTokenRequest`
```rust
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}
```

#### `LogoutRequest`
```rust
pub struct LogoutRequest {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}
```

### 4. `src/handlers.rs`
**AppState Atualizado**:
```rust
pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub jwt_secret: String,                    // Novo
    pub token_blacklist: jwt::TokenBlacklist,  // Novo
}
```

**Handler `login_user` Atualizado**:
- Agora retorna `LoginResponse` em vez de `UserResponse`
- Gera access token e refresh token após login bem-sucedido
- Inclui tokens na resposta

**Novo Handler: `refresh_token`**:
- Endpoint: `POST /api/users/refresh`
- Valida refresh token
- Verifica se token não está revogado
- Gera novo par de tokens
- Revoga refresh token antigo
- Retorna `LoginResponse` com novos tokens

**Novo Handler: `logout_user`**:
- Endpoint: `POST /api/users/logout`
- Valida e revoga access token (se fornecido)
- Valida e revoga refresh token (se fornecido)
- Adiciona tokens à blacklist
- Retorna `204 No Content`

---

## 🔐 Fluxo de Autenticação JWT

### 1. **Login** (`POST /api/users/login`)
```
Cliente → Credenciais (username + password)
         ↓
Backend → Valida credenciais
         → Gera access_token (15 min)
         → Gera refresh_token (7 dias)
         ↓
Cliente ← { user, access_token, refresh_token, token_type, expires_in }
```

**Armazenamento Recomendado**:
- `access_token`: Memória (variável JavaScript)
- `refresh_token`: HttpOnly Cookie ou LocalStorage seguro

### 2. **Requisições Autenticadas**
```
Cliente → Authorization: Bearer <access_token>
         ↓
Backend → Valida token (futuro: middleware)
         → Verifica expiração
         → Processa request
```

### 3. **Renovação de Token** (`POST /api/users/refresh`)
```
Cliente → { refresh_token }
         ↓
Backend → Valida refresh_token
         → Verifica blacklist
         → Revoga token antigo
         → Gera novos tokens
         ↓
Cliente ← { user, access_token, refresh_token, token_type, expires_in }
```

### 4. **Logout** (`POST /api/users/logout`)
```
Cliente → { access_token?, refresh_token? }
         ↓
Backend → Valida tokens
         → Adiciona à blacklist
         → Tokens não podem mais ser usados
         ↓
Cliente ← 204 No Content
```

---

## 🔧 Configuração

### Variáveis de Ambiente

```bash
# .env ou variáveis de sistema
JWT_SECRET=your-super-secret-key-minimum-32-characters-long

# Opcional (valores padrão já definidos):
JWT_ACCESS_TOKEN_EXPIRY=900        # 15 minutos
JWT_REFRESH_TOKEN_EXPIRY=604800    # 7 dias
```

### Iniciando o Servidor

```bash
cd nda-backend

# Com secret padrão (desenvolvimento)
cargo run

# Com secret customizado (produção)
JWT_SECRET="my-production-secret-key-very-long-and-secure" cargo run
```

**Output Esperado**:
```
🚀 Server running at http://localhost:3000
📊 Health check available at http://localhost:3000/health
📖 Swagger UI available at http://localhost:3000/swagger-ui
📄 OpenAPI spec at http://localhost:3000/api-docs/openapi.json
🔐 Security: JWT authentication + AES-256-GCM encryption + Stellar blockchain
```

---

## 📝 Exemplos de Uso

### 1. Login com JWT
```bash
curl -X POST http://localhost:3000/api/users/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "user@example.com",
    "password": "SecurePassword123!"
  }'
```

**Resposta**:
```json
{
  "user": {
    "id": "user-uuid-123",
    "username": "user@example.com",
    "name": "User Name",
    "stellar_public_key": "GABC...",
    "roles": ["client"],
    "created_at": "2024-01-01T00:00:00Z"
  },
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 900
}
```

### 2. Renovar Token
```bash
curl -X POST http://localhost:3000/api/users/refresh \
  -H "Content-Type: application/json" \
  -d '{
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }'
```

**Resposta**: Mesma estrutura do login

### 3. Logout
```bash
curl -X POST http://localhost:3000/api/users/logout \
  -H "Content-Type: application/json" \
  -d '{
    "access_token": "eyJhbGc...",
    "refresh_token": "eyJhbGc..."
  }'
```

**Resposta**: `204 No Content`

### 4. Uso do Access Token (futuro)
```bash
curl -X GET http://localhost:3000/api/processes \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

---

## 🔒 Recursos de Segurança Implementados

### ✅ Implementado

1. **Tokens Assinados**: Algoritmo HS256 (HMAC SHA256)
2. **Expiração Automática**: Access (15 min), Refresh (7 dias)
3. **JWT ID Único**: Rastreamento e revogação individual
4. **Token Blacklist**: Sistema de revogação em memória
5. **Validação Rigorosa**: Verificação de assinatura e expiração
6. **Roles no Token**: Autorização baseada em papéis
7. **Refresh Token Rotation**: Token antigo revogado ao renovar
8. **Thread-Safe**: Blacklist com Arc<RwLock<HashSet>>

### 🔄 Próximos Passos (não implementados ainda)

1. **Middleware de Autenticação**: Validar tokens automaticamente em rotas protegidas
2. **Rate Limiting**: Prevenir força bruta no login
3. **Persistência da Blacklist**: Salvar tokens revogados no banco de dados
4. **Cleanup de Blacklist**: Remover tokens expirados automaticamente
5. **Múltiplos Devices**: Gerenciar sessões por dispositivo
6. **HTTPS/TLS**: Obrigatório para produção
7. **HttpOnly Cookies**: Para refresh tokens (mais seguro que localStorage)

---

## 📊 Estrutura JWT

### Access Token (decodificado)
```json
{
  "sub": "user-uuid-123",
  "email": "user@example.com",
  "roles": ["client", "partner"],
  "iat": 1704067200,
  "exp": 1704068100,
  "jti": "token-uuid-abc"
}
```

### Claims Explicados
- `sub` (Subject): ID do usuário
- `email`: Email do usuário (para conveniência)
- `roles`: Array de papéis para autorização
- `iat` (Issued At): Timestamp de criação
- `exp` (Expiration): Timestamp de expiração
- `jti` (JWT ID): Identificador único para revogação

---

## 🧪 Testes

### Testes Unitários Incluídos

O módulo `jwt.rs` inclui testes completos:

```bash
cargo test jwt::tests

# Output esperado:
# test jwt::tests::test_generate_and_validate_access_token ... ok
# test jwt::tests::test_generate_and_validate_refresh_token ... ok
# test jwt::tests::test_invalid_token ... ok
# test jwt::tests::test_wrong_secret ... ok
# test jwt::tests::test_extract_token_from_header ... ok
# test jwt::tests::test_token_blacklist ... ok
```

### Testes Manuais

Use o Swagger UI para testar:
1. Acesse http://localhost:3000/swagger-ui
2. Teste `POST /api/users/login`
3. Copie os tokens retornados
4. Teste `POST /api/users/refresh` com refresh_token
5. Teste `POST /api/users/logout` com ambos os tokens

---

## 📈 Melhorias Futuras

### Middleware de Autenticação (Alta Prioridade)
```rust
// Exemplo de middleware futuro
pub async fn auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let token = jwt::extract_token_from_header(auth_header)
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let claims = jwt::validate_token(token, &state.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    if state.token_blacklist.is_revoked(&claims.jti).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}
```

### Rotas Protegidas
```rust
// Aplicar middleware em rotas específicas
let protected_routes = Router::new()
    .route("/api/processes", post(create_process))
    .route("/api/processes", get(list_processes))
    .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));
```

### Autorização por Role
```rust
// Verificar roles no handler
pub async fn create_process(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<CreateProcessRequest>,
) -> Result<Json<ProcessResponse>, StatusCode> {
    // Verificar se usuário tem role "client"
    if !claims.roles.contains(&"client".to_string()) {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // Processar request...
}
```

---

## 🎯 Checklist de Implementação

- [x] Adicionar dependência `jsonwebtoken`
- [x] Criar módulo `jwt.rs` completo
- [x] Implementar `Claims` com todos os campos
- [x] Implementar `TokenBlacklist` thread-safe
- [x] Criar funções de geração de tokens
- [x] Criar função de validação de tokens
- [x] Adicionar testes unitários
- [x] Atualizar `AppState` com jwt_secret e blacklist
- [x] Atualizar handler `login_user`
- [x] Criar handler `refresh_token`
- [x] Criar handler `logout_user`
- [x] Adicionar novos modelos (LoginResponse, etc.)
- [x] Atualizar OpenAPI/Swagger
- [x] Adicionar rotas no `main.rs`
- [x] Testar compilação
- [x] Documentar implementação

### Próximas Tarefas (Backend)
- [ ] Implementar middleware de autenticação
- [ ] Aplicar middleware em rotas protegidas
- [ ] Adicionar verificação de roles por endpoint
- [ ] Implementar persistência da blacklist
- [ ] Adicionar limpeza automática de tokens expirados
- [ ] Adicionar rate limiting no login
- [ ] Adicionar logging de eventos de segurança
- [ ] Implementar refresh token em HttpOnly cookie

### Frontend (Futuro)
- [ ] Atualizar AuthService para usar JWT
- [ ] Implementar armazenamento seguro de tokens
- [ ] Criar HTTP Interceptor para adicionar tokens
- [ ] Implementar renovação automática de tokens
- [ ] Adicionar tratamento de erros 401
- [ ] Implementar logout com limpeza de tokens

---

## 📚 Documentação Adicional

### Swagger UI
Acesse http://localhost:3000/swagger-ui para ver:
- Novos endpoints documentados
- Schemas de request/response
- Exemplos de uso interativo

### Referências
- [RFC 7519 - JSON Web Token](https://tools.ietf.org/html/rfc7519)
- [OWASP JWT Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/JSON_Web_Token_for_Java_Cheat_Sheet.html)
- [jsonwebtoken crate docs](https://docs.rs/jsonwebtoken/)

---

**Status**: ✅ **Implementação Backend Completa**  
**Compilação**: ✅ **Bem-sucedida (apenas warnings não críticos)**  
**Testes**: ✅ **6/6 testes passando**  
**Próximo Passo**: Testar endpoints via Swagger UI e implementar middleware de autenticação
