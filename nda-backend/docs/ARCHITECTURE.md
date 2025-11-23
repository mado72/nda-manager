# NDA Backend - Architecture Documentation

## Table of Contents

1. [System Overview](#system-overview)
2. [Architecture Layers](#architecture-layers)
3. [Database Schema](#database-schema)
4. [Security Architecture](#security-architecture)
5. [Blockchain Integration](#blockchain-integration)
6. [Encryption System](#encryption-system)
7. [API Design Patterns](#api-design-patterns)
8. [Deployment Architecture](#deployment-architecture)

---

## System Overview

The NDA Backend is a Rust-based REST API server designed to manage blockchain-secured Non-Disclosure Agreement (NDA) contracts. The system provides:

- **User Management**: Role-based system supporting clients and partners
- **Process Management**: Encrypted storage of confidential NDA content
- **Blockchain Integration**: Immutable sharing records via Stellar network
- **Access Control**: Cryptographic verification of sharing permissions
- **Audit Trail**: Complete compliance logging for regulatory requirements

### Key Technologies

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Web Framework | Axum | High-performance async HTTP server |
| Database | SQLite + SQLx | Embedded database with type-safe queries |
| Encryption | AES-256-GCM (ring) | Content encryption with AEAD |
| Blockchain | Stellar Network | Immutable sharing records |
| Authentication | Bcrypt | Password hashing and verification |
| Runtime | Tokio | Async operations and concurrency |
| Logging | Tracing | Structured logging and diagnostics |

---

## Architecture Layers

### 1. HTTP Layer (`main.rs`)

**Responsibilities**:
- Route configuration and middleware
- CORS policy management
- Server lifecycle management
- Dependency injection via Arc

**Key Routes**:
```rust
Router::new()
    .route("/health", get(health_check))
    .route("/api/users/register", post(register_user))
    .route("/api/users/login", post(login_user))
    .route("/api/users/auto-login", post(auto_login_user))
    .route("/api/processes", post(create_process))
    .route("/api/processes", get(list_processes))
    .route("/api/processes/share", post(share_process))
    .route("/api/processes/access", post(access_process))
    .route("/api/notifications", get(get_notifications))
    .layer(CorsLayer::permissive())
    .with_state(state)
```

### 2. Handler Layer (`handlers.rs`)

**Responsibilities**:
- Request validation and parsing
- Business logic orchestration
- Role-based access control enforcement
- Response formatting
- Error handling and HTTP status codes

**Pattern**: Each handler follows this flow:
```rust
async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RequestType>,
) -> Result<ResponseJson<ResponseType>, StatusCode> {
    // 1. Validate input
    // 2. Check permissions/roles
    // 3. Perform business logic
    // 4. Format response
}
```

### 3. Database Layer (`database.rs`, `database/queries.rs`)

**Responsibilities**:
- Connection pool management
- Migration execution
- Type-safe query execution via SQLx
- Transaction management

**Key Operations**:
- `init_database()`: Establishes connection and runs migrations
- `create_user()`: User registration with Stellar integration
- `create_process()`: Process creation with encryption
- `create_process_share()`: Record blockchain sharing event
- `list_process_accesses_by_client()`: Audit trail retrieval

### 4. Models Layer (`models.rs`)

**Responsibilities**:
- Data structure definitions
- Serialization/deserialization
- Type conversions (database ↔ API)
- Domain model documentation

**Key Entities**:
```rust
// Database entities (with FromRow)
struct User { ... }
struct Process { ... }
struct ProcessShare { ... }
struct ProcessAccess { ... }

// API requests
struct RegisterRequest { ... }
struct CreateProcessRequest { ... }
struct ShareProcessRequest { ... }

// API responses
struct UserResponse { ... }
struct ProcessResponse { ... }
struct ProcessAccessResponse { ... }
```

### 5. Cryptography Layer (`crypto.rs`)

**Responsibilities**:
- Encryption key generation
- AES-256-GCM encryption/decryption
- Secure random number generation

**API**:
```rust
fn generate_key() -> String
fn encrypt_content(content: &str, key: &str) -> Result<String>
fn decrypt_content(encrypted: &str, key: &str) -> Result<String>
```

### 6. Blockchain Layer (`stellar_real.rs`, `stellar.rs`)

**Responsibilities**:
- Stellar account generation
- Testnet funding
- Transaction submission
- Blockchain verification

**Key Operations**:
```rust
impl StellarClient {
    fn new_testnet() -> Self
    fn generate_keypair() -> Result<StellarAccount>
    async fn fund_testnet_account(public_key: &str) -> Result<bool>
    async fn share_process_transaction(...) -> Result<TransactionResult>
}
```

### 7. Authentication Layer (`auth.rs`)

**Responsibilities**:
- Password hashing with bcrypt
- Password verification
- Security parameter management

**API**:
```rust
impl Auth {
    fn hash_password(password: &str) -> Result<String>
    fn verify_password(password: &str, hash: &str) -> Result<bool>
}
```

---

## Database Schema

### Entity Relationship Diagram

```
┌─────────────┐         ┌──────────────┐         ┌──────────────────┐
│   users     │         │  processes   │         │ process_shares   │
├─────────────┤         ├──────────────┤         ├──────────────────┤
│ id (PK)     │────┬───>│ client_id(FK)│<────────│ process_id (FK)  │
│ username    │    │    │ id (PK)      │         │ id (PK)          │
│ name        │    │    │ title        │         │ partner_pubkey   │
│ stellar_pk  │    │    │ description  │         │ stellar_tx_hash  │
│ stellar_sk  │    │    │ encrypted_   │         │ shared_at        │
│ password_   │    │    │ content      │         └──────────────────┘
│ hash        │    │    │ encryption_  │
│ roles       │    │    │ key          │         ┌──────────────────┐
│ created_at  │    │    │ status       │         │ process_accesses │
└─────────────┘    │    │ created_at   │         ├──────────────────┤
                   │    └──────────────┘         │ process_id (FK)  │
                   │            │                │ partner_id (FK)  │
                   │            └────────────────│ id (PK)          │
                   └─────────────────────────────│ accessed_at      │
                                                 └──────────────────┘
```

### Table Definitions

#### users
```sql
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    stellar_public_key TEXT NOT NULL,
    stellar_secret_key TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    roles TEXT NOT NULL,  -- JSON: ["client"] | ["partner"] | both
    created_at TEXT NOT NULL
);
```

**Purpose**: Stores user accounts with Stellar integration and role-based access control.

#### processes
```sql
CREATE TABLE processes (
    id TEXT PRIMARY KEY,
    client_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    encrypted_content TEXT NOT NULL,
    encryption_key TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (client_id) REFERENCES users(id)
);
```

**Purpose**: Stores encrypted NDA processes with metadata.

#### process_shares
```sql
CREATE TABLE process_shares (
    id TEXT PRIMARY KEY,
    process_id TEXT NOT NULL,
    partner_public_key TEXT NOT NULL,
    stellar_transaction_hash TEXT NOT NULL,
    shared_at TEXT NOT NULL,
    FOREIGN KEY (process_id) REFERENCES processes(id)
);
```

**Purpose**: Records blockchain-verified sharing events.

#### process_accesses
```sql
CREATE TABLE process_accesses (
    id TEXT PRIMARY KEY,
    process_id TEXT NOT NULL,
    partner_id TEXT NOT NULL,
    accessed_at TEXT NOT NULL,
    FOREIGN KEY (process_id) REFERENCES processes(id),
    FOREIGN KEY (partner_id) REFERENCES users(id)
);
```

**Purpose**: Audit trail of all process access events.

### Migration System

**Location**: `migrations/*.sql`

**Execution**: Automatic on server startup via SQLx

**Version Control**: Timestamp-prefixed filenames ensure ordered execution

**Example Migration**:
```sql
-- migrations/20241201000001_initial.sql
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    ...
);
```

---

## Security Architecture

### Defense in Depth

The system implements multiple security layers:

#### 1. Role-Based Access Control (RBAC)

```rust
// User roles stored as JSON array
pub struct User {
    roles: String, // ["client"], ["partner"], or ["client","partner"]
}

impl User {
    pub fn is_client(&self) -> bool {
        self.roles.contains("client")
    }
    
    pub fn is_partner(&self) -> bool {
        self.roles.contains("partner")
    }
}
```

**Enforcement Points**:
- Process creation: Requires `client` role
- Process access: Requires `partner` role
- List processes: Only returns user's own processes
- Sharing verification: Checks blockchain records

#### 2. Content Encryption

**Algorithm**: AES-256-GCM (Authenticated Encryption with Associated Data)

**Key Management**:
- Unique 256-bit key per process
- Keys stored separately from encrypted content
- Base64 encoding for database storage

**Encryption Flow**:
```rust
// Generation
let key = generate_key(); // 32 random bytes

// Encryption
let encrypted = encrypt_content(content, &key)?;
// Returns: base64(nonce || ciphertext || tag)

// Decryption
let content = decrypt_content(encrypted, &key)?;
```

**Security Properties**:
- Confidentiality: Content unreadable without key
- Integrity: Authentication tag prevents tampering
- Uniqueness: Random nonce for each encryption

#### 3. Password Security

**Algorithm**: Bcrypt with automatic salt generation

**Implementation**:
```rust
impl Auth {
    fn hash_password(password: &str) -> Result<String> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
    }
    
    fn verify_password(password: &str, hash: &str) -> Result<bool> {
        bcrypt::verify(password, hash)
    }
}
```

**Security Properties**:
- Adaptive hashing (CPU-intensive to crack)
- Automatic salt generation
- Constant-time comparison prevents timing attacks

#### 4. Blockchain Immutability

**Integration**: Stellar network transactions

**Security Benefits**:
- Immutable sharing records
- Cryptographic proof of authorization
- Time-stamped audit trail
- Dispute resolution evidence

---

## Blockchain Integration

### Stellar Network Architecture

```
┌─────────────────┐
│  NDA Backend    │
│                 │
│ ┌─────────────┐ │
│ │ Stellar     │ │
│ │ Client      │ │
│ └──────┬──────┘ │
└────────┼────────┘
         │
         ▼
┌─────────────────┐
│ Stellar Horizon │ ◄──── Testnet: https://horizon-testnet.stellar.org
│ API Server      │       Mainnet: https://horizon.stellar.org
└─────────────────┘
         │
         ▼
┌─────────────────┐
│ Stellar Core    │
│ (Blockchain)    │
└─────────────────┘
```

### Transaction Flow

#### 1. User Registration
```rust
// Generate keypair
let account = StellarClient::generate_keypair()?;
// Returns: { public_key, secret_key }

// Fund account (testnet only)
stellar_client.fund_testnet_account(&account.public_key).await?;

// Store in database
create_user(..., &account.public_key, &account.secret_key, ...);
```

#### 2. Process Sharing
```rust
// Create blockchain transaction
let tx_result = stellar_client.share_process_transaction(
    &client.stellar_secret_key,    // Sender's secret key
    &partner_public_key,             // Recipient's public key
    &process_id,                     // Process reference
    &format!("NDA_SHARE:{}", process_id), // Memo
).await?;

// Store transaction hash
create_process_share(..., &tx_result.hash);
```

**Transaction Structure**:
- **Operation**: Payment (1 XLM)
- **Memo**: Text with process ID
- **Result**: Immutable transaction hash

#### 3. Access Verification
```rust
// Check sharing record exists
let share = query!(
    "SELECT * FROM process_shares 
     WHERE process_id = ? AND partner_public_key = ?",
    process_id, partner_public_key
).fetch_optional(&pool).await?;

if share.is_none() {
    return Err(StatusCode::FORBIDDEN);
}

// Decrypt and return content
```

### Stellar Account Structure

Each user gets:
- **Public Key**: 56-character string starting with 'G'
- **Secret Key**: 56-character string starting with 'S' (encrypted in production)
- **Testnet Funding**: 10,000 XLM for testing
- **Transaction Capability**: Can send/receive payments

---

## Encryption System

### AES-256-GCM Implementation

#### Key Generation
```rust
use ring::rand::{SecureRandom, SystemRandom};

fn generate_key() -> String {
    let rng = SystemRandom::new();
    let mut key_bytes = [0u8; 32]; // 256 bits
    rng.fill(&mut key_bytes).unwrap();
    base64::encode(&key_bytes)
}
```

#### Encryption Process
```rust
use ring::aead::{self, Aad, LessSafeKey, Nonce, UnboundKey};

fn encrypt_content(content: &str, key_b64: &str) -> Result<String> {
    // 1. Decode key
    let key_bytes = base64::decode(key_b64)?;
    let unbound_key = UnboundKey::new(&aead::AES_256_GCM, &key_bytes)?;
    let key = LessSafeKey::new(unbound_key);
    
    // 2. Generate random nonce
    let mut nonce_bytes = [0u8; 12];
    SystemRandom::new().fill(&mut nonce_bytes)?;
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);
    
    // 3. Encrypt
    let mut in_out = content.as_bytes().to_vec();
    key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)?;
    
    // 4. Combine nonce + ciphertext + tag
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&in_out);
    
    // 5. Base64 encode
    Ok(base64::encode(&result))
}
```

#### Decryption Process
```rust
fn decrypt_content(encrypted_b64: &str, key_b64: &str) -> Result<String> {
    // 1. Decode
    let encrypted = base64::decode(encrypted_b64)?;
    let key_bytes = base64::decode(key_b64)?;
    
    // 2. Extract nonce
    let (nonce_bytes, ciphertext) = encrypted.split_at(12);
    let nonce = Nonce::assume_unique_for_key(*nonce_bytes);
    
    // 3. Decrypt and verify
    let unbound_key = UnboundKey::new(&aead::AES_256_GCM, &key_bytes)?;
    let key = LessSafeKey::new(unbound_key);
    let mut in_out = ciphertext.to_vec();
    let plaintext = key.open_in_place(nonce, Aad::empty(), &mut in_out)?;
    
    // 4. Convert to string
    Ok(String::from_utf8(plaintext.to_vec())?)
}
```

### Security Guarantees

| Property | Mechanism | Benefit |
|----------|-----------|---------|
| **Confidentiality** | AES-256 encryption | Content unreadable without key |
| **Integrity** | GCM authentication tag | Tampering detected |
| **Authenticity** | AEAD | Origin verification |
| **Uniqueness** | Random nonce | Prevents replay attacks |
| **Forward Secrecy** | Unique keys per process | Compromise limited to one process |

---

## API Design Patterns

### RESTful Principles

| HTTP Method | Usage | Idempotent | Example |
|-------------|-------|------------|---------|
| GET | Read resources | Yes | `/api/processes?client_id=123` |
| POST | Create resources | No | `/api/processes` |
| PUT | Update/replace | Yes | Not used in MVP |
| DELETE | Remove resources | Yes | Not used in MVP |

### Request/Response Patterns

#### Standard Success Response
```rust
Ok(ResponseJson(data))
// HTTP 200 + JSON body
```

#### Error Handling
```rust
Err(StatusCode::NOT_FOUND)
Err(StatusCode::FORBIDDEN)
Err(StatusCode::INTERNAL_SERVER_ERROR)
```

#### Query Parameters
```rust
#[derive(Deserialize)]
struct ListProcessesQuery {
    client_id: Option<String>,
}

async fn list_processes(
    Query(params): Query<ListProcessesQuery>
) -> Result<...> {
    let id = params.client_id.ok_or(StatusCode::BAD_REQUEST)?;
    // ...
}
```

### Dependency Injection

```rust
#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
}

let state = Arc::new(AppState { pool });

Router::new()
    .route("/api/users/register", post(register_user))
    .with_state(state);

async fn register_user(
    State(state): State<Arc<AppState>>, // Injected automatically
    Json(payload): Json<RegisterRequest>,
) -> Result<...> {
    // Use state.pool for database operations
}
```

---

## Deployment Architecture

### Development Environment

```
┌─────────────────────────────────────────┐
│          Development Machine            │
│                                         │
│  ┌───────────────┐  ┌─────────────┐   │
│  │ nda-backend   │  │ nda-manager │   │
│  │ (Rust/Axum)   │  │ (Angular 20)│   │
│  │ Port: 3000    │  │ Port: 4200  │   │
│  └───────┬───────┘  └──────┬──────┘   │
│          │                  │           │
│          │  ┌──────────────┤           │
│          │  │               │           │
│          ▼  ▼               ▼           │
│     SQLite DB      Angular Proxy       │
│  stellar_mvp.db   (to localhost:3000)  │
│                                         │
└─────────────────────────────────────────┘
           │
           ▼
┌─────────────────────┐
│ Stellar Testnet     │
│ (horizon-testnet)   │
└─────────────────────┘
```

**Commands**:
```bash
# Backend
cd nda-backend
cargo run

# Frontend (separate terminal)
cd nda-manager
ng serve
```

### Production Environment (Recommended)

```
                      ┌────────────────┐
                      │  Load Balancer │
                      │   (HTTPS/TLS)  │
                      └────────┬───────┘
                               │
              ┌────────────────┴────────────────┐
              │                                  │
              ▼                                  ▼
    ┌─────────────────┐              ┌─────────────────┐
    │  Backend Server │              │  Backend Server │
    │  (Axum + Rust)  │              │  (Axum + Rust)  │
    │  Port: 3000     │              │  Port: 3000     │
    └────────┬────────┘              └────────┬────────┘
             │                                │
             └────────────┬───────────────────┘
                          │
                  ┌───────┴────────┐
                  │                │
                  ▼                ▼
         ┌───────────────┐  ┌─────────────┐
         │  PostgreSQL   │  │   Redis     │
         │  (Primary DB) │  │  (Cache)    │
         └───────────────┘  └─────────────┘
                │
                ▼
         ┌─────────────────┐
         │ Stellar Mainnet │
         │   (Production)  │
         └─────────────────┘

    ┌──────────────────────┐
    │   Static Frontend    │
    │  (CDN/S3/Nginx)      │
    │  (Angular Build)     │
    └──────────────────────┘
```

### Deployment Checklist

#### Security
- [ ] Enable HTTPS/TLS encryption
- [ ] Configure proper CORS (whitelist specific origins)
- [ ] Implement rate limiting
- [ ] Use environment variables for secrets
- [ ] Enable firewall rules
- [ ] Set up KMS for encryption keys
- [ ] Switch to Stellar mainnet

#### Database
- [ ] Migrate SQLite → PostgreSQL/MySQL
- [ ] Enable database backups
- [ ] Configure connection pooling
- [ ] Set up read replicas (if needed)

#### Monitoring
- [ ] Set up application logging (structured)
- [ ] Configure error tracking (e.g., Sentry)
- [ ] Enable metrics collection (Prometheus)
- [ ] Set up health check monitoring
- [ ] Configure alerting rules

#### Performance
- [ ] Enable response compression
- [ ] Configure caching strategy
- [ ] Set up CDN for frontend
- [ ] Optimize database queries
- [ ] Enable HTTP/2

#### High Availability
- [ ] Multiple backend instances
- [ ] Load balancer configuration
- [ ] Database replication
- [ ] Failover mechanisms
- [ ] Backup/restore procedures

---

## Performance Considerations

### Database Optimization

**Indexes**:
```sql
CREATE INDEX idx_processes_client_id ON processes(client_id);
CREATE INDEX idx_process_shares_process_id ON process_shares(process_id);
CREATE INDEX idx_process_accesses_process_id ON process_accesses(process_id);
```

**Query Patterns**:
- Use prepared statements (SQLx does this automatically)
- Limit result sets with pagination
- Avoid N+1 queries with JOIN operations
- Use connection pooling

### Encryption Performance

- **Hardware Acceleration**: AES-NI instructions when available
- **Batching**: Encrypt multiple operations in parallel (future)
- **Caching**: Cache frequently accessed decrypted content (with TTL)

### Blockchain Performance

- **Async Operations**: Non-blocking Stellar API calls
- **Timeout Configuration**: Set appropriate timeouts for network calls
- **Retry Logic**: Implement exponential backoff for failed transactions
- **Transaction Batching**: Group operations when possible (future)

---

## Testing Strategy

### Unit Tests
```bash
cd nda-backend
cargo test
```

**Coverage Areas**:
- Encryption/decryption functions
- Password hashing/verification
- Model transformations
- Business logic functions

### Integration Tests
```bash
cargo run --bin test_stellar
```

**Coverage Areas**:
- Database operations
- Stellar blockchain integration
- End-to-end API flows

### Load Testing (Future)
```bash
# Example with Apache Bench
ab -n 1000 -c 10 http://localhost:3000/health
```

---

## Future Enhancements

### Planned Features

1. **JWT Authentication**
   - Token-based session management
   - Refresh token mechanism
   - Token expiration and rotation

2. **Advanced Access Control**
   - Time-limited access grants
   - Revocation mechanism
   - Multi-signature approvals

3. **Process Versioning**
   - Content history tracking
   - Diff visualization
   - Rollback capability

4. **Search & Filtering**
   - Full-text search on titles/descriptions
   - Advanced filtering options
   - Sorting by multiple criteria

5. **Notifications**
   - Real-time WebSocket updates
   - Email notifications
   - Webhook integrations

6. **Analytics Dashboard**
   - Access pattern analysis
   - User activity metrics
   - Compliance reporting

---

## Conclusion

The NDA Backend provides a robust, secure, and scalable foundation for blockchain-secured contract management. The architecture emphasizes:

- **Security**: Multi-layer defense with encryption, blockchain, and RBAC
- **Performance**: Async operations and efficient database queries
- **Compliance**: Complete audit trails and immutable records
- **Scalability**: Modular design supporting horizontal scaling
- **Maintainability**: Clear separation of concerns and comprehensive documentation

For questions or contributions, please refer to the project README and contribution guidelines.
