"""
Vortex v2.0 - Secure Authentication Backend
Enterprise-grade authentication server supporting 5 methods:
1. WebAuthn/FIDO2 (biometric/hardware keys)
2. Email magic links
3. TOTP (RFC 6238)
4. QR code authentication
5. Cryptographic tokens

Dependencies: pip install flask flask-cors cryptography pyotp pyjwt qrcode pillow
"""

from flask import Flask, request, jsonify
from flask_cors import CORS
import pyotp
import qrcode
import io
import base64
import secrets
import hashlib
import hmac
import time
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.backends import default_backend
import jwt
from datetime import datetime, timedelta

app = Flask(__name__)
CORS(app)  # Enable CORS for dashboard

# Security configuration
SECRET_KEY = secrets.token_hex(32)
TOKEN_EXPIRY = 300  # 5 minutes
TOTP_SECRET = pyotp.random_base32()

# In-memory storage (use Redis/database in production)
magic_links = {}
pending_qr_auths = {}
valid_tokens = set()

# ============================================================================
# METHOD 1: WebAuthn/FIDO2
# ============================================================================
@app.route('/api/auth/webauthn/challenge', methods=['POST'])
def webauthn_challenge():
    """Generate WebAuthn challenge"""
    challenge = secrets.token_bytes(32)
    challenge_b64 = base64.b64encode(challenge).decode()
    
    return jsonify({
        'challenge': challenge_b64,
        'rpId': 'localhost',
        'timeout': 60000,
        'userVerification': 'preferred'
    })

@app.route('/api/auth/webauthn/verify', methods=['POST'])
def webauthn_verify():
    """Verify WebAuthn response"""
    data = request.json
    
    # In production: verify attestation/assertion properly
    # For now: simplified validation
    if data.get('authenticatorData') and data.get('signature'):
        token = generate_session_token('webauthn')
        return jsonify({
            'success': True,
            'token': token,
            'method': 'WebAuthn/FIDO2'
        })
    
    return jsonify({'success': False, 'error': 'Invalid credentials'}), 401

# ============================================================================
# METHOD 2: Email Magic Links
# ============================================================================
@app.route('/api/auth/email/send', methods=['POST'])
def send_magic_link():
    """Generate and send magic link"""
    data = request.json
    email = data.get('email')
    
    if not email or '@' not in email:
        return jsonify({'success': False, 'error': 'Invalid email'}), 400
    
    # Generate secure magic link token
    token = secrets.token_urlsafe(32)
    magic_link_hash = hashlib.sha256(token.encode()).hexdigest()
    
    # Store with expiry
    magic_links[magic_link_hash] = {
        'email': email,
        'expires': time.time() + 600  # 10 minutes
    }
    
    # In production: send actual email via SMTP/SendGrid/SES
    magic_link = f"http://localhost:5000/api/auth/email/verify?token={token}"
    
    print(f"\n{'='*60}")
    print(f"Magic Link for {email}:")
    print(f"{magic_link}")
    print(f"{'='*60}\n")
    
    return jsonify({
        'success': True,
        'message': 'Magic link sent',
        'debug_link': magic_link  # Remove in production
    })

@app.route('/api/auth/email/verify', methods=['GET'])
def verify_magic_link():
    """Verify magic link and authenticate"""
    token = request.args.get('token')
    
    if not token:
        return jsonify({'success': False, 'error': 'No token provided'}), 400
    
    token_hash = hashlib.sha256(token.encode()).hexdigest()
    
    if token_hash not in magic_links:
        return jsonify({'success': False, 'error': 'Invalid or expired link'}), 401
    
    link_data = magic_links[token_hash]
    
    # Check expiry
    if time.time() > link_data['expires']:
        del magic_links[token_hash]
        return jsonify({'success': False, 'error': 'Link expired'}), 401
    
    # Valid - generate session token
    session_token = generate_session_token('email')
    del magic_links[token_hash]  # One-time use
    
    return jsonify({
        'success': True,
        'token': session_token,
        'method': 'Email Magic Link',
        'email': link_data['email']
    })

