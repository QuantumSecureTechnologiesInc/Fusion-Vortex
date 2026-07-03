use std::path::Path;
use std::fs;
use anyhow::{Result, Context};

/// A unified structural rule that combines token matching with arbitrary logic checks.
pub struct ArchitecturalRule {
    pub name: &'static str,
    pub component_name: &'static str,
    pub required_tokens: Vec<&'static str>,
    pub forbidden_tokens: Vec<&'static str>,
    pub custom_detector: Option<fn(&str) -> Option<String>>,
}

pub struct FluxPolicyEngine {
    banned_cryptographic_symbols: Vec<String>,
    architectural_rules: Vec<ArchitecturalRule>,
}

impl FluxPolicyEngine {
    /// Compiles the complete security and systems architectural rule matrix.
    pub fn compile_ruleset() -> Self {
        // Strict global constraints blocking weak or outdated algorithms
        let banned_cryptographic_symbols = vec![
            "crypto::rsa".to_string(),
            "crypto::aes".to_string(),
            "md5::Hasher".to_string(),
        ];

        let architectural_rules = vec![
            ArchitecturalRule {
                name: "Warden Daemon Signal Check",
                component_name: "Warden Daemon",
                required_tokens: vec![
                    "signal_hook::flag::register",
                    "terminate_child_cleanly",
                    "try_wait",
                ],
                forbidden_tokens: vec!["std::process::exit"],
                custom_detector: Some(|content| {
                    if !content.contains("SIGINT") || !content.contains("SIGTERM") {
                        Some("The Warden Daemon must register both SIGINT and SIGTERM traps to ensure clean child termination under all exit states.".to_string())
                    } else {
                        None
                    }
                }),
            },
            ArchitecturalRule {
                name: "Nexus Performance Lock Check",
                component_name: "Nexus Router",
                required_tokens: vec![
                    "parking_lot::RwLock",
                    "BinaryHeap",
                    "handle_ingress_stream",
                ],
                forbidden_tokens: vec!["std::sync::RwLock"],
                custom_detector: Some(|content| {
                    if content.contains("std::thread::sleep") && !content.contains("from_millis(10)") {
                        Some("Thread starvation risk: The Nexus task loop must yield using highly predictable backoff intervals (exactly 10ms target).".to_string())
                    } else {
                        None
                    }
                }),
            },
            ArchitecturalRule {
                name: "TensorWeave Parity Verification",
                component_name: "TensorWeave Fabric",
                required_tokens: vec![
                    "is_parity",
                    "XOR",
                    "transmit_tensor",
                    "receive_and_reconstruct",
                ],
                forbidden_tokens: vec!["std::net::TcpStream"],
                custom_detector: Some(|content| {
                    if !content.contains("bincode::serialize") {
                        Some("Performance Warning: TensorWeave must utilize zero-copy binary serialization ('bincode') for packet-level layout efficiency.".to_string())
                    } else {
                        None
                    }
                }),
            },
            ArchitecturalRule {
                name: "Secure Handshake Integrity",
                component_name: "Secure TCP Stream",
                required_tokens: vec![
                    "Kyber768",
                    "X25519",
                    "EphemeralPrivateKey",
                    "SecureTcpListener",
                ],
                forbidden_tokens: vec!["crypto::rsa", "crypto::aes"],
                custom_detector: None,
            },
            ArchitecturalRule {
                name: "Solver Silicon Protection Guard",
                component_name: "Autonomous Solver",
                required_tokens: vec![
                    "thermal_threshold_celsius",
                    "Sha256",
                    "read_silicon_thermal_state",
                ],
                forbidden_tokens: vec!["std::thread::sleep(Duration::from_millis(0))"],
                custom_detector: Some(|content| {
                    if !content.contains("Duration::from_secs(5)") {
                        Some("Overheating Risk: The Solver core cooldown loop must execute a minimum 5-second wait window when thermal limits are breached.".to_string())
                    } else {
                        None
                    }
                }),
            },
            ArchitecturalRule {
                name: "Hypercycle Math Verification",
                component_name: "Lattice Mathematics Suite",
                required_tokens: vec![
                    "Quaternion",
                    "Octonion",
                    "slerp",
                    "chaotic_step",
                    "LatticeField",
                    "generate_noise",
                ],
                forbidden_tokens: vec!["std::f64::consts"],
                custom_detector: Some(|content| {
                    if !content.contains("hypersphere") {
                        Some("Mathematical Inaccuracy: Quaternion slerp documentation or implementation must explicitly reference 4D hypersphere paths.".to_string())
                    } else {
                        None
                    }
                }),
            },
            ArchitecturalRule {
                name: "Workspace Manifest Specification",
                component_name: "Project Manifest",
                required_tokens: vec![
                    "[package]",
                    "name",
                    "version",
                ],
                forbidden_tokens: vec!["[dependencies.cargo]"],
                custom_detector: Some(|content| {
                    if content.contains("[package]") && !content.contains("std") && !content.contains("dependencies") {
                        Some("Dependency Integrity: Ensure project packages declare explicit module mappings or upstream dependencies inside fusion.toml.".to_string())
                    } else {
                        None
                    }
                }),
            },
            ArchitecturalRule {
                name: "Strict Directives Enforcement",
                component_name: "Compiler Directives",
                required_tokens: vec![],
                forbidden_tokens: vec![],
                custom_detector: Some(|content| {
                    if content.contains("@unsafe") && !content.contains("@manual_memory") && !content.contains("@borrowed") {
                        Some("Safety Directive Violation: The use of '@unsafe' memory pathways requires explicit manual control via '@manual_memory' or '@borrowed' annotations.".to_string())
                    } else {
                        None
                    }
                }),
            },
        ];

        Self {
            banned_cryptographic_symbols,
            architectural_rules,
        }
    }

