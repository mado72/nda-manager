# JWT Authentication System

## Overview

The NDA Backend implements a secure JWT (JSON Web Token) authentication system with access and refresh tokens, role-based authorization, and token blacklisting for immediate revocation.

## Token Types

### Access Token
- **Lifetime**: 15 minutes
- **Purpose**: Authenticate API requests
- **Algorithm**: HS256 (HMAC SHA256)
- **Claims**:
  - `sub` (subject): User ID (UUID)
  - `username`: User's username/email
  - `roles`: Array of user roles (`["client"]`, `["partner"]`, `["admin"]`)
  - `exp` (expiration): Unix timestamp
  - `iat` (issued at): Unix timestamp

### Refresh Token
- **Lifetime**: 7 days
- **Purpose**: Obtain new access tokens without re-authentication
- **Algorithm**: HS256 (HMAC SHA256)
- **Security**: Single-use pattern (blacklisted after refresh)

## Authentication Flow

```
┌──────────────┐
│    Login     │
│  (username/  │  → POST /api/users/login
│   password)  │
└──────┬───────┘
       │
       ├─────────────────────────────────────┐
       │                                     │
       ▼                                     ▼
┌──────────────┐                    ┌──────────────┐
│ access_token │                    │refresh_token │
│  (15 min)    │                    │   (7 days)   │
└──────┬───────┘                    └──────┬───────┘
       │                                    │
       ▼                                    │
┌──────────────┐                           │
│  API Calls   │                           │
│ (with Bearer │                           │
│    token)    │                           │
└──────┬───────┘                           │
       │                                    │
       ▼                                    │
┌──────────────┐                           │
│   Expired?   │───(Yes)──────────────────►│
└──────┬───────┘                           │
       │                                    ▼
      (No)                          ┌──────────────┐
       │                            │   Refresh    │
       ▼                            │ Access Token │
┌──────────────┐                   │ POST /api/   │
│   Process    │                   │ users/refresh│
│   Request    │◄─────────────────┴──────────────┘
└──────────────┘
```

## API Endpoints

### 1. Login

**Request:**
```http
POST /api/users/login
Content-Type: application/json

{
    "username": "user@company.com",
    "password": "password123"
}
```

**Response:**
```json
{
    "user": {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "username": "user@company.com",
        "stellar_public_key": "GXXXXXXXXXXXXX",
        "roles": ["client"]
    },
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

### 2. Refresh Token

**Request:**
```http
POST /api/users/refresh
Content-Type: application/json