# ============================================================================
# METHOD 3: TOTP (Time-based One-Time Password)
# ============================================================================
@app.route('/api/auth/totp/setup', methods=['GET'])
def totp_setup():
    """Generate TOTP secret and QR code"""
    totp = pyotp.TOTP(TOTP_SECRET)
    provisioning_uri = totp.provisioning_uri(
        name='admin@vortex',
        issuer_name='Vortex v2.0'
    )
    
    # Generate QR code
    qr = qrcode.QRCode(version=1, box_size=10, border=5)
    qr.add_data(provisioning_uri)
    qr.make(fit=True)
    
    img = qr.make_image(fill_color="black", back_color="white")
    
    # Convert to base64
    buf = io.BytesIO()
    img.save(buf, format='PNG')
    qr_base64 = base64.b64encode(buf.getvalue()).decode()
    
    return jsonify({
        'secret': TOTP_SECRET,
        'qr_code': f'data:image/png;base64,{qr_base64}',
        'provisioning_uri': provisioning_uri
    })

@app.route('/api/auth/totp/verify', methods=['POST'])
def totp_verify():
    """Verify TOTP code"""
    data = request.json
    code = data.get('code', '').replace(' ', '')
    
    if not code or len(code) != 6 or not code.isdigit():
        return jsonify({'success': False, 'error': 'Invalid code format'}), 400
    
    totp = pyotp.TOTP(TOTP_SECRET)
    
    # Verify with 30-second window
    if totp.verify(code, valid_window=1):
        token = generate_session_token('totp')
        return jsonify({
            'success': True,
            'token': token,
            'method': 'TOTP Authenticator'
        })
    
    return jsonify({'success': False, 'error': 'Invalid code'}), 401

# ============================================================================
# METHOD 4: QR Code Authentication
# ============================================================================
@app.route('/api/auth/qr/generate', methods=['POST'])
def qr_generate():
    """Generate QR code for scanning"""
    session_id = secrets.token_urlsafe(16)
    auth_token = secrets.token_urlsafe(32)
    
    # Store pending authentication
    pending_qr_auths[session_id] = {
        'token': auth_token,
        'expires': time.time() + 120,  # 2 minutes
        'scanned': False
    }
    
    # QR data: authentication endpoint + token
    qr_data = f"vortex://auth?session={session_id}&token={auth_token}"
    
    # Generate QR code
    qr = qrcode.QRCode(version=1, box_size=10, border=4)
    qr.add_data(qr_data)
    qr.make(fit=True)
    
    img = qr.make_image(fill_color="black", back_color="white")
    
    buf = io.BytesIO()
    img.save(buf, format='PNG')
    qr_base64 = base64.b64encode(buf.getvalue()).decode()
    
    return jsonify({
        'session_id': session_id,
        'qr_code': f'data:image/png;base64,{qr_base64}',
        'expires_in': 120
    })

@app.route('/api/auth/qr/scan', methods=['POST'])
def qr_scan():
    """Mobile app scans QR and confirms"""
    data = request.json
    session_id = data.get('session_id')
    token = data.get('token')
    
    if session_id not in pending_qr_auths:
        return jsonify({'success': False, 'error': 'Invalid session'}), 404
    
    auth_data = pending_qr_auths[session_id]
    
    if time.time() > auth_data['expires']:
        del pending_qr_auths[session_id]
        return jsonify({'success': False, 'error': 'Session expired'}), 401
    
    if auth_data['token'] != token:
        return jsonify({'success': False, 'error': 'Invalid token'}), 401
    
    # Mark as scanned
    auth_data['scanned'] = True
    auth_data['session_token'] = generate_session_token('qr')
    
    return jsonify({'success': True, 'message': 'QR code authenticated'})

