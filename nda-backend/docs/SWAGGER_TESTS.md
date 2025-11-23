# Swagger Integration Test Script

## Quick Verification Tests

Run these commands to verify the Swagger integration is working correctly.

### Prerequisites

Ensure the server is running:
```powershell
cd C:\@Desenv\nda\code\nda-backend
cargo run
```

Wait for the startup message:
```
🚀 Server running at http://localhost:3000
📖 Swagger UI available at http://localhost:3000/swagger-ui
```

---

## Test 1: Health Check Endpoint

Test the basic health endpoint:

```powershell
Invoke-RestMethod -Uri http://localhost:3000/health | ConvertTo-Json
```

**Expected Output**:
```json
{
  "status": "ok",
  "timestamp": "2024-01-01T12:00:00Z",
  "database": "connected"
}
```

---

## Test 2: OpenAPI Spec Endpoint

Verify the OpenAPI specification is accessible:

```powershell
Invoke-RestMethod -Uri http://localhost:3000/api-docs/openapi.json | ConvertTo-Json -Depth 3
```

**Expected Output**: JSON object with:
- `openapi`: "3.0.3"
- `info`: API title, version, description
- `paths`: All 9 endpoint definitions
- `components`: All 12 schema definitions

---

## Test 3: Swagger UI Access

Open Swagger UI in browser:

```powershell
Start-Process "http://localhost:3000/swagger-ui"
```

**Expected Result**: Browser opens with interactive Swagger UI interface showing:
- NDA Manager API title
- 5 tagged sections (Health Check, User Management, etc.)
- 9 endpoints with expandable documentation

---

## Test 4: Register User via API

Test user registration endpoint:

```powershell
$body = @{
    username = "testuser@example.com"
    password = "SecurePassword123!"
    name = "Test User"
    role = "client"
} | ConvertTo-Json

Invoke-RestMethod -Uri http://localhost:3000/api/users/register `
    -Method Post `
    -ContentType "application/json" `
    -Body $body | ConvertTo-Json
```

**Expected Output**:
```json
{
  "id": "generated-uuid",
  "username": "testuser@example.com",
  "name": "Test User",
  "roles": ["client"],
  "stellar_public_key": "G...",
  "stellar_secret_key": "S...",
  "created_at": "2024-01-01T12:00:00Z"
}
```

---

## Test 5: Login User

Test login endpoint:

```powershell
$loginBody = @{
    username = "testuser@example.com"
    password = "SecurePassword123!"
} | ConvertTo-Json

Invoke-RestMethod -Uri http://localhost:3000/api/users/login `
    -Method Post `
    -ContentType "application/json" `
    -Body $loginBody | ConvertTo-Json
```

**Expected Output**: Same user object as registration

---

## Test 6: Create Process

Test process creation (use email from registration):

```powershell
$processBody = @{
    title = "Test NDA Contract"
    description = "Test confidential agreement"
    content = "This is confidential information"
    creator_email = "testuser@example.com"
} | ConvertTo-Json

Invoke-RestMethod -Uri http://localhost:3000/api/processes `
    -Method Post `
    -ContentType "application/json" `
    -Body $processBody | ConvertTo-Json
```

**Expected Output**:
```json
{
  "id": "generated-uuid",
  "title": "Test NDA Contract",
  "description": "Test confidential agreement",
  "creator_id": "user-uuid",
  "created_at": "2024-01-01T12:00:00Z",
  "status": "active"
}
```

---

## Test 7: List Processes

Test process listing:

```powershell
Invoke-RestMethod -Uri "http://localhost:3000/api/processes?creator_email=testuser@example.com" | ConvertTo-Json
```

**Expected Output**: Array with the created process

---

## Test 8: Swagger UI Interactive Test

### Using Swagger UI (Browser-Based):

1. Open http://localhost:3000/swagger-ui
2. Find `POST /api/users/register`
3. Click **"Try it out"**
4. Edit the request body:
   ```json
   {
     "username": "swaggertest@example.com",
     "password": "SecurePass456!",
     "name": "Swagger Test User",
     "role": "partner"
   }
   ```
5. Click **"Execute"**
6. Verify response shows:
   - Status: `201 Created`
   - Response body with user details and Stellar keypair

### Test Other Endpoints:

**Health Check**:
- Expand `GET /health`
- Click "Try it out" → "Execute"
- Should return 200 OK

**Login**:
- Expand `POST /api/users/login`
- Use credentials from registration
- Execute and verify 200 OK response

**Create Process**:
- Expand `POST /api/processes`
- Fill in process details
- Use registered user's email
- Execute and verify 201 Created

---

## Test 9: OpenAPI Spec Validation

Validate the OpenAPI specification structure:

```powershell
$spec = Invoke-RestMethod -Uri http://localhost:3000/api-docs/openapi.json

# Check version
Write-Host "OpenAPI Version: $($spec.openapi)"

# Count endpoints
Write-Host "Total Paths: $($spec.paths.PSObject.Properties.Count)"

# Count schemas
Write-Host "Total Schemas: $($spec.components.schemas.PSObject.Properties.Count)"

# List all endpoints
Write-Host "`nEndpoints:"
$spec.paths.PSObject.Properties | ForEach-Object {
    $path = $_.Name
    $methods = $_.Value.PSObject.Properties.Name
    Write-Host "  $path : $($methods -join ', ')"
}

# List all schemas
Write-Host "`nSchemas:"
$spec.components.schemas.PSObject.Properties.Name | ForEach-Object {
    Write-Host "  - $_"
}
```

**Expected Output**:
```
OpenAPI Version: 3.0.3
Total Paths: 9
Total Schemas: 13

