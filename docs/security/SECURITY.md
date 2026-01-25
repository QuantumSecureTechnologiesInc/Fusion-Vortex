# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

The Fusion team takes security vulnerabilities seriously. We appreciate your efforts to responsibly disclose your findings.

### How to Report

<!-- Please DO NOT report security vulnerabilities through public GitHub issues. -->

Instead, please report them via email to: [security@fusion-lang.org](mailto:security@fusion-lang.org) (coming soon)

For now, please contact the maintainers directly through GitHub.

### What to Include

Please include the following information in your report:

- **Description** of the vulnerability
- **Steps to reproduce** the issue
- **Potential impact** of the vulnerability
- **Suggested fix** (if you have one)
- **Your contact information**

### Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial assessment**: Within 1 week
- **Status update**: Every 2 weeks
- **Resolution**: Depends on severity

### Disclosure Policy

- We will coordinate disclosure with you
- We request 90 days before public disclosure
- We will credit you in the security advisory (unless you prefer to remain anonymous)

## Security Best Practices

When using Fusion:

1. **Keep dependencies updated**
2. **Use latest stable version**
3. **Review third-party packages** before use
4. **Follow secure coding practices**
5. **Enable all compiler warnings**

## Known Security Considerations

### Borrow Checker

The Fusion borrow checker helps prevent:

- Use-after-free bugs
- Data races
- Null pointer dereferences

However, it's not a complete guarantee. Always:

- Review unsafe code carefully
- Test thoroughly
- Use static analysis tools

### WASM Security

When deploying to WebAssembly:

- Validate all inputs
- Implement proper sandboxing
- Follow WASM security best practices
- Keep runtime updated

## Security Updates

Security updates will be:

- Released as soon as possible
- Announced in release notes
- Documented in ChangeLog.md
- Tagged with security advisory

## Bug Bounty Program

Currently, we do not have a bug bounty program. This may change in the future.

---

**Thank you for helping keep Fusion secure!** 🔒