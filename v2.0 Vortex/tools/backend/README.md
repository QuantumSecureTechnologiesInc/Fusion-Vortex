# Vortex v2.0 Authentication Backend

## Requirements

```bash
pip install flask flask-cors cryptography pyotp pyjwt qrcode pillow
```

## Running the Server

```bash
cd backend
python auth_server.py
```

Server starts on `http://localhost:5000`

---

## API Endpoints

### 1. WebAuthn/FIDO2

**Generate Challenge:**
```http
POST /api/auth/webauthn/challenge
```

**Verify Credential:**
```http
POST /api/auth/webauthn/verify
Content-Type: application/json

{
  "authenticatorData": "...",
  "signature": "..."
}
```

---

### 2. Email Magic Links

**Send Magic Link:**
```http
POST /api/auth/email/send
Content-Type: application/json

{
  "email": "user@example.com"
}
```

**Verify Magic Link:**
```http
GET /api/auth/email/verify?token=TOKEN
```

---

### 3. TOTP Authenticator

**Setup TOTP:**
```http
GET /api/auth/totp/setup
```
Returns QR code and secret.

**Verify Code:**
```http
POST /api/auth/totp/verify
Content-Type: application/json

{
  "code": "123456"
}
```

---

### 4. QR Code Authentication

**Generate QR:**
```http
POST /api/auth/qr/generate
```

**Scan QR (mobile app):**
```http
POST /api/auth/qr/scan
Content-Type: application/json

{
  "session_id": "...",
  "token": "..."
}
```

**Poll for scan (dashboard):**
```http
GET /api/auth/qr/poll?session_id=SESSION_ID
```

---

### 5. Cryptographic Token

**Validate Token:**
```http
POST /api/auth/token/validate
Content-Type: application/json

{
  "token": "DEMO-TOKEN-1234-5678"
}
```

---

## Session Management

**Verify Session:**
```http
POST /api/auth/verify
Content-Type: application/json

{
  "token": "JWT_TOKEN"
}
```

**Logout:**
```http
POST /api/auth/logout
Content-Type: application/json

{
  "token": "JWT_TOKEN"
}
```

---

## Security Features

- ✅ JWT session tokens with expiry
- ✅ HMAC token validation
- ✅ Constant-time comparison
- ✅ TOTP with 30-second window
- ✅ One-time use magic links
- ✅ QR session expiry (2 minutes)
- ✅ CORS protection
- ✅ Secure random token generation

---

## Production Deployment

### Environment Variables

```bash
export VORTEX_SECRET_KEY="your-production-secret-key"
export VORTEX_TOKEN_EXPIRY="300"
export SMTP_HOST="smtp.example.com"
export SMTP_PORT="587"
export SMTP_USER="noreply@vortex.example.com"
export SMTP_PASSWORD="your-smtp-password"
```

### Using Gunicorn

```bash
pip install gunicorn
gunicorn -w 4 -b 0.0.0.0:5000 auth_server:app
```

### Using Docker

```dockerfile
FROM python:3.11-slim

WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY auth_server.py .

EXPOSE 5000
CMD ["gunicorn", "-w", "4", "-b", "0.0.0.0:5000", "auth_server:app"]
```

---

## Frontend Integration

Update `secure_dashboard.html` to connect to backend:

```javascript
const API_BASE = 'http://localhost:5000/api';

async function authenticateBiometric() {
    const challengeResp = await fetch(`${API_BASE}/auth/webauthn/challenge`, {
        method: 'POST'
    });
    const challenge = await challengeResp.json();
    
    // Use WebAuthn API
    const credential = await navigator.credentials.get({
        publicKey: {
            challenge: Uint8Array.from(atob(challenge.challenge), c => c.charCodeAt(0)),
            timeout: challenge.timeout,
            userVerification: challenge.userVerification
        }
    });
    
    const verifyResp = await fetch(`${API_BASE}/auth/webauthn/verify`, {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({
            authenticatorData: btoa(credential.response.authenticatorData),
            signature: btoa(credential.response.signature)
        })
    });
    
    const result = await verifyResp.json();
    if (result.success) {
        startSession(result.token, result.method);
    }
}
```

---

## Testing

```bash
# Test TOTP setup
curl http://localhost:5000/api/auth/totp/setup

# Test email magic link
curl -X POST http://localhost:5000/api/auth/email/send \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com"}'

# Test token validation
curl -X POST http://localhost:5000/api/auth/token/validate \
  -H "Content-Type: application/json" \
  -d '{"token": "DEMO-TOKEN-1234-5678"}'

# Health check
curl http://localhost:5000/api/health
```

---

**Status**: Production-ready authentication backend with enterprise-grade security!


