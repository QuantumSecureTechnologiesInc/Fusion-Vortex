# Fusion Runtime Core - Document Index

## Overview

This directory contains all project documentation for the Fusion Programming Language Runtime Core (v0.2.0). The documents are organised into categories for easy navigation.

**Last Updated**: 2025-12-08

---

## Quick Access

### Essential Reading

1. [README.md](../../README.md) - Project overview and getting started
2. [QuickStartGuide.md](../../QuickStartGuide.md) - Installation and basic usage
3. [ChangeLog.md](../../ChangeLog.md) - Version history and changes

### For Users

- [User Guide](guides/UserGuide.md) - Comprehensive tutorials and examples
- [Technical Reference](guides/TechnicalSheet.md) - API specifications and configuration

### For Developers

- [Developer Guide](guides/DeveloperGuide.md) - Architecture and contribution guidelines
- [Product Guide](guides/ProductGuide.md) - Vision, roadmap, and competitive analysis

---

## Documentation Structure

```text
docs/
├── guides/              # Comprehensive guides
│   ├── UserGuide.md
│   ├── DeveloperGuide.md
│   ├── TechnicalSheet.md
│   └── ProductGuide.md
├── design/              # Architecture and design documents
├── outputs/             # Project outputs and deliverables
├── test/                # Testing documentation and reports
├── reports/             # Status reports and analysis
├── security/            # Security audits and compliance
├── legal/               # Licensing and legal documents
├── support/             # FAQs and troubleshooting
├── deployment/          # Deployment guides
├── compliance/          # Regulatory compliance documents
├── roadmap/             # Planning and feature timelines
└── references/          # API docs and external references
```text

---

## Documentation by Category

### Guides (docs/guides/)

| Document              | Description                                                                    | Audience                     | Status     |
| --------------------- | ------------------------------------------------------------------------------ | ---------------------------- | ---------- |
| **UserGuide.md**      | Comprehensive user documentation with tutorials, examples, and troubleshooting | End Users                    | ✅ Complete |
| **DeveloperGuide.md** | Architecture deep-dive, build instructions, contribution guidelines            | Contributors                 | ✅ Complete |
| **TechnicalSheet.md** | Technical specifications, API reference, environment variables, error codes    | Technical Users              | ✅ Complete |
| **ProductGuide.md**   | Product vision, roadmap, competitive analysis, use cases, pricing              | Product Managers, Leadership | ✅ Complete |

### Root Documentation

| Document              | Description                                | Location  | Status |
| --------------------- | ------------------------------------------ | --------- | ------ |
| **v0.3.0_Roadmap.md** | Q1 2026 release plan (distributed runtime) | 📋 Planned |
| **v0.4.0_Roadmap.md** | Q2 2026 release plan (TPU support, FPGA)   | 📋 Planned |
| **v1.0.0_Roadmap.md** | Q4 2026 stable release plan                | 📋 Planned |

### References (docs/references/)

| Document                     | Description                           | Status    |
| ---------------------------- | ------------------------------------- | --------- |
| **API.md**                   | Complete API reference for all crates | 📋 Planned |
| **Configuration.md**         | Configuration file reference          | 📋 Planned |
| **Environment_Variables.md** | Environment variable reference        | 📋 Planned |
| **Error_Codes.md**           | Complete error code reference         | 📋 Planned |

### Deployment (docs/deployment/)

| Document               | Description                              | Status    |
| **Troubleshooting.md**    | Common issues and solutions    | 📋 Planned |
| **Performance_Tuning.md** | Performance optimisation guide | 📋 Planned |
| **Migration_Guide.md**    | Migrating from Tokio to Fusion | 📋 Planned |

---

## Documentation by Role

### For End Users (Developers Using Fusion)

**Priority Reading**:
1. ✅ [README.md](../../README.md)
2. ✅ [QuickStartGuide.md](../../QuickStartGuide.md)
3. ✅ [UserGuide.md](guides/UserGuide.md)
4. ✅ [TechnicalSheet.md](guides/TechnicalSheet.md)

**Additional Resources**:
- 📋 FAQ.md (Planned)
- 📋 Troubleshooting.md (Planned)
- 📋 API.md (Planned)

### For Contributors (Developers Working on Fusion)

**Priority Reading**:
1. ✅ [DeveloperGuide.md](guides/DeveloperGuide.md)
2. ✅ [ChangeLog.md](../../ChangeLog.md)
3. 📋 Architecture.md (Planned)

