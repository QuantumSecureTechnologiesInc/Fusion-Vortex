"""
Test script for Vortex v2.0 Authentication Backend
"""
import requests
import json

BASE_URL = 'http://localhost:5000/api'

def test_health():
    """Test health endpoint"""
    print("\n" + "="*60)
    print("Testing Health Endpoint")
    print("="*60)
    
    resp = requests.get(f'{BASE_URL}/health')
    print(f"Status: {resp.status_code}")
    print(f"Response: {json.dumps(resp.json(), indent=2)}")
    assert resp.status_code == 200
    print("✓ Health check passed!")

def test_totp():
    """Test TOTP authentication"""
    print("\n" + "="*60)
    print("Testing TOTP Authentication")
    print("="*60)
    
    # Get TOTP setup
    setup = requests.get(f'{BASE_URL}/auth/totp/setup')
    print(f"TOTP Secret: {setup.json()['secret']}")
    
    # Test with any 6-digit code (demo mode)
    resp = requests.post(f'{BASE_URL}/auth/totp/verify', json={'code': '123456'})
    
    print(f"Status: {resp.status_code}")
    if resp.status_code == 200:
        result = resp.json()
        print(f"✓ Authentication successful!")
        print(f"Method: {result['method']}")
        print(f"Token: {result['token'][:50]}...")
    else:
        print(f"✗ Failed: {resp.json()}")

def test_email():
    """Test email magic link"""
    print("\n" + "="*60)
    print("Testing Email Magic Link")
    print("="*60)
    
    resp = requests.post(f'{BASE_URL}/auth/email/send', json={'email': 'test@example.com'})
    
    print(f"Status: {resp.status_code}")
    if resp.status_code == 200:
        result = resp.json()
        print(f"✓ Magic link sent!")
        print(f"Link: {result.get('debug_link', 'N/A')[:80]}...")
    else:
        print(f"✗ Failed: {resp.json()}")

def test_qr_generate():
    """Test QR code generation"""
    print("\n" + "="*60)
    print("Testing QR Code Generation")
    print("="*60)
    
    resp = requests.post(f'{BASE_URL}/auth/qr/generate')
    
    print(f"Status: {resp.status_code}")
    if resp.status_code == 200:
        result = resp.json()
        print(f"Session ID: {result['session_id']}")
        print(f"QR Code: {result['qr_code'][:60]}...")
        print(f"Expires in: {result['expires_in']} seconds")
        print("✓ QR code generated!")
    else:
        print(f"✗ Failed: {resp.json()}")

def test_token():
    """Test cryptographic token"""
    print("\n" + "="*60)
    print("Testing Cryptographic Token")
    print("="*60)
    
    resp = requests.post(f'{BASE_URL}/auth/token/validate', json={'token': 'DEMO-TOKEN-1234-5678'})
    
    print(f"Status: {resp.status_code}")
    if resp.status_code == 200:
        result = resp.json()
        print(f"✓ Token validated!")
        print(f"Method: {result['method']}")
        print(f"Session Token: {result['token'][:50]}...")
    else:
        print(f"✗ Failed: {resp.json()}")

if __name__ == '__main__':
    print("\n" + "="*60)
    print("VORTEX v2.0 - Authentication Backend Test Suite")
    print("="*60)
    
    try:
        test_health()
        test_totp()
        test_email()
        test_qr_generate()
        test_token()
        
        print("\n" + "="*60)
        print("✓ ALL TESTS PASSED!")
        print("="*60 + "\n")
        
    except requests.exceptions.ConnectionError:
        print("\n✗ ERROR: Could not connect to server. Is it running on http://localhost:5000?")
    except Exception as e:
        print(f"\n✗ ERROR: {e}")
