# NDA Backend - Quick Start Guide

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (version 1.70+): [Install Rust](https://rustup.rs/)
- **SQLite3**: Usually pre-installed on most systems
- **Git**: For cloning the repository

Optional (for frontend integration):
- **Node.js** (version 20+) and npm
- **Angular CLI**: `npm install -g @angular/cli`

---

## Installation

### 1. Clone the Repository

```bash
git clone <repository-url>
cd nda-manager/nda-backend
```

### 2. Install Dependencies

Cargo will automatically download and compile dependencies:

```bash
cargo build
```

This may take a few minutes on first run as it compiles all dependencies.

---

## Running the Backend

### Development Mode

Start the server with debug logging:

```bash
cargo run
```

**Expected Output**:
```
🚀 Server running at http://localhost:3000
📊 Health check available at http://localhost:3000/health
📋 API documentation: All endpoints support JSON request/response
🔐 Security: AES-256-GCM encryption + Stellar blockchain integration
```

The server will:
- Bind to `0.0.0.0:3000`
- Create `stellar_mvp.db` if it doesn't exist
- Automatically run database migrations
- Enable debug-level logging

### With Custom Database

```bash
DATABASE_URL=sqlite:./my_database.db cargo run
```

### Production Mode

Build optimized release binary:

```bash
cargo build --release
./target/release/nda-backend
```

---

## Verify Installation

### 1. Health Check

```bash
curl http://localhost:3000/health
```

**Expected Response**:
```json
{
  "status": "OK",
  "timestamp": "2024-01-01T00:00:00Z"
}
```

### 2. Database Check

Verify migrations ran successfully:

```bash
cargo run --bin check_db
```

### 3. Stellar Integration Test

Test blockchain connectivity:

```bash
cargo run --bin test_stellar
```

---

## Quick API Examples

### Register a User

```bash
curl -X POST http://localhost:3000/api/users/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "test_client",
    "name": "Test Client",
    "password": "secure_password",
    "roles": ["client"]
  }'
```

**Response**:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "test_client",
  "name": "Test Client",
  "stellar_public_key": "GCKFBEIYTKP...",
  "roles": ["client"],
  "created_at": "2024-01-01T00:00:00Z"
}
```

### Create an NDA Process

```bash
curl -X POST http://localhost:3000/api/processes \
  -H "Content-Type: application/json" \
  -d '{
    "client_id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Test NDA",
    "description": "Test confidential agreement",
    "confidential_content": "This is sensitive information"
  }'
```

### List Processes

```bash
curl "http://localhost:3000/api/processes?client_id=550e8400-e29b-41d4-a716-446655440000"
```

---

## Project Structure

```
nda-backend/
├── Cargo.toml              # Rust dependencies and project config
├── README.md               # Project overview
├── stellar_mvp.db          # SQLite database (auto-created)
├── docs/                   # Documentation
│   ├── API_REFERENCE.md    # Complete API documentation
│   ├── ARCHITECTURE.md     # System architecture guide
│   └── QUICKSTART.md       # This file
├── migrations/             # Database migrations (auto-applied)
│   ├── 20241201000001_initial.sql
│   ├── 20241202000001_add_roles_system.sql
│   └── ...
├── src/
│   ├── main.rs            # Application entry point
│   ├── handlers.rs        # HTTP request handlers
│   ├── models.rs          # Data structures
│   ├── database.rs        # Database initialization
│   ├── crypto.rs          # Encryption functions
│   ├── stellar_real.rs    # Blockchain integration
│   ├── auth.rs            # Authentication utilities
│   └── bin/               # Utility programs
│       ├── test_stellar.rs
│       ├── check_db.rs
│       └── ...
└── database/
    └── queries.rs         # Database query functions
