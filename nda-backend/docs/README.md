# NDA Backend Documentation

Comprehensive documentation for the blockchain-secured NDA contract management backend system.

## Documentation Index

### 📘 [API Reference](API_REFERENCE.md)
Complete API documentation with all endpoints, request/response formats, and examples.

**Contents**:
- Health Check
- User Management (Register, Login, Auto-Login)
- Process Management (Create, List)
- Sharing & Access (Blockchain-secured)
- Audit & Compliance (Notifications)
- Error Codes
- Security Model
- Complete Request Flow Examples

**Best for**: Frontend developers, API consumers, integration partners

---

### 🏗️ [Architecture Guide](ARCHITECTURE.md)
Deep dive into system design, security architecture, and technical implementation.

**Contents**:
- System Overview
- Architecture Layers (HTTP, Handlers, Database, Models, Crypto, Blockchain, Auth)
- Database Schema & ERD
- Security Architecture (RBAC, Encryption, Blockchain)
- Blockchain Integration (Stellar Network)
- Encryption System (AES-256-GCM)
- API Design Patterns
- Deployment Architecture
- Performance Considerations
- Testing Strategy

**Best for**: Backend developers, architects, security auditors, DevOps engineers

---

### 🚀 [Quick Start Guide](QUICKSTART.md)
Step-by-step guide to get the backend running quickly.

**Contents**:
- Prerequisites
- Installation
- Running the Backend
- Verify Installation
- Quick API Examples
- Project Structure
- Environment Variables
- VS Code Integration
- Running with Frontend
- Common Issues & Solutions
- Database Management
- Testing
- API Testing Tools
- Logging
- Performance Tips
- Security Best Practices

**Best for**: New developers, quick setup, troubleshooting

---

## Quick Links

### Common Tasks

- **Start Backend**: `cargo run` (from `nda-backend/`)
- **Health Check**: `curl http://localhost:3000/health`
- **Run Tests**: `cargo test`
- **View Docs**: `cargo doc --open`

### Key Files

- `src/main.rs` - Application entry point and route configuration
- `src/handlers.rs` - HTTP request handlers with comprehensive documentation
- `src/models.rs` - Data structures and API contracts
- `src/database.rs` - Database initialization and migrations
- `src/crypto.rs` - AES-256-GCM encryption implementation
- `src/stellar_real.rs` - Stellar blockchain integration
- `src/auth.rs` - Password hashing and authentication

### Database

