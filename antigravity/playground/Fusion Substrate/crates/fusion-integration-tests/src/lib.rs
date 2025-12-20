#[cfg(test)]
mod tests {
    use fusion_observer::{EventType, Observer, TraceStatus};
    use fusion_plugin_loader::PluginLoader;
    use fusion_policy_dsl::{PolicyCompiler, PolicyDecision};
    use fusion_tee::{AttestationProvider, SoftwareAttestationProvider, TeeEnclave};
    use std::path::Path;

    #[tokio::test]
    async fn test_full_trust_chain_integration() {
        // 1. Compile Policy from DSL
        let dsl = r#"
            ALLOW capability.network.external IF trust > 0.8
            DENY capability.filesystem.write
            DEFAULT DENY
        "#;
        let policy = PolicyCompiler::compile(dsl).expect("Failed to compile policy");

        // 2. Initialize Observer
        let mut observer = Observer::new();
        let trace_id = "test-execution-001";
        observer.record_event(trace_id, "system", EventType::ExecutionStart);

        // 3. Mock Tool Data & Provenance (Hashing)
        let mock_plugin_data = b"WASM BINARY DATA STUB";
        let expected_hash = PluginLoader::compute_hash(mock_plugin_data);

        // Simulate hash verification success
        observer.record_event(
            trace_id,
            "provenance",
            EventType::ToolCall {
                tool_name: "network-agent".into(),
            },
        );

        // 4. Policy Decision based on Trust & Capability
        let current_trust = 0.85; // High trust
        let capability = "capability.network.external";

        let decision = policy.evaluate(capability, current_trust);
        match decision {
            PolicyDecision::Allow => {
                observer.record_event(
                    trace_id,
                    "policy",
                    EventType::PolicyDecision {
                        allowed: true,
                        reason: "Trust score and capability allowed by policy".into(),
                    },
                );
            }
            PolicyDecision::Deny { reason } => {
                panic!("Policy should have allowed this: {}", reason);
            }
        }

        // 5. TEE Execution & Attestation
        let mut enclave = TeeEnclave::new();
        enclave
            .initialize(mock_plugin_data)
            .expect("Failed to init enclave");

        let attestation = SoftwareAttestationProvider::new();
        let runtime_id = attestation.get_runtime_identity().unwrap();

        // Emit attestation claim as part of execution
        let claim = attestation.emit_claim(mock_plugin_data).unwrap();

        observer.record_event(
            trace_id,
            "tee",
            EventType::TrustUpdate {
                new_score: current_trust,
            },
        );

        // 6. Finalize execution in observer
        observer.record_event(trace_id, "runtime", EventType::ExecutionEnd);

        // 7. Verify Unified Trace
        let trace = observer.get_trace(trace_id).unwrap();
        assert_eq!(trace.status, TraceStatus::Completed);
        assert_eq!(trace.events.len(), 5);

        println!("Integration Test Trace Summary:");
        for event in &trace.events {
            println!(
                "[{:?}] Component: {} | Event: {:?}",
                event.timestamp, event.component, event.event_type
            );
        }

        assert!(runtime_id.contains("software-runtime"));
    }
}
