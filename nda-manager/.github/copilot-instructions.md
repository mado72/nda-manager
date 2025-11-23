# NDA Manager - Copilot Instructions

## Architecture Overview

This is a **blockchain-secured NDA (Non-Disclosure Agreement) contract management system** with a **Rust backend** (`nda-backend`) and **Angular 20 frontend** (`nda-manager`). The system enables secure sharing of confidential contracts where clients create NDAs with sensitive information, generate unique contract keys, and allow partners to digitally "sign" confidentiality terms through blockchain verification.

### Core Business Flow
```
Client Creates Contract → Generates Unique Key → Shares with Partner → 
Partner "Signs" via Blockchain → Establishes Trust Bond → Content Access Granted
```

### Key Components
- **Backend**: Axum web server with SQLite database, Stellar network integration
- **Frontend**: Angular 20 with standalone components, Angular Material UI
- **Security**: End-to-end encryption + blockchain verification for contract signing
- **Database**: SQLite with SQLx for type-safe queries and automatic migrations
- **Contract Workflow**: Unique keys for document sharing and partner signature verification

## Development Workflow

### Running the System
```bash
# Backend (from nda-backend/):
cargo run  # Starts on localhost:3000, auto-runs migrations

# Frontend (from nda-manager/):  
ng serve   # Starts on localhost:4200, proxies to backend
```

### Key VS Code Tasks
- `ng-serve`: Angular dev server with source maps and watch mode
- `rust: cargo run`: Backend server with debug logging and SQLite connection
- All tasks are pre-configured in `.vscode/tasks.json`

## Critical Patterns & Conventions

### Backend Architecture (Rust)
- **Handler Pattern**: `handlers.rs` contains all HTTP endpoints with comprehensive docs
- **Type-Safe Database**: All models in `models.rs` derive `FromRow` for SQLx integration
- **Encryption Layer**: `crypto.rs` handles AES-256-GCM for all sensitive content
- **Stellar Integration**: `stellar_real.rs` manages blockchain operations and account creation
- **Migration System**: Auto-applied on startup, versioned in `migrations/`

### Frontend Architecture (Angular)
- **Standalone Components**: No NgModules, all components are standalone
- **Signal-Based State**: Uses Angular signals for reactive state management
- **Service Pattern**: `services/` contains business logic with observables
- **Route Guards**: `auth.guard.ts` / `unauth.guard.ts` for access control
- **Master-Detail UI**: Main pattern for contracts/processes management

### NDA Contract Flow Patterns
```
Client Registration → Contract Creation with Confidential Data → 
Unique Key Generation → Contract Sharing (Link/Direct) → 
Partner "Signs" Confidentiality Term → Blockchain Trust Bond → 
Secure Access to Confidential Information → Complete Audit Trail
```

### API Structure
- **REST Design**: `/api/users`, `/api/processes`, `/api/share`, `/api/access`
- **Contract Management**: Create, list, and manage NDA contracts with confidential content
- **Sharing Mechanism**: Generate unique contract links for partner access
- **Signature Verification**: Blockchain-based partner "signing" of confidentiality terms
- **JSON Payloads**: All requests/responses use JSON with type-safe models
- **Error Handling**: Standard HTTP status codes with detailed error messages
- **CORS Enabled**: Configured for development frontend access

## Security Implementation

### Contract Security Workflow
1. **Contract Creation**: Confidential content encrypted with unique AES-256-GCM key per contract
2. **Unique Key Generation**: Each contract gets a unique identifier/key for secure sharing
3. **Partner Authentication**: Blockchain transaction serves as digital "signature" of confidentiality terms
4. **Trust Bond Establishment**: Immutable blockchain record creates legal proof of NDA acceptance
5. **Controlled Access**: Content decryption only after verified blockchain signature
6. **Audit Compliance**: Complete traceability for regulatory and legal requirements

