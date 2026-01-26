# HyperCycle Vortex v2.0.2 - Production Security Hardening

## Overview

This guide provides comprehensive security hardening procedures for production deployments of HyperCycle Vortex v2.0.2.

## Built-in Security Features

Vortex v2.0.2 includes the following security hardening at compile-time:

✅ **Stack Protection**: `-fstack-protector-strong`  
✅ **Buffer Overflow Detection**: `-D_FORTIFY_SOURCE=2`  
✅ **Format Security**: `-Wformat-security`  
✅ **PIE/ASLR**: `-fPIE` + `-Wl,-z,relro -Wl,-z,now`  
✅ **Windows Hardening**: `/GS /guard:cf /DYNAMICBASE`

## Operating System Hardening

### Linux

#### 1. Kernel Parameters

```bash
# /etc/sysctl.d/99-hypercycle-hardening.conf

# ASLR
kernel.randomize_va_space = 2

# Restrict dmesg
kernel.dmesg_restrict = 1

# Restrict kernel pointers
kernel.kptr_restrict = 2

# Disable core dumps
kernel.core_uses_pid = 1
fs.suid_dumpable = 0

# Network hardening
net.ipv4.conf.all.rp_filter = 1
net.ipv4.conf.default.rp_filter = 1
net.ipv4.tcp_syncookies = 1
net.ipv4.conf.all.accept_redirects = 0
net.ipv6.conf.all.accept_redirects = 0
```

Apply with: `sudo sysctl -p /etc/sysctl.d/99-hypercycle-hardening.conf`

#### 2. AppArmor/SELinux

**AppArmor Profile** (`/etc/apparmor.d/hypercycle-vortex`):

```
#include <tunables/global>

/opt/hypercycle/bin/vortex_dashboard {
  #include <abstractions/base>
  
  # Binary
  /opt/hypercycle/bin/vortex_dashboard mr,
  /opt/hypercycle/lib/** mr,
  
  # Data
  /opt/hypercycle/data/** rw,
  /opt/hypercycle/logs/** rw,
  
  # System
  /dev/urandom r,
  /proc/cpuinfo r,
  
  # Deny everything else
  deny /** wx,
}
```

Enable: `sudo apparmor_parser -r /etc/apparmor.d/hypercycle-vortex`

### Windows

#### 1. Windows Defender Application Control (WDAC)

Create code integrity policy:

```powershell
New-CIPolicy -Level FilePublisher `
  -FilePath "C:\HyperCycle\WDAC-Policy.xml" `
  -Fallback Hash `
  -UserPEs `
  -ScanPath "C:\Program Files\HyperCycle\"
```

#### 2. Windows Firewall

```powershell
# Allow only necessary ports
New-NetFirewallRule -DisplayName "HyperCycle Vortex" `
  -Direction Inbound `
  -Program "C:\Program Files\HyperCycle\vortex_dashboard.exe" `
  -Action Allow `
  -Profile Domain,Private
```

## Container Security

### Docker

#### 1. Run as Non-Root

```dockerfile
USER hypercycle:hypercycle
```

#### 2. Read-Only Root Filesystem

```yaml
security_opt:
  - no-new-privileges:true
read_only: true
```

#### 3. Resource Limits

```yaml
deploy:
  resources:
    limits:
      cpus: '2.0'
      memory: 2G
```

#### 4. Security Scanning

```bash
# Scan image for vulnerabilities
docker scan hypercycle/vortex:2.0.2

# Or use Trivy
trivy image hypercycle/vortex:2.0.2
```

### Kubernetes

#### 1. Pod Security Standards

```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: hypercycle
  labels:
    pod-security.kubernetes.io/enforce: restricted
    pod-security.kubernetes.io/audit: restricted
    pod-security.kubernetes.io/warn: restricted
```

#### 2. Network Policies

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: hypercycle-vortex-netpol
spec:
  podSelector:
    matchLabels:
      app: hypercycle-vortex
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - podSelector:
        matchLabels:
          app: allowed-client
    ports:
    - protocol: TCP
      port: 8080
  egress:
  - to:
    - podSelector:
        matchLabels:
          app: monitoring
    ports:
    - protocol: TCP
      port: 9090
