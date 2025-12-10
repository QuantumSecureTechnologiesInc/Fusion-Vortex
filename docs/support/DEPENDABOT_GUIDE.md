# Dependabot Configuration Guide

## Overview

This project uses GitHub Dependabot to automatically manage and update dependencies across multiple package ecosystems. This ensures the project stays secure and up-to-date with the latest library versions.

## Supported Ecosystems

### 1. Rust/Cargo (Main Compiler)

- **Directory**: `/` (root)
- **Schedule**: Weekly on Mondays at 09:00 UTC
- **Max PRs**: 10
- **Grouping**:
  - Production dependencies grouped together
  - Development dependencies grouped separately

### 2. npm (VS Code Extension)

- **Directory**: `/editors/vscode-fusion`
- **Schedule**: Weekly on Tuesdays at 09:00 UTC
- **Max PRs**: 10
- **Grouping**:
  - Production dependencies grouped together
  - Dev dependencies (`@types/*`, `@typescript-eslint/*`) grouped separately

### 3. GitHub Actions

- **Directory**: `/` (root)
- **Schedule**: Weekly on Wednesdays at 09:00 UTC
- **Max PRs**: 5
- **Grouping**: All actions grouped together

## Auto-Merge Policy

Dependabot PRs are automatically processed based on the update type:

### Automatic Approval & Merge

**Patch updates** (`1.0.0` → `1.0.1`) and **minor updates** (`1.0.0` → `1.1.0`):

- ✅ Automatically approved
- ✅ Auto-merged after CI passes
- ✅ Uses squash merge strategy

### Manual Review Required

**Major updates** (`1.0.0` → `2.0.0`):

- ⚠️ Requires manual review
- ⚠️ May contain breaking changes
- ⚠️ Comment added to PR alerting reviewers

## Security Updates

- 🔒 **Security updates are created immediately** when vulnerabilities are detected
- 🔒 They bypass the normal schedule
- 🔒 Labeled with `security` for easy identification
- 🔒 Have higher priority than version updates

## Labels

All Dependabot PRs are tagged with labels for easy filtering:

- `dependencies` - All dependency updates
- `rust` / `npm` / `github-actions` - Ecosystem-specific
- `automated` - Automated updates
- `security` - Security-related updates (auto-added by GitHub)

## Commit Message Format

Dependabot follows Conventional Commits:

- **Cargo**: `deps(cargo): bump dependency-name from x.y.z to a.b.c`
- **npm**: `deps(npm): bump dependency-name from x.y.z to a.b.c`
- **npm dev**: `deps(npm-dev): bump dependency-name from x.y.z to a.b.c`
- **GitHub Actions**: `ci: bump actions/checkout from v3 to v4`

## Configuration Files

### `.github/dependabot.yml`

Main Dependabot configuration file that specifies:

- Package ecosystems to monitor
- Update schedules
- PR limits
- Reviewers
- Labels
- Commit message formatting
- Dependency grouping rules

### `.github/workflows/dependabot-auto-merge.yml`

Automated workflow that:

1. Detects Dependabot PRs
2. Fetches update metadata (patch/minor/major)
3. Auto-approves patch and minor updates
4. Enables auto-merge after CI passes
5. Adds warning comment for major updates

### `.github/workflows/ci.yml`

Comprehensive CI pipeline that runs on all PRs:

- ✅ Rust compiler tests (multi-OS, multi-version)
- ✅ Code formatting checks (`cargo fmt`)
- ✅ Linting (`cargo clippy`)
- ✅ Security audit (`cargo audit`)
- ✅ VS Code extension tests
- ✅ Dependency review
- ✅ Code coverage

## Monitoring Dependabot

### View Dependabot PRs

Navigate to the repository and filter pull requests:

```text
is:pr is:open author:app/dependabot
```

### Check Dependabot Alerts

Go to the repository **Security** tab → **Dependabot alerts**

### Review Pending Updates

Go to **Insights** → **Dependency graph** → **Dependabot**

## Manual Dependabot Actions

### Trigger Updates Manually

You can manually trigger Dependabot updates from the GitHub UI:

1. Go to **Insights** → **Dependency graph** → **Dependabot**
2. Click **"Check for updates"** for any ecosystem

### Dismiss a PR

If you want to dismiss a Dependabot PR:

```

# Comment on the PR

@dependabot ignore this dependency

# or

@dependabot ignore this major version

# or

@dependabot ignore this minor version
```

### Rebase a PR

If a Dependabot PR has conflicts:

```

# Comment on the PR

@dependabot rebase
```

### Recreate a PR

If you closed a PR but want it back:

```

# Comment on the closed PR

@dependabot recreate
```

## Ignoring Dependencies

To ignore specific dependencies or versions, update `.github/dependabot.yml`:

```yaml
updates:

  - package-ecosystem: "cargo"
    directory: "/"
    ignore:

      - dependency-name: "some-crate"
        versions: ["1.x"]  # Ignore 1.x versions

      - dependency-name: "another-crate"
        update-types: ["version-update:semver-major"]  # Ignore major updates
```

## Best Practices

1. **Review weekly**: Check Dependabot PRs every Monday/Tuesday/Wednesday
2. **Security first**: Merge security updates immediately
3. **Test before major updates**: Always test major version updates locally
4. **Keep CI green**: Ensure CI passes before merging
5. **Group updates**: Dependabot groups compatible updates to reduce PR noise
6. **Monitor alerts**: Check Security tab regularly for vulnerabilities

## Troubleshooting

### Dependabot not creating PRs

1. Check `.github/dependabot.yml` syntax
2. Verify schedule settings
3. Check if open PR limit is reached
4. Review Dependabot logs in **Insights** → **Dependency graph**

### Auto-merge not working

1. Ensure CI workflow passed
2. Check branch protection rules
3. Verify `dependabot-auto-merge.yml` workflow ran
4. Confirm GitHub Actions permissions are correct

### Too many PRs

Reduce the `open-pull-requests-limit` in `dependabot.yml`:

```yaml
open-pull-requests-limit: 5  # Default is 10
```

## Related Documentation

- [GitHub Dependabot Documentation](https://docs.github.com/en/code-security/dependabot)
- [Dependabot Configuration Options](https://docs.github.com/en/code-security/dependabot/dependabot-version-updates/configuration-options-for-the-dependabot.yml-file)
- [Auto-merge Dependabot PRs](https://docs.github.com/en/code-security/dependabot/working-with-dependabot/automating-dependabot-with-github-actions)

## Questions?

For issues or questions about Dependabot configuration, please open an issue in the repository.

---

**Last Updated**: December 7, 2025
**Maintainer**: Quantum Secure Technologies Inc.