```

---

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `sqlite:./stellar_mvp.db` | SQLite database path |
| `RUST_LOG` | `debug` | Logging level (trace, debug, info, warn, error) |

### Example: Custom Configuration

```bash
export DATABASE_URL=sqlite:./production.db
export RUST_LOG=info
cargo run
```

Or create a `.env` file (not tracked in git):

```bash
# .env
DATABASE_URL=sqlite:./production.db
RUST_LOG=info
```

---

## VS Code Integration

### Recommended Extensions

- **rust-analyzer**: Rust language server
- **CodeLLDB**: Debugging support
- **Better TOML**: Cargo.toml syntax highlighting

### Running Tasks

Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on Mac), then:

1. **Tasks: Run Task**
2. Select:
   - `rust: cargo run` - Start backend server
   - `rust: cargo build` - Build debug version
   - `rust: cargo build --release` - Build optimized version
   - `rust: cargo test --no-run` - Compile tests

### Debugging

1. Set breakpoints in VS Code
2. Press `F5` or go to Run → Start Debugging
3. Use debug console to inspect variables

---

## Running with Frontend

### 1. Start Backend

```bash
cd nda-backend
cargo run
```

Backend will run on `http://localhost:3000`

### 2. Start Frontend (Separate Terminal)

```bash
cd nda-manager
ng serve
```

Frontend will run on `http://localhost:4200` and proxy API requests to backend.

### 3. Access Application

Open browser: `http://localhost:4200`

---

## Common Issues & Solutions

### Issue: "Port 3000 already in use"

**Solution**: Kill existing process or use different port

```bash
# Find process
lsof -i :3000  # Mac/Linux
netstat -ano | findstr :3000  # Windows

# Kill process
kill -9 <PID>  # Mac/Linux
taskkill /PID <PID> /F  # Windows
```

### Issue: Database migration errors

**Solution**: Delete database and restart (development only)

```bash
rm stellar_mvp.db
cargo run
```

### Issue: Stellar testnet connection timeout

**Solution**: Check internet connection or retry. Testnet may be temporarily unavailable.

```bash
# Test connectivity
curl https://horizon-testnet.stellar.org/

# Retry after a moment
cargo run
```

### Issue: Compilation errors

**Solution**: Update Rust toolchain

```bash
rustup update stable
cargo clean
cargo build
```

---

## Database Management

### Inspect Database

Use SQLite CLI or GUI tool (e.g., DB Browser for SQLite):

```bash
sqlite3 stellar_mvp.db

# Inside SQLite shell
.tables                          # List all tables
.schema users                    # Show users table schema
SELECT * FROM users LIMIT 5;     # Query users
.exit                            # Exit
```

### Reset Database

⚠️ **Warning**: This will delete all data!

```bash
rm stellar_mvp.db
cargo run  # Recreates database with migrations
```

### Backup Database

```bash
cp stellar_mvp.db stellar_mvp.db.backup
```

### Run Specific Migration

```bash
cargo run --bin apply_migration
```

---

## Testing

### Run Unit Tests

```bash
cargo test
```

### Run Specific Test

```bash
cargo test test_name
```

### Test with Output

```bash
cargo test -- --nocapture
```

### Test Stellar Integration

```bash
cargo run --bin test_stellar
```

**Expected Output**:
```
✅ Keypair generation successful
✅ Testnet funding successful
✅ Transaction submission successful
```

---

## API Testing Tools

### Using cURL

See examples above or check `docs/API_REFERENCE.md`

### Using Postman/Insomnia

1. Import base URL: `http://localhost:3000`
2. Create requests for each endpoint
3. Set Content-Type: `application/json`
4. Use JSON bodies for POST requests

### Using httpie

```bash
# Install httpie
pip install httpie

# Register user
http POST localhost:3000/api/users/register \
  username=test_user \
  name="Test User" \
  password=secure123 \
  roles:='["client"]'
```

---

## Logging

### Log Levels

```bash
# Trace (most verbose)
RUST_LOG=trace cargo run

# Debug (development default)
RUST_LOG=debug cargo run

# Info (production recommended)
RUST_LOG=info cargo run

# Warn (only warnings and errors)
RUST_LOG=warn cargo run

# Error (only errors)
RUST_LOG=error cargo run
```

### Module-Specific Logging

```bash
# Only log from specific module
RUST_LOG=nda_backend::handlers=debug cargo run

# Multiple modules
RUST_LOG=nda_backend::handlers=debug,nda_backend::database=info cargo run
```

---

## Performance Tips

### Release Build