Endpoints:
  /health : get
  /api/users/register : post
  /api/users/login : post
  /api/users/auto-login : post
  /api/processes : post, get
  /api/share : post
  /api/access : post
  /api/notifications : get

Schemas:
  - RegisterRequest
  - LoginRequest
  - AutoLoginRequest
  - CreateProcessRequest
  - ShareProcessRequest
  - AccessProcessRequest
  - UserResponse
  - ProcessResponse
  - ProcessAccessResponse
  - HealthResponse
  - ProcessShare
  - ProcessAccessWithDetails
  - ListProcessesQuery
```

---

## Test 10: Export and Import to Postman

### Export OpenAPI Spec:

```powershell
Invoke-RestMethod -Uri http://localhost:3000/api-docs/openapi.json | 
    ConvertTo-Json -Depth 10 | 
    Out-File -Encoding UTF8 "C:\@Desenv\nda\code\nda-backend\openapi.json"

Write-Host "OpenAPI spec saved to: C:\@Desenv\nda\code\nda-backend\openapi.json"
```

### Import to Postman:

1. Open Postman
2. Click **"Import"** button
3. Select **"Link"** tab
4. Enter: `http://localhost:3000/api-docs/openapi.json`
5. Click **"Continue"**
6. Review the collection and click **"Import"**
7. All 9 endpoints should be imported with schemas

### Verify in Postman:

- Check "NDA Manager API" collection is created
- Verify all folders: Health Check, User Management, etc.
- Test an endpoint (e.g., Health Check)
- Verify request body schemas are populated

---

## Troubleshooting

### Issue: Server not responding

**Solution**:
```powershell
# Check if server is running
Get-Process | Where-Object {$_.ProcessName -like "*nda-backend*"}

# Restart server
cd C:\@Desenv\nda\code\nda-backend
cargo run
```

### Issue: Swagger UI shows blank page

**Solutions**:
1. Clear browser cache (Ctrl + Shift + Delete)
2. Try incognito/private browsing mode
3. Check browser console for JavaScript errors
4. Verify OpenAPI JSON is accessible:
   ```powershell
   Invoke-RestMethod -Uri http://localhost:3000/api-docs/openapi.json
   ```

### Issue: CORS errors in browser

**Solution**: CORS is already enabled for development. If issues persist:
```rust
// Already configured in src/main.rs
let cors = CorsLayer::permissive();
```

### Issue: Authentication errors

**Solution**: The API currently uses email-based authentication. Ensure:
1. User is registered first
2. Correct email is used in subsequent requests
3. Email matches exactly (case-sensitive)

---

## Complete Integration Test Workflow

Run this complete workflow to test the entire system:

```powershell
# 1. Health Check
Write-Host "1. Testing Health Check..." -ForegroundColor Cyan
$health = Invoke-RestMethod -Uri http://localhost:3000/health
Write-Host "Status: $($health.status)" -ForegroundColor Green

# 2. Register Client
Write-Host "`n2. Registering Client..." -ForegroundColor Cyan
$clientBody = @{
    username = "client@test.com"
    password = "ClientPass123!"
    name = "Test Client"
    role = "client"
} | ConvertTo-Json

$client = Invoke-RestMethod -Uri http://localhost:3000/api/users/register `
    -Method Post -ContentType "application/json" -Body $clientBody
Write-Host "Client ID: $($client.id)" -ForegroundColor Green

# 3. Register Partner
Write-Host "`n3. Registering Partner..." -ForegroundColor Cyan
$partnerBody = @{
    username = "partner@test.com"
    password = "PartnerPass123!"
    name = "Test Partner"
    role = "partner"
} | ConvertTo-Json

$partner = Invoke-RestMethod -Uri http://localhost:3000/api/users/register `
    -Method Post -ContentType "application/json" -Body $partnerBody
Write-Host "Partner ID: $($partner.id)" -ForegroundColor Green

# 4. Create Process
Write-Host "`n4. Creating Process..." -ForegroundColor Cyan
$processBody = @{
    title = "Integration Test NDA"
    description = "Testing complete workflow"
    content = "Confidential test content"
    creator_email = "client@test.com"
} | ConvertTo-Json

$process = Invoke-RestMethod -Uri http://localhost:3000/api/processes `
    -Method Post -ContentType "application/json" -Body $processBody
Write-Host "Process ID: $($process.id)" -ForegroundColor Green

# 5. List Processes
Write-Host "`n5. Listing Processes..." -ForegroundColor Cyan
$processes = Invoke-RestMethod -Uri "http://localhost:3000/api/processes?creator_email=client@test.com"
Write-Host "Total Processes: $($processes.Count)" -ForegroundColor Green

Write-Host "`n✅ All tests completed successfully!" -ForegroundColor Green
```

---

## Success Criteria

✅ All tests should pass with expected outputs  
✅ Swagger UI should be accessible and interactive  
✅ OpenAPI spec should be valid and complete  
✅ All 9 endpoints should be documented  
✅ All 12 schemas should be available  
✅ Postman import should work without errors  

---

## Next Steps After Verification

1. **Test with Frontend**: Integrate with Angular application
2. **Security Testing**: Test authentication and authorization
3. **Performance Testing**: Load test with multiple requests
4. **Documentation**: Add more examples and use cases
5. **Production**: Deploy with production configurations

---

**Last Updated**: January 2024  
**Status**: Ready for Testing  
**Version**: 1.0.0