- **Schema**: See [ARCHITECTURE.md - Database Schema](ARCHITECTURE.md#database-schema)
- **Migrations**: `migrations/*.sql` (auto-applied on startup)
- **Inspect**: `sqlite3 stellar_mvp.db` or use DBeaver

---

## System Overview

The NDA Backend is a **Rust-based REST API** for managing blockchain-secured Non-Disclosure Agreement (NDA) contracts. It provides:

### Core Features

✅ **User Management**
- Role-based system (client, partner, or both)
- Automatic Stellar blockchain account creation
- Bcrypt password authentication

✅ **Process Management**
- AES-256-GCM encrypted content storage
- Unique encryption key per process
- Secure metadata management

✅ **Blockchain Integration**
- Immutable sharing records on Stellar network
- Cryptographic proof of authorization
- Testnet support for development

✅ **Access Control**
- Verification before content decryption
- Complete audit trail for compliance
- Time-stamped access events

### Technology Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Web Framework | Axum | High-performance async HTTP |
| Database | SQLite + SQLx | Embedded database with type safety |
| Encryption | AES-256-GCM | Confidential content protection |
| Blockchain | Stellar Network | Immutable sharing records |
| Authentication | Bcrypt | Password hashing |
| Runtime | Tokio | Async operations |
| Logging | Tracing | Structured logging |

---

## Getting Started

### Prerequisites

- Rust 1.70+ ([Install](https://rustup.rs/))
- SQLite3 (usually pre-installed)
- Git

### Quick Start

```bash
# Clone and navigate
cd nda-backend

# Build and run
cargo run

# Verify
curl http://localhost:3000/health
```

**Expected Output**:
```
🚀 Server running at http://localhost:3000
📊 Health check available at http://localhost:3000/health
🔐 Security: AES-256-GCM encryption + Stellar blockchain integration
```

For detailed setup instructions, see [QUICKSTART.md](QUICKSTART.md)

---

## API Overview

### Base URL
`http://localhost:3000`

### Key Endpoints

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/health` | GET | Health check |
| `/api/users/register` | POST | Register new user |
| `/api/users/login` | POST | Authenticate user |
| `/api/processes` | POST | Create encrypted process |
| `/api/processes` | GET | List client's processes |
| `/api/processes/share` | POST | Share via blockchain |
| `/api/processes/access` | POST | Access with decryption |
| `/api/notifications` | GET | Get audit trail |

For complete API documentation, see [API_REFERENCE.md](API_REFERENCE.md)

---

## Security Features

### Multi-Layer Protection

1. **Role-Based Access Control**
   - Client role: Create and manage processes
   - Partner role: Access shared processes
   - Hybrid roles supported

2. **End-to-End Encryption**
   - AES-256-GCM for all confidential content
   - Unique key per process
   - Hardware-accelerated when available

3. **Blockchain Verification**
   - Stellar network integration
   - Immutable sharing records
   - Cryptographic proof

4. **Complete Audit Trail**
   - All access logged
   - Time-stamped events
   - Regulatory compliance ready

For detailed security architecture, see [ARCHITECTURE.md - Security Architecture](ARCHITECTURE.md#security-architecture)

---

## Business Flow

### Complete NDA Workflow

```
1. Client Registration
   ↓
2. Partner Registration
   ↓
3. Client Creates Encrypted Process (AES-256-GCM)
   ↓
4. Client Shares via Blockchain (Stellar Transaction)
   ↓
5. Partner Accesses Content (Verification + Decryption)
   ↓
6. Access Logged for Audit Trail
```

### Example: Sharing Confidential Content

```bash
# 1. Register client
POST /api/users/register
{
  "username": "acme_corp",
  "name": "ACME Corporation",
  "password": "secure_pass",
  "roles": ["client"]
}

# 2. Create encrypted process
POST /api/processes
{
  "client_id": "<client_id>",
  "title": "Project Alpha NDA",
  "description": "Confidential specifications",
  "confidential_content": "Secret algorithms..."
}

# 3. Share via blockchain
POST /api/processes/share
{
  "client_username": "acme_corp",
  "process_id": "<process_id>",
  "partner_public_key": "<stellar_public_key>"
}
# Returns: stellar_transaction_hash (immutable proof)

# 4. Partner accesses content
POST /api/processes/access
{
  "process_id": "<process_id>",
  "partner_username": "partner_corp",
  "partner_public_key": "<stellar_public_key>"
}
# Returns: decrypted content + logs access
```

---

## Database Schema

### Core Tables

```
users
├── id (PK)
├── username (unique)
├── name
├── stellar_public_key
├── stellar_secret_key
├── password_hash
├── roles (JSON: ["client"] | ["partner"] | both)
└── created_at

processes
├── id (PK)
├── client_id (FK → users.id)
├── title
├── description
├── encrypted_content
├── encryption_key
├── status
└── created_at

process_shares
├── id (PK)
├── process_id (FK → processes.id)
├── partner_public_key
├── stellar_transaction_hash
└── shared_at

process_accesses
├── id (PK)
├── process_id (FK → processes.id)
├── partner_id (FK → users.id)
└── accessed_at
```

For complete schema and relationships, see [ARCHITECTURE.md - Database Schema](ARCHITECTURE.md#database-schema)

---

## Development

### Project Structure

```
nda-backend/
├── Cargo.toml           # Dependencies
├── stellar_mvp.db       # SQLite database (auto-created)
├── docs/                # This documentation
├── migrations/          # Database migrations (auto-applied)
├── src/
│   ├── main.rs         # Entry point + routes
│   ├── handlers.rs     # HTTP handlers
│   ├── models.rs       # Data structures
│   ├── database.rs     # DB initialization
│   ├── crypto.rs       # Encryption
│   ├── stellar_real.rs # Blockchain
│   ├── auth.rs         # Authentication
│   └── bin/            # Utility programs
└── database/
    └── queries.rs      # DB operations
```

### Common Commands

```bash
# Development
cargo run                    # Start server
cargo test                   # Run tests
cargo check                  # Check compilation
cargo clippy                 # Lint code
cargo fmt                    # Format code

# Release
cargo build --release        # Optimized build
cargo run --release          # Run optimized

# Utilities
cargo run --bin check_db     # Verify database
cargo run --bin test_stellar # Test blockchain
cargo doc --open             # Generate docs
```

### VS Code Tasks

- `rust: cargo run` - Start backend server
- `rust: cargo build` - Build debug version
- `rust: cargo build --release` - Build optimized
- `rust: cargo test --no-run` - Compile tests

---

## Testing

### Unit Tests

```bash
cargo test
```

### Stellar Integration Test

```bash
cargo run --bin test_stellar
```

**Expected Output**:
```
✅ Keypair generation successful
✅ Testnet funding successful
✅ Transaction submission successful
```

### Manual API Testing

```bash
# Health check
curl http://localhost:3000/health

# Register user
curl -X POST http://localhost:3000/api/users/register \
  -H "Content-Type: application/json" \
  -d '{"username":"test","name":"Test User","password":"pass","roles":["client"]}'

# Create process
curl -X POST http://localhost:3000/api/processes \
  -H "Content-Type: application/json" \
  -d '{"client_id":"<id>","title":"Test","description":"Desc","confidential_content":"Secret"}'
```

---

## Deployment

### Development

```bash
cargo run
# Binds to localhost:3000
# Uses SQLite
# Connects to Stellar testnet
```

### Production Considerations

✅ **Must Have**:
- HTTPS/TLS encryption
- Proper CORS configuration
- Rate limiting
- Environment-based secrets
- PostgreSQL/MySQL (instead of SQLite)
- Stellar mainnet
- Comprehensive logging
- Monitoring & alerting

❌ **Avoid**:
- HTTP in production
- Permissive CORS
- Hardcoded secrets
- SQLite for high concurrency
- Testnet for real transactions

For detailed deployment guide, see [ARCHITECTURE.md - Deployment Architecture](ARCHITECTURE.md#deployment-architecture)

---

## Troubleshooting

### Common Issues

| Issue | Solution |
|-------|----------|
| Port 3000 in use | Kill process or change port |
| Database errors | Delete `stellar_mvp.db` and restart (dev only) |
| Stellar timeout | Check internet, retry after a moment |
| Compilation errors | `rustup update && cargo clean && cargo build` |

For detailed troubleshooting, see [QUICKSTART.md - Common Issues](QUICKSTART.md#common-issues--solutions)

### Debug Logging

```bash
RUST_LOG=trace cargo run
```

### Database Inspection

```bash
sqlite3 stellar_mvp.db
.tables
SELECT * FROM users LIMIT 5;
.exit
```

---

## Contributing

### Code Style

- Follow Rust naming conventions
- Use `cargo fmt` before committing
- Run `cargo clippy` to check for issues
- Add comprehensive documentation comments
- Write unit tests for new features

### Documentation

- Update API docs when adding endpoints
- Document new models and functions
- Include examples in comments
- Update README for major changes

### Testing

- Write tests for new functionality
- Ensure existing tests pass
- Test blockchain integration
- Verify encryption/decryption

---

## Additional Resources

### Rust Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/)

### Stellar Resources

- [Stellar Documentation](https://developers.stellar.org/)
- [Stellar Testnet](https://horizon-testnet.stellar.org/)
- [Stellar Laboratory](https://laboratory.stellar.org/)

### Security Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)

---

## License & Support

For questions, issues, or contributions, please refer to the project README and contribution guidelines.

**Version**: 1.0.0  
**Last Updated**: November 2025

---

## Quick Reference

### Must-Read Documents

1. **New Developer?** → Start with [QUICKSTART.md](QUICKSTART.md)
2. **Building Frontend?** → Read [API_REFERENCE.md](API_REFERENCE.md)
3. **Understanding System?** → Review [ARCHITECTURE.md](ARCHITECTURE.md)

### Essential Commands

```bash
# Start
cargo run

# Test
curl localhost:3000/health

# Verify
cargo run --bin check_db
cargo run --bin test_stellar
```

### Need Help?

1. Check documentation (this folder)
2. Review inline code comments
3. Run `cargo doc --open` for API docs
4. Enable debug logging: `RUST_LOG=trace cargo run`

---

**Happy Coding! 🚀**