```

#### 3. RBAC

```yaml
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: hypercycle-vortex-role
rules:
- apiGroups: [""]
  resources: ["configmaps"]
  verbs: ["get", "list"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: hypercycle-vortex-rolebinding
subjects:
- kind: ServiceAccount
  name: hypercycle-vortex
roleRef:
  kind: Role
  name: hypercycle-vortex-role
  apiGroup: rbac.authorization.k8s.io
```

## Network Security

### TLS/SSL Configuration

#### 1. Generate Certificates

```bash
# Self-signed for testing
openssl req -x509 -newkey rsa:4096 -nodes \
  -keyout hypercycle.key \
  -out hypercycle.crt \
  -days 365 \
  -subj "/CN=hypercycle.local"

# For production, use Let's Encrypt or your CA
```

#### 2. Nginx Reverse Proxy

```nginx
server {
    listen 443 ssl http2;
    server_name hypercycle.example.com;
    
    ssl_certificate /etc/ssl/certs/hypercycle.crt;
    ssl_certificate_key /etc/ssl/private/hypercycle.key;
    
    # Modern SSL configuration
    ssl_protocols TLSv1.3;
    ssl_prefer_server_ciphers off;
    ssl_ciphers 'TLS_AES_128_GCM_SHA256:TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256';
    
    # HSTS
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    
    # Security headers
    add_header X-Frame-Options "DENY" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    
    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## Monitoring & Auditing

### 1. Enable Audit Logging

```bash
# Configure auditd rules
cat >> /etc/audit/rules.d/hypercycle.rules <<EOF
-w /opt/hypercycle/bin/ -p x -k hypercycle_exec
-w /opt/hypercycle/data/ -p wa -k hypercycle_data
-w /opt/hypercycle/logs/ -p wa -k hypercycle_logs
EOF

sudo service auditd restart
```

### 2. Log Monitoring

Use centralized logging (ELK, Splunk, or similar):

```yaml
# Filebeat configuration
filebeat.inputs:
- type: log
  enabled: true
  paths:
    - /opt/hypercycle/logs/*.log
  fields:
    app: hypercycle-vortex
    env: production
```

## Secrets Management

### 1. Kubernetes Secrets

```bash
# Create secret
kubectl create secret generic hypercycle-secrets \
  --from-literal=api-key=your-api-key \
  --namespace=hypercycle

# Use in deployment
env:
- name: API_KEY
  valueFrom:
    secretKeyRef:
      name: hypercycle-secrets
      key: api-key
```

### 2. HashiCorp Vault

```bash
# Store secret
vault kv put secret/hypercycle/api-key value=your-api-key

# Retrieve in application
vault kv get -field=value secret/hypercycle/api-key
```

## Backup & Recovery

### 1. Data Backup

```bash
#!/bin/bash
# backup-hypercycle.sh

BACKUP_DIR="/backup/hypercycle"
DATE=$(date +%Y%m%d_%H%M%S)

# Backup data
tar -czf "$BACKUP_DIR/data-$DATE.tar.gz" /opt/hypercycle/data/

# Backup configuration
tar -czf "$BACKUP_DIR/config-$DATE.tar.gz" /opt/hypercycle/config/

# Retain last 30 days
find "$BACKUP_DIR" -name "*.tar.gz" -mtime +30 -delete
```

### 2. Disaster Recovery

Document and test recovery procedures:

1. Restore from backup
2. Verify data integrity
3. Test functionality
4. Update DNS/load balancers

## Compliance Checklist

- [ ] All binaries compiled with security hardening flags
- [ ] Running as non-root user
- [ ] Read-only root filesystem (containers)
- [ ] Resource limits configured
- [ ] Network policies in place
- [ ] TLS/SSL enabled
- [ ] Secrets properly managed
- [ ] Audit logging enabled
- [ ] Backups automated and tested
- [ ] Incident response plan documented
- [ ] Security scanning automated
- [ ] Vulnerability management process in place

## Security Contacts

- **Security Team**: <security@hypercycle.example.com>
- **Incident Response**: <incident@hypercycle.example.com>
- **Vulnerability Disclosure**: See SECURITY.md

---

**Last Updated**: 2026-01-25  
**Version**: 1.0