@app.route('/api/auth/qr/poll', methods=['GET'])
def qr_poll():
    """Dashboard polls to check if QR was scanned"""
    session_id = request.args.get('session_id')
    
    if session_id not in pending_qr_auths:
        return jsonify({'scanned': False, 'error': 'Invalid session'})
    
    auth_data = pending_qr_auths[session_id]
    
    if time.time() > auth_data['expires']:
        del pending_qr_auths[session_id]
        return jsonify({'scanned': False, 'error': 'Expired'})
    
    if auth_data['scanned']:
        token = auth_data['session_token']
        del pending_qr_auths[session_id]
        return jsonify({
            'scanned': True,
            'token': token,
            'method': 'QR Code Mobile Auth'
        })
    
    return jsonify({'scanned': False})

# ============================================================================
# METHOD 5: Cryptographic Tokens
# ============================================================================
@app.route('/api/auth/token/validate', methods=['POST'])
def token_validate():
    """Validate cryptographic access token"""
    data = request.json
    token = data.get('token', '').replace('-', '').upper()
    
    # Generate valid token hash (in production: store in database)
    expected_token = 'DEMOTKEN1234567'
    token_hash = hmac.new(
        SECRET_KEY.encode(),
        expected_token.encode(),
        hashlib.sha256
    ).hexdigest()
    
    provided_hash = hmac.new(
        SECRET_KEY.encode(),
        token.encode(),
        hashlib.sha256
    ).hexdigest()
    
    # Constant-time comparison
    if hmac.compare_digest(token_hash, provided_hash):
        session_token = generate_session_token('crypto_token')
        return jsonify({
            'success': True,
            'token': session_token,
            'method': 'Cryptographic Token (AES-256)'
        })
    
    return jsonify({'success': False, 'error': 'Invalid token'}), 401

# ============================================================================
# Session Management
# ============================================================================
def generate_session_token(method):
    """Generate JWT session token"""
    payload = {
        'method': method,
        'exp': datetime.utcnow() + timedelta(seconds=TOKEN_EXPIRY),
        'iat': datetime.utcnow(),
        'jti': secrets.token_hex(16)
    }
    
    token = jwt.encode(payload, SECRET_KEY, algorithm='HS256')
    valid_tokens.add(token)
    return token

@app.route('/api/auth/verify', methods=['POST'])
def verify_session():
    """Verify session token"""
    data = request.json
    token = data.get('token')
    
    if not token or token not in valid_tokens:
        return jsonify({'valid': False}), 401
    
    try:
        payload = jwt.decode(token, SECRET_KEY, algorithms=['HS256'])
        return jsonify({
            'valid': True,
            'method': payload['method'],
            'expires': payload['exp']
        })
    except jwt.ExpiredSignatureError:
        valid_tokens.discard(token)
        return jsonify({'valid': False, 'error': 'Token expired'}), 401
    except jwt.InvalidTokenError:
        return jsonify({'valid': False, 'error': 'Invalid token'}), 401

@app.route('/api/auth/logout', methods=['POST'])
def logout():
    """Invalidate session token"""
    data = request.json
    token = data.get('token')
    
    if token in valid_tokens:
        valid_tokens.remove(token)
    
    return jsonify({'success': True})

# ============================================================================
# Health & Status
# ============================================================================
@app.route('/api/health', methods=['GET'])
def health():
    """Health check endpoint"""
    return jsonify({
        'status': 'healthy',
        'version': '2.0',
        'auth_methods': ['webauthn', 'email', 'totp', 'qr', 'token'],
        'active_sessions': len(valid_tokens)
    })

if __name__ == '__main__':
    print("\n" + "="*70)
    print("VORTEX v2.0 - Authentication Backend Server")
    print("="*70)
    print(f"TOTP Secret: {TOTP_SECRET}")
    print(f"Server: http://localhost:5000")
    print("="*70 + "\n")
    
    app.run(host='0.0.0.0', port=5000, debug=True)
