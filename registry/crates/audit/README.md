# Fusion Audit

**Version:** Workspace  
**Type:** Security Tool  
**License:** MIT / Apache 2.0 Dual License

## Overview

Fusion Audit (`fusion-audit`) is a security compliance tool designed to scan Fusion projects for vulnerabilities, analyze dependencies (SCA), and ensure supply chain security.

## Features

- **Dependency Scanning**: Checks dependencies against known vulnerability databases
- **License Compliance**: Verifies dependency licenses are compatible
- **Security Policy**: Enforces project-level security policies
- **Reporting**: Generates JSON/SARIF reports for CI/CD integration

## Usage

```rust
use fusion_audit::audit;

fn main() -> anyhow::Result<()> {
    // Run audit and print report
    let result = audit(true)?;
    
    for vuln in result.vulnerabilities {
        println!("Found vulnerability: {} in {}", vuln.title, vuln.package);
    }
    Ok(())
}
```

## Integration

- **CI/CD**: Designed to run as a step in build pipelines
- **Sentinel**: Integrates with Sentinel TriBrid for automated remediation

## Dependencies

- `reqwest`: Database queries
- `serde`: Report generation

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
