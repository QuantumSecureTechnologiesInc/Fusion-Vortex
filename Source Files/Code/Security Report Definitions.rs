// src/security/report.fu - Standardized Security Analysis Data Structures

use fusion::collections::List;

// --- Finding Primitives ---

enum Severity:
    Critical
    High
    Medium
    Low
    Informational

enum FindingType:
    SQLInjection // SAST
    XSS // SAST
    InsecureCrypto // SAST
    DependencyVulnerability // SCA
    InputValidationFailure // Fuzzing

struct Location:
    file_path: String
    line: u32
    column: u32

/// Represents a single vulnerability or security issue found.
struct SecurityFinding:
    type: FindingType
    severity: Severity
    message: String
    location: Location
    cve_id: Option<String> // Common Vulnerabilities and Exposures ID (for SCA)

// --- Report Structure ---

/// Aggregates all findings from static analysis (SAST), dependency scanning (SCA), etc.
struct SecurityReport:
    tool_name: String // SAST, SCA, Fuzzing, etc.
    findings: List<SecurityFinding>
    
    fn add_finding(self, finding: SecurityFinding) -> SecurityReport:
        self.findings.push(finding)
        return self

    /// Sorts and prints the report based on severity (Critical first).
    fn summarize(self):
        // Sort logic placeholder
        println!("--- Security Report ({}) ---", self.tool_name);
        println!("Total Findings: {}", self.findings.len());
        
        for finding in self.findings {
            println!("[{}] {}: {}", finding.severity, finding.type, finding.message);
        }