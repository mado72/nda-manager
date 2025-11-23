# Swagger/OpenAPI Integration Guide

## Overview

The NDA Manager backend now includes **interactive API documentation** using Swagger UI, powered by the `utoipa` crate. This provides a web-based interface to explore, test, and understand all API endpoints.

## Accessing Swagger UI

Once the server is running, access the interactive documentation at:

```
http://localhost:3000/swagger-ui
```

The OpenAPI specification (JSON format) is available at:

```
http://localhost:3000/api-docs/openapi.json
```

## Quick Start

### 1. Start the Server

```bash
cd nda-backend
cargo run
```

You should see:

```
🚀 Server running at http://localhost:3000
📊 Health check available at http://localhost:3000/health
📖 Swagger UI available at http://localhost:3000/swagger-ui
📄 OpenAPI spec at http://localhost:3000/api-docs/openapi.json
🔐 Security: AES-256-GCM encryption + Stellar blockchain integration
```

### 2. Open Swagger UI

Navigate to `http://localhost:3000/swagger-ui` in your browser.

### 3. Explore the API

The Swagger UI provides:
- **Complete endpoint documentation** with request/response schemas
- **Interactive testing** - try out API calls directly from the browser
- **Request examples** with proper JSON formatting
- **Response codes** and error handling documentation
- **Security requirements** for each endpoint

## Available API Endpoints

### Health & Monitoring
- `GET /health` - System health check

### User Management
- `POST /api/users/register` - Register new user
- `POST /api/users/login` - User login with credentials
- `POST /api/users/auto-login` - Auto-login with Stellar keypair

### Process Management
- `POST /api/processes` - Create new NDA contract/process
- `GET /api/processes` - List user's processes (with optional filters)

### Sharing & Access
- `POST /api/share` - Share process with partner (blockchain signature)
- `POST /api/access` - Access shared process content

### Audit & Compliance
- `GET /api/notifications` - Get user's process access notifications

## Using Swagger UI

### Testing Endpoints

1. **Expand an endpoint** by clicking on it
2. **Click "Try it out"** button
3. **Fill in the required parameters** (request body, query params, etc.)
4. **Click "Execute"** to send the request
5. **View the response** including status code, headers, and body

### Example: Testing Health Endpoint

1. Navigate to `http://localhost:3000/swagger-ui`
2. Find the `GET /health` endpoint
3. Click "Try it out"
4. Click "Execute"
5. You should see a 200 response with:
   ```json
   {
     "status": "ok",
     "timestamp": "2024-01-01T12:00:00Z",
     "database": "connected"
   }
   ```

### Example: Registering a User

1. Find the `POST /api/users/register` endpoint
2. Click "Try it out"
3. Edit the request body:
   ```json
   {
     "username": "testuser@example.com",
     "password": "SecurePassword123!",
     "name": "Test User",
     "role": "client"
   }
   ```
4. Click "Execute"
5. Check the response for the created user with Stellar keypair

### Example: Creating a Process

1. First, register and login to get authentication
2. Find the `POST /api/processes` endpoint
3. Fill in the process details:
   ```json
   {
     "title": "Confidential Project NDA",
     "description": "NDA for Project X development",
     "content": "This is the confidential information...",
     "creator_email": "client@example.com"
   }
   ```
4. Execute and note the returned `process_id`

## API Organization

The API is organized into logical groups:

- **Health Check** - System status
- **User Management** - Authentication and user operations
- **Process Management** - Contract creation and listing
- **Sharing & Access** - Partner collaboration with blockchain
- **Audit & Compliance** - Access tracking and notifications

## Security Testing

### Authentication Flow

Most endpoints require authentication. Test the flow:

1. **Register** a new user → Get user details
2. **Login** with credentials → Get session token (if implemented)
3. **Create Process** using authenticated user email
4. **Share Process** to establish blockchain trust bond
5. **Access Process** with partner authentication

### Blockchain Integration Testing

Test the Stellar blockchain integration:

1. Register a client user → Receives Stellar keypair
2. Register a partner user → Receives Stellar keypair
3. Create a process as client
4. Share process with partner → Creates blockchain transaction
5. Partner accesses process → Verified via blockchain signature

## OpenAPI Specification