Always use release build for performance testing:

```bash
cargo build --release
./target/release/nda-backend
```

**Performance Improvements**:
- ~10x faster execution
- Optimized binary size
- Better memory usage

### Connection Pooling

SQLx automatically manages connection pool. Default settings work well for most cases.

To customize, modify `database.rs`:

```rust
SqlitePoolOptions::new()
    .max_connections(20)  // Adjust as needed
    .connect(&database_url)
    .await?
```

---

## Security Best Practices

### Development

✅ **DO**:
- Use strong passwords in testing
- Keep `stellar_mvp.db` in `.gitignore`
- Use environment variables for configuration
- Test on localhost only

❌ **DON'T**:
- Commit database files
- Expose server to public internet
- Use production keys in development
- Share secret keys

### Production

✅ **DO**:
- Enable HTTPS/TLS
- Use Key Management Service (KMS)
- Implement rate limiting
- Configure proper CORS
- Regular security audits
- Monitor logs for suspicious activity

❌ **DON'T**:
- Use testnet in production
- Store secrets in code
- Use permissive CORS
- Disable logging

---

## Next Steps

### Learn More

1. **API Reference**: `docs/API_REFERENCE.md`
   - Complete endpoint documentation
   - Request/response examples
   - Error handling guide

2. **Architecture Guide**: `docs/ARCHITECTURE.md`
   - System design overview
   - Security architecture
   - Database schema
   - Deployment strategies

3. **Code Documentation**:
   ```bash
   cargo doc --open
   ```
   - Generates and opens Rust documentation

### Explore Features

- Create users with different roles
- Set up process sharing workflow
- Test blockchain integration
- Review audit trails
- Experiment with encryption

### Integrate with Frontend

- Connect Angular application
- Implement authentication flow
- Create process management UI
- Display access notifications

---

## Getting Help

### Documentation

- **API Reference**: `docs/API_REFERENCE.md`
- **Architecture**: `docs/ARCHITECTURE.md`
- **Code Comments**: Extensive inline documentation in `src/`

### Debugging

Enable detailed logging:

```bash
RUST_LOG=trace cargo run
```

Check database state:

```bash
cargo run --bin check_db
```

Test Stellar connectivity:

```bash
cargo run --bin test_stellar
```

### Common Commands Reference

```bash
# Development
cargo run                    # Start server
cargo build                  # Build debug
cargo test                   # Run tests
cargo check                  # Check compilation
cargo clean                  # Clean build artifacts

# Release
cargo build --release        # Build optimized
cargo run --release          # Run optimized

# Utilities
cargo run --bin check_db     # Check database
cargo run --bin test_stellar # Test blockchain

# Documentation
cargo doc --open             # Generate and open docs
```

---

## Quick Reference Card

### Server

| Action | Command |
|--------|---------|
| Start server | `cargo run` |
| Start (release) | `cargo run --release` |
| Health check | `curl localhost:3000/health` |

### Development

| Action | Command |
|--------|---------|
| Build | `cargo build` |
| Test | `cargo test` |
| Check | `cargo check` |
| Format | `cargo fmt` |
| Lint | `cargo clippy` |

### Database

| Action | Command |
|--------|---------|
| Inspect | `sqlite3 stellar_mvp.db` |
| Check | `cargo run --bin check_db` |
| Reset | `rm stellar_mvp.db && cargo run` |
| Backup | `cp stellar_mvp.db stellar_mvp.db.backup` |

### API Endpoints

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/health` | GET | Health check |
| `/api/users/register` | POST | Register user |
| `/api/users/login` | POST | Login user |
| `/api/processes` | POST | Create process |
| `/api/processes` | GET | List processes |
| `/api/processes/share` | POST | Share process |
| `/api/processes/access` | POST | Access process |
| `/api/notifications` | GET | Get notifications |

---

## Congratulations! 🎉

You're now ready to use the NDA Backend. Start by:

1. ✅ Verifying health endpoint
2. ✅ Registering test users
3. ✅ Creating encrypted processes
4. ✅ Testing blockchain sharing

For detailed API usage, see `docs/API_REFERENCE.md`

Happy coding! 🚀
