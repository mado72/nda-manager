# NDA Backend - API Reference

## Overview

The NDA Backend provides a RESTful API for managing blockchain-secured Non-Disclosure Agreement (NDA) contracts. The system enables secure sharing of confidential contracts where clients create NDAs with sensitive information, generate unique contract keys, and allow partners to digitally "sign" confidentiality terms through blockchain verification.

**Base URL**: `http://localhost:3000`

**Content-Type**: `application/json` for all requests and responses

**Authentication**: Currently username-based (password verification implemented for future JWT integration)

---

## Table of Contents

1. [Health Check](#health-check)
2. [User Management](#user-management)
   - [Register User](#register-user)
   - [Login User](#login-user)
   - [Auto Login](#auto-login)
3. [Process Management](#process-management)
   - [Create Process](#create-process)
   - [List Processes](#list-processes)
4. [Sharing & Access](#sharing--access)
   - [Share Process](#share-process)
   - [Access Process](#access-process)
5. [Audit & Compliance](#audit--compliance)
   - [Get Notifications](#get-notifications)
6. [Error Codes](#error-codes)
7. [Security Model](#security-model)

---

## Health Check

### GET /health

Simple health check endpoint to verify service availability.

**Request**: No parameters required

**Response**: `200 OK`

```json
{
  "status": "OK",
  "timestamp": "2024-01-01T00:00:00Z"
}
```

**Use Cases**:
- Load balancer health checks
- Deployment verification
- Monitoring systems

---

## User Management

### Register User

**POST** `/api/users/register`

Registers a new user with automatic Stellar blockchain account creation. Each user gets a unique Stellar keypair and testnet funding.

#### Request Body

```json
{
  "username": "client_company",
  "name": "Client Company Inc.",
  "password": "secure_password_123",
  "roles": ["client"]
}
```

**Fields**:
- `username` (string, required): Unique username, alphanumeric
- `name` (string, required): Full name or display name
- `password` (string, required): User password (bcrypt hashed)
- `roles` (array, required): One or more roles: `["client"]`, `["partner"]`, or `["client", "partner"]`

**Role Types**:
- `"client"`: Can create and manage NDA processes
- `"partner"`: Can access shared processes
- Hybrid: Both roles for full functionality

#### Response: `200 OK`

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "client_company",
  "name": "Client Company Inc.",
  "stellar_public_key": "GCKFBEIYTKP7WM6T4ZVPVQFKGMFB2VL3F4GLC2PEKXJNVXHCJGP6ZVGP",
  "roles": ["client"],
  "created_at": "2024-01-01T00:00:00Z"
}
```

**Note**: `stellar_secret_key` and `password_hash` are NOT included in the response for security.

#### Error Responses

- `409 Conflict`: Username already exists
- `500 Internal Server Error`: Stellar account creation or database error

#### Example: Multi-Role User

```json
{
  "username": "hybrid_user",
  "name": "Hybrid Business Solutions",
  "password": "another_secure_pass",
  "roles": ["client", "partner"]
}
```

---

### Login User

**POST** `/api/users/login`

Authenticates an existing user with username and password verification.

#### Request Body

```json
{
  "username": "client_company",
  "password": "secure_password_123"
}
```

**Fields**:
- `username` (string, required): User's username
- `password` (string, required): User's password

#### Response: `200 OK`

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "client_company",
  "name": "Client Company Inc.",
  "stellar_public_key": "GCKFBEIYTKP7WM6T4ZVPVQFKGMFB2VL3F4GLC2PEKXJNVXHCJGP6ZVGP",
  "roles": ["client"],
  "created_at": "2024-01-01T00:00:00Z"
}
```

#### Error Responses

- `401 Unauthorized`: Invalid username or password
- `500 Internal Server Error`: Database or password verification error

---

### Auto Login

**POST** `/api/users/auto-login`

Performs automatic login using localStorage information (user_name and user_id) without requiring password re-entry.

#### Request Body

```json
{
  "user_name": "client_company",
  "user_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Fields**:
- `user_name` (string, required): Username from localStorage
- `user_id` (string, required): User ID from localStorage

#### Response: `200 OK`

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "client_company",
  "name": "Client Company Inc.",
  "stellar_public_key": "GCKFBEIYTKP7WM6T4ZVPVQFKGMFB2VL3F4GLC2PEKXJNVXHCJGP6ZVGP",
  "roles": ["client"],
  "created_at": "2024-01-01T00:00:00Z"
}
```

#### Error Responses

- `401 Unauthorized`: User not found or username/ID mismatch
- `500 Internal Server Error`: Database error

**Security Note**: This endpoint does not require password verification and should be used only in trusted environments. Consider implementing additional security measures for production use.

---

## Process Management

### Create Process

**POST** `/api/processes`

Creates a new NDA process with AES-256-GCM encrypted confidential content. Requires client role.

#### Request Body

```json
{
  "client_id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "Software Development NDA",
  "description": "Confidential software project details",
  "confidential_content": "Sensitive technical specifications, trade secrets, and proprietary algorithms..."
}
```

**Fields**:
- `client_id` (string, required): UUID of the client creating the process
- `title` (string, required): Process title/name
- `description` (string, required): Detailed process description
- `confidential_content` (string, required): Sensitive content to be encrypted

#### Response: `200 OK`

```json
{
  "id": "650e8400-e29b-41d4-a716-446655440001",
  "client_id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "Software Development NDA",
  "description": "Confidential software project details",
  "status": "active",
  "created_at": "2024-01-01T00:00:00Z"
}
```

**Note**: Encrypted content and encryption key are NOT returned for security reasons.

#### Error Responses

- `403 Forbidden`: User doesn't have client role
- `422 Unprocessable Entity`: Client ID not found
- `500 Internal Server Error`: Encryption or database error

#### Security Features

- Content encrypted with AES-256-GCM
- Unique encryption key per process
- Encryption keys stored separately
- Only owner and shared partners can decrypt

---

### List Processes

**GET** `/api/processes?client_id={client_id}`

Retrieves all NDA processes owned by a specific client, ordered by creation date (newest first).

#### Query Parameters

- `client_id` (string, required): UUID of the client

#### Example Request

```
GET /api/processes?client_id=550e8400-e29b-41d4-a716-446655440000
```

#### Response: `200 OK`

```json
[
  {
    "id": "650e8400-e29b-41d4-a716-446655440001",
    "client_id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Software Development NDA",
    "description": "Confidential software project details",
    "status": "active",
    "created_at": "2024-01-01T00:00:00Z"
  },
  {
    "id": "650e8400-e29b-41d4-a716-446655440002",
    "client_id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Marketing Partnership NDA",
    "description": "Joint marketing campaign specifications",
    "status": "active",
    "created_at": "2023-12-15T00:00:00Z"
  }
]
```

#### Error Responses

- `400 Bad Request`: Missing client_id parameter
- `404 Not Found`: Client ID not found
- `500 Internal Server Error`: Database error

---

## Sharing & Access

### Share Process

**POST** `/api/processes/share`

Shares a process with a partner via Stellar blockchain transaction. Creates an immutable record of sharing on the blockchain.

#### Request Body

```json
{
  "client_username": "client_company",
  "process_id": "650e8400-e29b-41d4-a716-446655440001",
  "partner_public_key": "GDKJP6RVN7MJQ3NSNXHS5LMYR4YVZLHQJCPX2KXMGHFR2YVDTMPBTXYZ"
}
```

**Fields**:
- `client_username` (string, required): Username of the process owner
- `process_id` (string, required): UUID of the process to share
- `partner_public_key` (string, required): Stellar public key of the partner

#### Response: `200 OK`

```json
{
  "id": "750e8400-e29b-41d4-a716-446655440003",
  "process_id": "650e8400-e29b-41d4-a716-446655440001",
  "partner_public_key": "GDKJP6RVN7MJQ3NSNXHS5LMYR4YVZLHQJCPX2KXMGHFR2YVDTMPBTXYZ",
  "stellar_transaction_hash": "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6",
  "shared_at": "2024-01-01T00:00:00Z"
}
```

#### Error Responses

- `404 Not Found`: Process or client not found
- `500 Internal Server Error`: Blockchain transaction or database error

#### Blockchain Integration

- Creates Stellar transaction with metadata
- Transaction hash provides immutable proof
- Memo field contains process ID
- Recorded on Stellar testnet (development)

#### Compliance Benefits

- Immutable audit trail
- Cryptographically verifiable permissions
- Dispute resolution via blockchain evidence
- Regulatory compliance ready

---

### Access Process

**POST** `/api/processes/access`

Allows partners to access and decrypt shared process content. Verifies sharing record before granting access and logs the event for audit trails. Requires partner role.

#### Request Body

```json
{
  "process_id": "650e8400-e29b-41d4-a716-446655440001",
  "partner_username": "partner_company",
  "partner_public_key": "GDKJP6RVN7MJQ3NSNXHS5LMYR4YVZLHQJCPX2KXMGHFR2YVDTMPBTXYZ"
}
```

**Fields**:
- `process_id` (string, required): UUID of the process to access
- `partner_username` (string, required): Username of the accessing partner
- `partner_public_key` (string, required): Partner's Stellar public key

#### Response: `200 OK`

```json
{
  "process_id": "650e8400-e29b-41d4-a716-446655440001",
  "title": "Software Development NDA",
  "description": "Confidential software project details",
  "content": "Decrypted confidential content with sensitive technical specifications...",
  "accessed_at": "2024-01-01T00:00:00Z"
}
```

#### Error Responses

- `403 Forbidden`: Process not shared with this partner OR user doesn't have partner role
- `404 Not Found`: Process or partner not found
- `500 Internal Server Error`: Decryption or database error

#### Access Control Flow

1. Verify process exists
2. Verify partner exists
3. Check partner has partner role
4. Verify sharing record in database
5. Decrypt content
6. Log access event
7. Return decrypted content

#### Security Notes

- Content decrypted in memory only
- Every access logged for compliance
- Failed attempts also logged
- Sharing verification prevents unauthorized access

---

## Audit & Compliance

### Get Notifications

**GET** `/api/notifications?client_id={client_id}`

Retrieves access notifications for a client's processes with comprehensive details including process descriptions and status.

#### Query Parameters

- `client_id` (string, required): UUID of the client

#### Example Request

```
GET /api/notifications?client_id=550e8400-e29b-41d4-a716-446655440000
```

#### Response: `200 OK`

```json
[
  {
    "id": "850e8400-e29b-41d4-a716-446655440004",
    "process_id": "650e8400-e29b-41d4-a716-446655440001",
    "partner_id": "950e8400-e29b-41d4-a716-446655440005",
    "accessed_at": "2024-01-01T10:30:00Z",
    "process_title": "Software Development NDA",
    "process_description": "Comprehensive confidentiality agreement for software development partnership",
    "process_status": "active",
    "partner_username": "partner_company"
  },
  {
    "id": "850e8400-e29b-41d4-a716-446655440006",
    "process_id": "650e8400-e29b-41d4-a716-446655440002",
    "partner_id": "950e8400-e29b-41d4-a716-446655440007",
    "accessed_at": "2024-01-01T09:15:00Z",
    "process_title": "Marketing Partnership NDA",
    "process_description": "Non-disclosure agreement for marketing collaboration and data sharing",
    "process_status": "completed",
    "partner_username": "another_partner"
  }
]
```

**Field Descriptions**:
- `id`: Access record UUID (nullable if no access yet)
- `process_id`: Process UUID
- `partner_id`: Partner UUID (nullable if no access yet)
- `accessed_at`: Timestamp of access (nullable if no access yet)
- `process_title`: Process title
- `process_description`: Detailed process description
- `process_status`: Current process status
- `partner_username`: Partner's username (nullable if user deleted)

#### Error Responses

- `400 Bad Request`: Missing client_id parameter
- `404 Not Found`: Client ID not found
- `500 Internal Server Error`: Database error

#### Use Cases

- **Enhanced Compliance Reporting**: Generate detailed audit reports with full context
- **Status-Aware Analytics**: Track access patterns with current process status
- **Security Monitoring**: Monitor suspicious access with process details
- **Rich Client Dashboard**: Display comprehensive notifications with context

---

## Error Codes

The API follows standard HTTP status codes:

| Code | Description | Common Causes |
|------|-------------|---------------|
| `200` | OK | Successful operation |
| `400` | Bad Request | Missing parameters, invalid JSON format |
| `401` | Unauthorized | Invalid credentials |
| `403` | Forbidden | Insufficient permissions or role requirements not met |
| `404` | Not Found | Resource not found (user, process, etc.) |
| `409` | Conflict | Resource already exists (e.g., username) |
| `422` | Unprocessable Entity | Valid request but cannot be processed |
| `500` | Internal Server Error | Database, encryption, or blockchain errors |

### Error Response Format

All errors return a plain text message or status code. Future versions may include structured error responses:

```json
{
  "error": "Resource not found",
  "message": "Process with ID xyz not found",
  "code": "PROCESS_NOT_FOUND"
}
```

---

## Security Model

### Multi-Layer Security Architecture

1. **Role-Based Access Control (RBAC)**
   - Endpoints verify user roles before operations
   - Clients can only manage their own processes
   - Partners can only access explicitly shared processes

2. **End-to-End Encryption**
   - AES-256-GCM for all confidential content
   - Unique encryption key per process
   - Hardware-accelerated encryption when available

3. **Blockchain Verification**
   - Stellar network integration for sharing records
   - Immutable transaction hashes as proof
   - Cryptographically verifiable permissions

4. **Password Security**
   - Bcrypt hashing with salt for password storage
   - Future: JWT tokens for session management
   - Consider implementing rate limiting

5. **Audit Trail**
   - Complete access logging
   - Timestamps for all operations
   - Regulatory compliance ready

### Best Practices for Clients

1. **Secure Storage**: Never expose `stellar_secret_key` or `encryption_key` in client applications
2. **HTTPS Only**: Always use HTTPS in production environments
3. **Token Management**: Implement JWT tokens for session management (future enhancement)
4. **Rate Limiting**: Consider implementing rate limiting for production deployments
5. **Input Validation**: Validate all user inputs before sending to API
6. **Error Handling**: Handle all error responses gracefully

### Production Deployment Checklist

- [ ] Configure proper CORS settings (currently permissive)
- [ ] Enable HTTPS/TLS encryption
- [ ] Implement API rate limiting
- [ ] Use Key Management Service (KMS) for encryption keys
- [ ] Switch to Stellar mainnet
- [ ] Enable comprehensive logging
- [ ] Set up monitoring and alerting
- [ ] Implement JWT-based authentication
- [ ] Regular security audits

---

## Technology Stack

- **Web Framework**: Axum (async HTTP server)
- **Database**: SQLite with SQLx (type-safe queries)
- **Blockchain**: Stellar network (testnet for development)
- **Encryption**: AES-256-GCM with ring crate
- **Authentication**: Bcrypt password hashing
- **Logging**: Tracing for structured logging
- **Runtime**: Tokio for async operations

---

## Complete Request Flow Example

### Scenario: Client shares confidential content with partner

```bash
# 1. Register client user
POST /api/users/register
{
  "username": "acme_corp",
  "name": "ACME Corporation",
  "password": "secure_pass_123",
  "roles": ["client"]
}
# Response: user_id, stellar_public_key

# 2. Register partner user
POST /api/users/register
{
  "username": "beta_solutions",
  "name": "Beta Solutions Inc.",
  "password": "another_pass_456",
  "roles": ["partner"]
}
# Response: partner_id, stellar_public_key

# 3. Client creates encrypted process
POST /api/processes
{
  "client_id": "550e8400-...",
  "title": "Project Alpha NDA",
  "description": "Confidential project specifications",
  "confidential_content": "Secret algorithms and formulas..."
}
# Response: process_id

# 4. Client shares via blockchain
POST /api/processes/share
{
  "client_username": "acme_corp",
  "process_id": "650e8400-...",
  "partner_public_key": "GDKJP6RVN7..."
}
# Response: stellar_transaction_hash (immutable proof)

# 5. Partner accesses content
POST /api/processes/access
{
  "process_id": "650e8400-...",
  "partner_username": "beta_solutions",
  "partner_public_key": "GDKJP6RVN7..."
}
# Response: decrypted content

# 6. Client views audit trail
GET /api/notifications?client_id=550e8400-...
# Response: list of access events with details
```

---

## Additional Resources

- **Source Code**: Located in `nda-backend/src/`
- **Database Migrations**: `nda-backend/migrations/`
- **Models Documentation**: See `models.rs` for detailed type definitions
- **Handler Documentation**: See `handlers.rs` for implementation details
- **Stellar Integration**: See `stellar_real.rs` for blockchain operations

---

## Support & Contribution

For questions, issues, or contributions, please refer to the project README and contribution guidelines.

**Version**: 1.0.0  
**Last Updated**: November 2025