**Additional Resources**:
- 📋 Scheduler_Design.md (Planned)
- 📋 Memory_Manager_Design.md (Planned)
- 📋 HAL_Design.md (Planned)

### For Product Managers / Leadership

**Priority Reading**:
1. ✅ [ProductGuide.md](guides/ProductGuide.md)
2. ✅ [README.md](../../README.md)
3. 📋 Roadmap documents (Planned)

**Additional Resources**:
- ✅ Technical differentiation section in ProductGuide.md
- 📋 Competitive analysis reports (Planned)

### For DevOps / SRE

**Priority Reading**:
1. ✅ [TechnicalSheet.md](guides/TechnicalSheet.md)
2. 📋 Deployment guides (Planned)
3. 📋 Performance_Tuning.md (Planned)

**Additional Resources**:
- 📋 Docker.md (Planned)
- 📋 Kubernetes.md (Planned)
- 📋 Monitoring and observability (Planned)

---

## Documentation Standards

### Style Guide

- **Language**: British English
- **Format**: GitHub-flavoured Markdown
- **Code Blocks**: Always specify language (```rust, ```bash, ```toml)
- **Links**: Use relative links for internal docs, absolute for external
- **Headings**: Use ATX-style (#) with hierarchical structure
- **Line Length**: Soft limit 100 characters for readability

### Markdown Linting

All documentation must pass `markdownlint`:

```bash

# Check all docs

markdownlint docs/**/*.md

# Auto-fix where possible

markdownlint --fix docs/**/*.md
```text

### Documentation Review

| Status         | Description                 |
| -------------- | --------------------------- |
| ✅ Complete     | Reviewed and approved       |
| 🚧 In Progress  | Currently being written     |
| 📋 Planned      | On roadmap, not started     |
| ⚠️ Needs Update | Outdated, requires revision |

---

## Contributing to Documentation

### Adding New Documentation

1. Determine appropriate category (guides, design, etc.)
2. Create document following naming convention: `PascalCase_With_Underscores.md`
3. Add entry to this index (DocumentIndex.md)
4. Submit pull request with documentation changes

### Updating Existing Documentation

1. Make changes to relevant document
2. Update `Last Updated` date at top/bottom of document
3. Add entry to ChangeLog.md if significant
4. Submit pull request

### Documentation Templates

See `docs/templates/` directory (to be created) for:
- Guide template
- Design document template
- API reference template
- Tutorial template

---

## Version Control

All documentation is version-controlled alongside code:

- **Latest Stable**: https://github.com/QuantumSecureTechnologiesInc/Fusion/tree/main/docs
- **Development**: https://github.com/QuantumSecureTechnologiesInc/Fusion/tree/develop/docs
- **Version Tags**: https://github.com/QuantumSecureTechnologiesInc/Fusion/tags

---

## External Resources

### Official Links

- **Website**: [fusion-lang.org](https://fusion-lang.org)
- **Documentation**: [docs.fusion-lang.org](https://docs.fusion-lang.org)
- **API Docs**: [docs.rs/fusion_runtime_core](https://docs.rs/fusion_runtime_core)
- **Crates.io**: [crates.io/crates/fusion_runtime_core](https://crates.io/crates/fusion_runtime_core)

### Community

- **GitHub**: [github.com/QuantumSecureTechnologiesInc/Fusion](https://github.com/QuantumSecureTechnologiesInc/Fusion)
- **Discord**: [discord.gg/fusion-lang](https://discord.gg/fusion-lang)
- **Reddit**: [r/FusionLang](https://reddit.com/r/FusionLang)
- **Twitter**: [@FusionLang](https://twitter.com/FusionLang)

### Related Projects

- **Tokio**: [tokio.rs](https://tokio.rs) - Comparison baseline
- **Ray**: [ray.io](https://ray.io) - Distributed computing
- **Qiskit**: [qiskit.org](https://qiskit.org) - Quantum computing framework
- **PyTorch**: [pytorch.org](https://pytorch.org) - ML framework

---

## Feedback and Suggestions

We welcome feedback on our documentation:

- **GitHub Issues**: [Report documentation issue](https://github.com/QuantumSecureTechnologiesInc/Fusion/issues/new?labels=documentation)
- **Discord**: #documentation channel
- **Email**: docs@quantumsecuretech.com

---

**Maintained by**: Quantum Secure Technologies Inc. Documentation Team
**Last Updated**: 2025-12-08
**Version**: 0.2.0