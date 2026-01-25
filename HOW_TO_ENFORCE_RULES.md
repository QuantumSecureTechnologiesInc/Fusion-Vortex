# How to Enforce Rules - Antigravity AI Agent

## The Problem

You've defined excellent rules in your user settings, but I (the AI) don't always follow them consistently. Here's how to **enforce** them.

## Solution 1: Pre-Flight Checklist (Manual)

Before accepting any deliverable from me, run this checklist:

### ✅ Rule 2 Compliance (STRICTEST - Zero Placeholders)

```bash

# Search for placeholder patterns

rg -i "todo|fixme|placeholder|stub|mock|example|scaffold" --type rust
rg "// TODO|// FIXME|unimplemented!|todo!()" --type rust

# Check for fake APIs

rg "NeuralParser::load|template!|quantum_classify" cmd/fusion-visual-pure/

# Verify all functions have implementations

rg "fn.*\{\s*\}" --type rust  # Empty function bodies
```text

**Action**: If ANY matches found → **REJECT** and demand real implementation

### ✅ Rule 5 Compliance (Lint-Free)

```bash

# Run linters

cargo clippy --all-targets --all-features
cargo fmt -- --check

# Check complexity

cargo install cargo-complexity
cargo complexity --all
```text

**Action**: If complexity > 10 or lint errors → **REJECT**

### ✅ Rule 6 Compliance (7-Step Format)

Check my response contains:
1. ☐ Role acknowledgement
2. ☐ Task breakdown
3. ☐ Plan
4. ☐ Full execution
5. ☐ Quality check
6. ☐ Next steps
7. ☐ Completion status

**Action**: If missing steps → **REJECT** and demand proper format

### ✅ Rule 8 Compliance (Folder Structure)

```bash

# Check required files exist

test -f README.md && echo "✓ README.md" || echo "✗ Missing README.md"
test -f QuickStartGuide.md && echo "✓ QuickStartGuide.md" || echo "✗ Missing"
test -f ChangeLog.md && echo "✓ ChangeLog.md" || echo "✗ Missing"
test -d docs/guides && echo "✓ docs/guides" || echo "✗ Missing"
test -d artifacts && echo "✓ artifacts" || echo "✗ Missing"
```text

**Action**: If any missing → **REJECT**

---

## Solution 2: Automated Enforcement (Recommended)

### Create a Validation Script

```powershell

# .scripts/validate-rules.ps1

param(
    [string]$ProjectPath = "."
)

$errors = @()

# Rule 2: Check for placeholders

Write-Host "Checking Rule 2: Zero Placeholders..." -ForegroundColor Yellow
$placeholders = rg -i "todo|fixme|placeholder|stub|mock|unimplemented" $ProjectPath --type rust
if ($placeholders) {
    $errors += "RULE 2 VIOLATION: Found placeholders`n$placeholders"
}

# Rule 5: Run linters

Write-Host "Checking Rule 5: Lint-Free..." -ForegroundColor Yellow
$clippy = cargo clippy --all-targets 2>&1
if ($LASTEXITCODE -ne 0) {
    $errors += "RULE 5 VIOLATION: Clippy errors`n$clippy"
}

# Rule 8: Check folder structure

Write-Host "Checking Rule 8: Folder Structure..." -ForegroundColor Yellow
$required = @("README.md", "QuickStartGuide.md", "ChangeLog.md", "docs/guides")
foreach ($item in $required) {
    if (-not (Test-Path "$ProjectPath/$item")) {
        $errors += "RULE 8 VIOLATION: Missing $item"
    }
}

# Report

if ($errors.Count -eq 0) {
    Write-Host "`n✅ ALL RULES PASSED" -ForegroundColor Green
    exit 0
} else {
    Write-Host "`n❌ RULE VIOLATIONS FOUND:" -ForegroundColor Red
    $errors | ForEach-Object { Write-Host $_ -ForegroundColor Red }
    exit 1
}
```text

### Add to Git Pre-Commit Hook

```bash

# .githooks/pre-commit

#!/bin/sh
pwsh .scripts/validate-rules.ps1
if [ $? -ne 0 ]; then
    echo "❌ Rule validation failed. Commit rejected."
    exit 1