    /// Audits a specific file path against the compiled policy matrix.
    pub fn audit_source_tree(&self, file_path: &Path) -> Result<Vec<String>> {
        let content = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read source file at: {}", file_path.display()))?;
        
        let mut structural_violations = Vec::new();

        self.audit_legacy_cryptography(&content, &mut structural_violations);
        self.audit_unsafe_directives(&content, &mut structural_violations);
        self.audit_systems_architecture(&content, &mut structural_violations);

        Ok(structural_violations)
    }

    fn audit_legacy_cryptography(&self, content: &str, violations: &mut Vec<String>) {
        for banned_token in &self.banned_cryptographic_symbols {
            if content.contains(banned_token) {
                violations.push(format!(
                    "Policy Violation: Source references insecure legacy primitive: '{}'. All cryptography must use hybrid PQC workflows.",
                    banned_token
                ));
            }
        }
    }

    fn audit_unsafe_directives(&self, content: &str, violations: &mut Vec<String>) {
        if content.contains("@unsafe") && !content.contains("@manual_memory") && !content.contains("@borrowed") {
            violations.push(
                "Style Violation: Macro block overrides require manual memory lifecycle attributes explicitly via '@manual_memory' or '@borrowed'.".to_string()
            );
        }
    }

    fn audit_systems_architecture(&self, content: &str, violations: &mut Vec<String>) {
        for rule in &self.architectural_rules {
            // Step 1: Scan required token constraints
            for req in &rule.required_tokens {
                if !content.contains(req) {
                    violations.push(format!(
                        "Rule [{}]: Missing required structural signature element '{}' in component '{}'.",
                        rule.name, req, rule.component_name
                    ));
                }
            }

            // Step 2: Scan forbidden token constraints
            for banned in &rule.forbidden_tokens {
                if content.contains(banned) {
                    violations.push(format!(
                        "Rule [{}]: Prohibited token '{}' identified within component '{}'.",
                        rule.name, banned, rule.component_name
                    ));
                }
            }

            // Step 3: Run custom logic closures if present
            if let Some(detector_fn) = rule.custom_detector {
                if let Some(error_msg) = detector_fn(content) {
                    violations.push(format!("Rule [{}]: {}", rule.name, error_msg));
                }
            }
        }
    }
}

#[allow(dead_code)]
fn run_linter_audit() -> Result<()> {
    let engine = FluxPolicyEngine::compile_ruleset();
    let target_paths = vec![
        Path::new("src/net.rs"),
        Path::new("bin/the_warden_daemon.rs"),
        Path::new("bin/the_nexus_router_node.rs"),
        Path::new("bin/tensorweave_implementation.rs"),
        Path::new("bin/the_solver_compute_node.rs"),
        Path::new("std/src/math.fu"),
        Path::new("fusion.toml"),
    ];

    let mut total_failures = 0;

    for target in target_paths {
        if target.exists() {
            println!("Flux Verification Scan active on: {}", target.display());
            let violations = engine.audit_source_tree(target)?;
            if !violations.is_empty() {
                eprintln!("--- Architectural Integrity Check Failed for {} ---", target.display());
                for trace in &violations {
                    eprintln!("  * {}", trace);
                }
                total_failures += violations.len();
            }
        } else {
            // Log missing target files gracefully in discovery telemetry
            println!("Flux Discovery: Path '{}' is not present in local sandbox workspace.", target.display());
        }
    }

    if total_failures > 0 {
        eprintln!("Verification terminated with {} total policy violations.", total_failures);
        std::process::exit(1);
    }

    println!("Flux Verification Complete. All sovereign systems components strictly comply with Supernova rules.");
    Ok(())
}