### Stellar Blockchain Usage
- **Account Generation**: Ed25519 keypairs auto-generated for all users (clients and partners)
- **Digital Signatures**: Blockchain transactions act as cryptographic "signatures" of NDA terms
- **Testnet Integration**: Auto-funded accounts for development and testing
- **Immutable Records**: Transaction proof cannot be altered, providing legal certainty
- **Trust Verification**: Partners must complete blockchain transaction to access confidential content

## Database Schema Notes

### Core Tables
- `users`: Stellar keypairs + role system (`["client"]`, `["partner"]`, or both)
- `processes`: Encrypted NDA contracts + metadata + client ownership
- `process_shares`: Blockchain transaction hashes proving partner "signature" of confidentiality terms
- `process_accesses`: Audit trail tracking when partners access confidential content
- **Unique Keys**: Each process/contract has unique identifiers for secure sharing links

### Migration Pattern
- SQLite with automatic schema evolution
- Versioned migrations in `migrations/` directory
- Role system migration shows data transformation approach

## Common Development Tasks

### Adding New API Endpoints
1. Add request/response models to `models.rs`
2. Implement handler in `handlers.rs` with full documentation
3. Add route in `main.rs` router configuration
4. Create corresponding Angular service method

### Frontend Component Development
- Use `ng generate component` for new components
- Follow master-detail pattern for contract management
- Implement proper loading/error states with signals
- Add route guards for protected routes (`auth.guard.ts` for authenticated users)

### Contract Management Features
- **Create Contract**: `register-contract` component for NDA creation with confidential content
- **Share Contract**: `share-contract` component generates unique links and handles partner sharing
- **Access Contract**: Partner interface for "signing" confidentiality terms via blockchain
- **List Contracts**: Master-detail view for contract management and status tracking

### Database Changes
- Create new migration file with timestamp prefix
- Test migration with `cargo run` (auto-applies)
- Update models in `models.rs` to match schema
- Use SQLx compile-time verification with `cargo check`

## Environment & Configuration

### Backend Environment Variables
- `DATABASE_URL`: SQLite path (default: `sqlite:./stellar_mvp.db`)
- `RUST_LOG`: Logging level (default: `debug`)
- Production: Add encryption key management and Stellar mainnet config

### Frontend Configuration
- `src/environments/`: Environment-specific API URLs and settings
- Angular Material theme configured in `styles.scss`
- HTTP client configured in `app.config.ts` with providers

## Testing & Debugging

### Backend Testing
- Use `cargo test` for unit tests
- `src/bin/test_stellar.rs` for blockchain integration testing
- Structured logging with `tracing` crate for debugging

### Frontend Testing  
- `ng test` runs Karma/Jasmine tests
- Example test in `jasmine-demo.spec.ts`
- Browser dev tools for Angular-specific debugging

## Integration Points

The system is designed as a cohesive blockchain-secured NDA contract management platform. When working on features, consider the complete business flow:

**Client Side**: User registration → Contract creation with confidential data → Unique key generation → Contract sharing (direct or via links) → Monitoring partner signatures

**Partner Side**: Receive contract invitation → Review NDA terms → "Sign" via blockchain transaction → Access confidential content → Ongoing compliance tracking

**System Integration**: Every interaction creates immutable audit trails, ensuring legal compliance and providing cryptographic proof of confidentiality agreements.

### General Guidelines for Copilot
- **Language**: Use TypeScript for Angular frontend, Rust for backend
- **Frameworks**: Follow Angular 20 standalone component patterns, Axum for Rust backend
- **Database**: Use DBeaver to inspect and manage the SQLite database.
- **Error Handling**: Implement robust error handling with user-friendly messages
- **Code Style**: Follow existing code style and conventions in both frontend and backend
- **Comments**: Add clear comments and documentation for all new code
- **Terminal Commands Summary**: Use powershell syntax for terminal commands.
- **Testing**: Write unit tests for new features and ensure existing tests pass
- **Security**: Ensure all sensitive data is encrypted and access is properly controlled
- **Version Control**: Commit changes with clear messages and follow branching strategy. Use commit patterns like `feat:`, `fix:`, `docs:`, etc.
- **Angular**: Use newer controls like signals, standalone components, @if and @for. Avoid NgModules and CommonModule.