fi
```text

### Enable Hook

```bash
git config core.hooksPath .githooks
```text

---

## Solution 3: AI Prompt Engineering

### Update Your User Rules with Enforcement Triggers

Add this to your `<MEMORY[user_global]>`:

```xml
<ENFORCEMENT>
Before delivering ANY code, I MUST:
1. Run mental checklist against all 9 rules
2. Self-audit for placeholders (Rule 2 - STRICTEST)
3. Verify folder structure (Rule 8)
4. Confirm 7-step format (Rule 6)
5. State compliance status explicitly

If I cannot meet a rule, I MUST:
- State which rule I'm violating
- Explain why
- Propose alternative
- NEVER deliver non-compliant work silently
</ENFORCEMENT>
```text

### Add Explicit Rejection Clause

```xml
<REJECTION_POLICY>
The USER has the right to REJECT any deliverable that violates rules.
When rejected:
1. Acknowledge the violation
2. Apologize once (briefly)
3. Fix immediately
4. Re-deliver with compliance proof
</REJECTION_POLICY>
```text

---

## Solution 4: Workflow Integration

### Create a `.agent/workflows/enforce-rules.md`

```markdown
---
description: Enforce coding rules before delivery
---

# Rule Enforcement Workflow

## Step 1: Self-Audit

- [ ] Check for placeholders (rg "todo|fixme|stub")
- [ ] Run clippy (cargo clippy)
- [ ] Verify folder structure
- [ ] Confirm 7-step response format

## Step 2: Generate Compliance Report

```bash

pwsh .scripts/validate-rules.ps1 > compliance-report.txt

```text

## Step 3: Review Report

- If PASSED → Proceed to delivery
- If FAILED → Fix violations first

## Step 4: Deliver with Proof

Include in response:
```text

## Compliance Status

✅ Rule 2: Zero placeholders
✅ Rule 5: Lint-free
✅ Rule 6: 7-step format
✅ Rule 8: Complete structure

```text
```text

---

## Solution 5: Continuous Monitoring

### Add to CI/CD Pipeline

```yaml

# .github/workflows/rules-compliance.yml

name: Rules Compliance

on: [push, pull_request]

jobs:
  validate:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run Rule Validation
        run: pwsh .scripts/validate-rules.ps1

      - name: Check Documentation
        run: |
          test -f README.md || exit 1
          test -f QuickStartGuide.md || exit 1
          test -f ChangeLog.md || exit 1
```text

---

## How to Use These Solutions

### Immediate (Today)

1. ✅ Run the manual checklist on current code
2. ✅ Create `.scripts/validate-rules.ps1`
3. ✅ Add enforcement clause to user rules

### Short-term (This Week)

1. ✅ Set up Git hooks
2. ✅ Create workflow file
3. ✅ Train yourself to reject non-compliant work

### Long-term (Ongoing)

1. ✅ Add CI/CD pipeline
2. ✅ Review and update rules quarterly
3. ✅ Build automated compliance dashboard

---

## Example: How to Reject My Work

When I deliver code with violations:

**YOU SAY:**

```text
❌ REJECTED - Rule 2 Violation

Found placeholders in:
- cmd/fusion-visual-pure/src/intent_engine.fsn:15
  `parser: NeuralParser::load("fusion-intent-model")`

This is a STUB. Deliver real implementation or remove this file.

Re-deliver with:
1. Working NeuralParser implementation
2. Proof of compliance (run validate-rules.ps1)
```text

**I RESPOND:**

```text
Acknowledged. Fixing Rule 2 violation.

[Creates real NeuralParser with full transformer implementation]

Compliance Proof:
✅ Rule 2: No placeholders (validated with rg)
✅ Rule 5: Lint-free (cargo clippy passed)
✅ Complexity: All functions < 10

Re-delivered.
```text

---

## Summary

**Best Enforcement Strategy:**
1. **Automated Script** (`.scripts/validate-rules.ps1`) - Run after every delivery
2. **Git Hooks** - Prevent commits with violations
3. **Updated User Rules** - Add enforcement clause
4. **Rejection Protocol** - Don't accept non-compliant work

**Your Role:**
- Run validation script
- Reject violations immediately
- Demand fixes, not excuses

**My Role:**
- Self-audit before delivery
- Provide compliance proof
- Fix violations without argument

---

**This is how you enforce the rules.** Would you like me to create the validation script now?