{
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Response:**
```json
{
    "user": {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "username": "user@company.com",
        "stellar_public_key": "GXXXXXXXXXXXXX",
        "roles": ["client"]
    },
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

### 3. Logout

**Request:**
```http
POST /api/users/logout
Content-Type: application/json
Authorization: Bearer <access_token>

{
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Response:**
```json
{
    "message": "Successfully logged out"
}
```

## Using JWT in API Calls

### Protected Endpoints

All endpoints marked with 🔒 require JWT authentication:

| Endpoint | Method | Required Role | Description |
|----------|--------|---------------|-------------|
| `/api/processes` | POST | `client` | Create new NDA process |
| `/api/processes` | GET | `client`, `partner`, `admin` | List processes |
| `/api/users/logout` | POST | Any authenticated | Logout |

### Example: Create Process

```bash
# 1. Login and save token
LOGIN_RESPONSE=$(curl -s -X POST http://localhost:3000/api/users/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "client@company.com",
    "password": "password123"
  }')

# 2. Extract access token
ACCESS_TOKEN=$(echo $LOGIN_RESPONSE | jq -r '.access_token')

# 3. Make authenticated request
curl -X POST http://localhost:3000/api/processes \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -d '{
    "client_username": "client@company.com",
    "title": "NDA - Confidential Project",
    "confidential_content": "Ultra-secret content..."
  }'
```

### Example: Handle Token Expiration

```bash
# Function to get valid access token
get_access_token() {
    if [ -z "$ACCESS_TOKEN" ] || token_expired "$ACCESS_TOKEN"; then
        if [ -n "$REFRESH_TOKEN" ]; then
            # Refresh the token
            REFRESH_RESPONSE=$(curl -s -X POST http://localhost:3000/api/users/refresh \
              -H "Content-Type: application/json" \
              -d "{\"refresh_token\":\"$REFRESH_TOKEN\"}")
            
            ACCESS_TOKEN=$(echo $REFRESH_RESPONSE | jq -r '.access_token')
            REFRESH_TOKEN=$(echo $REFRESH_RESPONSE | jq -r '.refresh_token')
        else
            # Login again
            login_user
        fi
    fi
    echo $ACCESS_TOKEN
}

# Use in API calls
TOKEN=$(get_access_token)
curl -H "Authorization: Bearer $TOKEN" http://localhost:3000/api/processes
```

## Error Handling

### Error Responses

| Status Code | Error Type | Description | Solution |
|------------|-----------|-------------|----------|
| `401 Unauthorized` | Missing Token | Authorization header not provided | Add `Authorization: Bearer <token>` header |
| `401 Unauthorized` | Invalid Token | Token signature invalid or malformed | Login again to get new token |
| `401 Unauthorized` | Expired Token | Access token expired (>15 min) | Use refresh token to get new access token |
| `401 Unauthorized` | Revoked Token | Token in blacklist (after logout) | Login again |
| `403 Forbidden` | Insufficient Permissions | User role doesn't match requirement | Contact admin for role assignment |

### Example Error Response

```json
{
    "error": "Unauthorized",
    "message": "Invalid or expired token"
}
```

## Security Features

### 1. Token Signing
- **Algorithm**: HS256 (HMAC SHA256)
- **Secret Key**: Configured via `JWT_SECRET` environment variable
- **Minimum Length**: 32 characters recommended
- **Production**: Use strong, randomly generated secret

### 2. Token Validation
- ✅ Signature verification
- ✅ Expiration check
- ✅ Blacklist verification
- ✅ Claims validation

### 3. Token Blacklist
- **Purpose**: Immediate token revocation on logout
- **Implementation**: In-memory `HashSet` with `Arc<RwLock>`
- **Thread-Safe**: Concurrent access from multiple requests
- **Persistence**: Tokens removed after expiration
- **Operations**:
  - `revoke(token)`: Add token to blacklist
  - `is_revoked(token)`: Check if token is blacklisted
  - `clear()`: Clear all blacklisted tokens

### 4. Role-Based Authorization

```rust
// Example: Endpoint requiring "client" role
pub async fn create_process(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateProcessRequest>,
) -> Result<ResponseJson<ProcessResponse>, StatusCode> {
    // Validate JWT and extract claims
    let claims = jwt::validate_auth_header(
        headers.get("authorization").and_then(|h| h.to_str().ok()),
        &state.jwt_secret,
        &state.token_blacklist
    ).await.map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Check role
    if !claims.roles.contains(&"client".to_string()) {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // Process request...
}
```

## Configuration

### Environment Variables

```bash
# JWT secret key (REQUIRED for production)
JWT_SECRET=your-super-secure-secret-min-32-chars-long

# Database
DATABASE_URL=sqlite:./stellar_mvp.db

# Logging
RUST_LOG=debug
```

### Token Lifetimes

Configured in `src/jwt.rs`:

```rust
// Access token: 15 minutes
const ACCESS_TOKEN_EXPIRATION: i64 = 15 * 60;

// Refresh token: 7 days
const REFRESH_TOKEN_EXPIRATION: i64 = 7 * 24 * 60 * 60;
```

To customize:
1. Edit constants in `src/jwt.rs`
2. Rebuild: `cargo build`
3. Restart server

## Testing

### Unit Tests

```bash
# Run all JWT tests
cargo test jwt::tests

# Run specific test
cargo test jwt::tests::test_generate_access_token
```

### Manual Testing

```bash
# 1. Test login
curl -X POST http://localhost:3000/api/users/login \
  -H "Content-Type: application/json" \
  -d '{"username":"test@test.com","password":"test123"}'

# 2. Test protected endpoint
curl -X POST http://localhost:3000/api/processes \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token>" \
  -d '{...}'

# 3. Test token refresh
curl -X POST http://localhost:3000/api/users/refresh \
  -H "Content-Type: application/json" \
  -d '{"refresh_token":"<refresh_token>"}'

# 4. Test logout
curl -X POST http://localhost:3000/api/users/logout \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token>" \
  -d '{"token":"<token>"}'
```

## Best Practices

### For API Consumers

1. **Store Tokens Securely**
   - Frontend: Use `httpOnly` cookies or secure storage
   - Mobile: Use platform-specific secure storage (Keychain, KeyStore)
   - Never store in localStorage for sensitive apps

2. **Handle Token Expiration**
   - Implement automatic refresh logic
   - Retry failed requests with new token
   - Fallback to login if refresh fails

3. **Logout Properly**
   - Always call logout endpoint
   - Clear stored tokens
   - Redirect to login page

4. **Token Transmission**
   - Always use HTTPS in production
   - Include token in `Authorization: Bearer <token>` header
   - Never send token in URL query parameters

### For Backend Developers

1. **Secret Management**
   - Use strong, randomly generated secrets (min 32 chars)
   - Rotate secrets periodically
   - Never commit secrets to version control
   - Use environment variables or secret managers

2. **Token Validation**
   - Always validate signature
   - Check expiration
   - Verify blacklist status
   - Validate required claims

3. **Role-Based Access**
   - Check user roles for each protected endpoint
   - Use principle of least privilege
   - Document required roles in API docs

4. **Monitoring**
   - Log authentication attempts
   - Track token usage patterns
   - Monitor blacklist size
   - Alert on suspicious activity

## Troubleshooting

### Common Issues

1. **401 Unauthorized on Protected Endpoints**
   - Check if token is included in header
   - Verify token format: `Authorization: Bearer <token>`
   - Ensure token hasn't expired
   - Check if JWT_SECRET matches between token generation and validation

2. **403 Forbidden**
   - Verify user has required role
   - Check if user is accessing their own resources
   - Review endpoint authorization requirements

3. **Token Not Refreshing**
   - Ensure refresh token hasn't expired (7 days)
   - Check if refresh token was blacklisted
   - Verify refresh token format

4. **High Memory Usage from Blacklist**
   - Consider implementing periodic cleanup
   - Remove expired tokens from blacklist
   - For high-traffic apps, use Redis for blacklist

## Implementation Details

### JWT Module Structure

```
src/jwt.rs
├── Claims struct            # JWT token claims
├── TokenBlacklist struct    # Thread-safe blacklist
├── generate_access_token()  # Create 15-min access token
├── generate_refresh_token() # Create 7-day refresh token
├── validate_token()         # Verify token signature & expiration
├── extract_token_from_header() # Parse Authorization header
├── validate_auth_header()   # Complete authentication validation
└── tests module             # Unit tests
```

### Integration Points

1. **AppState** (`src/handlers.rs`)
   ```rust
   pub struct AppState {
       pub pool: SqlitePool,
       pub jwt_secret: String,
       pub token_blacklist: jwt::TokenBlacklist,
   }
   ```

2. **Protected Handlers** (Add to any handler requiring auth)
   ```rust
   pub async fn protected_handler(
       headers: HeaderMap,
       State(state): State<Arc<AppState>>,
       // ... other parameters
   ) -> Result<ResponseJson<T>, StatusCode> {
       let claims = jwt::validate_auth_header(
           headers.get("authorization").and_then(|h| h.to_str().ok()),
           &state.jwt_secret,
           &state.token_blacklist
       ).await.map_err(|_| StatusCode::UNAUTHORIZED)?;
       
       // Use claims.sub, claims.username, claims.roles
       // ...
   }
   ```

## Future Enhancements

- [ ] Implement token rotation for refresh tokens
- [ ] Add Redis support for distributed blacklist
- [ ] Implement rate limiting per token
- [ ] Add token fingerprinting for additional security
- [ ] Support multiple signing algorithms (RS256, ES256)
- [ ] Add token introspection endpoint
- [ ] Implement OAuth2/OpenID Connect support
- [ ] Add two-factor authentication (2FA)
- [ ] Implement API key authentication for service accounts
- [ ] Add webhook notifications for suspicious activity

## References

- [RFC 7519: JSON Web Token (JWT)](https://tools.ietf.org/html/rfc7519)
- [OWASP JWT Security Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/JSON_Web_Token_for_Java_Cheat_Sheet.html)
- [jsonwebtoken Rust crate](https://docs.rs/jsonwebtoken/)
- [Axum Framework](https://docs.rs/axum/)