### Exporting the Spec

Download the OpenAPI JSON specification:

```bash
curl http://localhost:3000/api-docs/openapi.json > openapi.json
```

### Using with Other Tools

The OpenAPI spec can be imported into:

- **Postman** - Import collection from OpenAPI spec
- **Insomnia** - Import OpenAPI document
- **API Gateway** - Import for deployment configuration
- **Code Generators** - Generate client SDKs in various languages

Example with Postman:
1. Open Postman
2. Click "Import"
3. Enter URL: `http://localhost:3000/api-docs/openapi.json`
4. Postman will create a complete collection with all endpoints

## Implementation Details

### Technology Stack

- **utoipa 4.0** - OpenAPI spec generation with Rust macros
- **utoipa-swagger-ui 6.0** - Embedded Swagger UI web interface
- **Axum integration** - Seamless route integration

### Code Structure

All API documentation is generated from code annotations:

```rust
// Models with schema definitions
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub name: String,
    pub role: String,
}

// Handlers with OpenAPI metadata
#[utoipa::path(
    post,
    path = "/api/users/register",
    tag = "User Management",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn register_user(/* ... */) { /* ... */ }
```

### Customization

The OpenAPI spec is configured in `src/main.rs`:

```rust
#[derive(OpenApi)]
#[openapi(
    info(
        title = "NDA Manager API",
        version = "1.0.0",
        description = "Blockchain-secured NDA contract management system"
    ),
    paths(
        health_check,
        register_user,
        login_user,
        // ... more endpoints
    ),
    components(
        schemas(RegisterRequest, UserResponse, /* ... */)
    ),
    tags(
        (name = "Health Check", description = "System health monitoring"),
        (name = "User Management", description = "User registration and authentication"),
        // ... more tags
    )
)]
struct ApiDoc;
```

## Troubleshooting

### Swagger UI Not Loading

**Problem**: Swagger UI page is blank or not loading

**Solutions**:
1. Verify the server is running: `curl http://localhost:3000/health`
2. Check for JavaScript errors in browser console
3. Try accessing OpenAPI spec directly: `http://localhost:3000/api-docs/openapi.json`
4. Clear browser cache and reload

### OpenAPI Spec Shows No Endpoints

**Problem**: Swagger UI loads but shows empty API

**Solutions**:
1. Verify all handlers are included in `ApiDoc` paths
2. Check that handlers have `#[utoipa::path(...)]` annotations
3. Rebuild the project: `cargo clean && cargo build`

### CORS Issues When Testing from Frontend

**Problem**: Requests fail with CORS errors

**Solutions**:
1. CORS is already enabled for development
2. For production, update CORS settings in `src/main.rs`
3. Check that frontend is making requests to correct origin

### Authentication Required Errors

**Problem**: Many endpoints return 401 Unauthorized

**Solution**: This is expected behavior. Follow the authentication flow:
1. Register a user first
2. Use the returned email in subsequent requests
3. Implement proper session management in frontend

## Production Considerations

### Security

- [ ] Remove overly permissive CORS settings
- [ ] Add rate limiting to prevent abuse
- [ ] Implement proper authentication tokens (JWT)
- [ ] Enable HTTPS/TLS
- [ ] Add request validation middleware

### Performance

- [ ] Cache OpenAPI spec generation
- [ ] Consider serving Swagger UI as static files
- [ ] Implement connection pooling
- [ ] Add request logging and monitoring

### Documentation

- [ ] Add more detailed descriptions to endpoints
- [ ] Include authentication flow diagrams
- [ ] Document error codes comprehensively
- [ ] Add usage examples for complex workflows

## Additional Resources

- [utoipa Documentation](https://docs.rs/utoipa/)
- [OpenAPI Specification](https://swagger.io/specification/)
- [Swagger UI Documentation](https://swagger.io/tools/swagger-ui/)
- [Axum Documentation](https://docs.rs/axum/)

## Support

For issues or questions:
1. Check existing documentation in `/docs`
2. Review API examples in Swagger UI
3. Consult the codebase comments
4. Review database schema in `docs/ARCHITECTURE.md`

---

**Last Updated**: January 2024  
**Version**: 1.0.0  
**Status**: ✅ Fully Operational
