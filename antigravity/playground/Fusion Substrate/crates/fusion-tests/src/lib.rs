#[cfg(test)]
mod integration_tests {
    use fusion_ledger::Ledger;
    use fusion_mcp_spec::{assert_version, McpRequest};
    use fusion_policy::{AllowListPolicy, Policy, PolicyDecision};
    use fusion_runtime::Runtime;
    use serde_json::json;
    use tempfile::NamedTempFile;

    #[test]
    fn test_mcp_version_lock() {
        // MCP 1.0 must be accepted
        assert!(assert_version("1.0").is_ok());

        // Any other version must be rejected
        assert!(assert_version("0.9").is_err());
        assert!(assert_version("1.1").is_err());
        assert!(assert_version("2.0").is_err());
    }

    #[test]
    fn test_policy_blocks_unauthorized_tool() {
        let temp_file = NamedTempFile::new().unwrap();
        let ledger = Ledger::new(temp_file.path().to_str().unwrap());
        let policy = AllowListPolicy::new(vec!["safe_tool".into()]);
        let mut runtime = Runtime::new(ledger, policy).unwrap();

        let req = McpRequest {
            id: "1".into(),
            tool: "evil_tool".into(),
            input: json!({}),
        };

        let result = runtime.execute(req);
        assert!(result.is_err());

        // Verify no ledger entry was created
        assert_eq!(runtime.sequence(), 0);
    }

    #[test]
    fn test_authorized_tool_executes() {
        let temp_file = NamedTempFile::new().unwrap();
        let ledger = Ledger::new(temp_file.path().to_str().unwrap());
        let policy = AllowListPolicy::new(vec!["safe_tool".into()]);
        let mut runtime = Runtime::new(ledger, policy).unwrap();

        let req = McpRequest {
            id: "1".into(),
            tool: "safe_tool".into(),
            input: json!({"data": "test"}),
        };

        let result = runtime.execute(req);
        assert!(result.is_ok());

        // Verify ledger entry was created
        assert_eq!(runtime.sequence(), 1);
    }

    #[test]
    fn test_crash_recovery_maintains_state() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        // Simulate normal execution
        {
            let ledger = Ledger::new(path);
            let policy =
                AllowListPolicy::new(vec!["tool_a".into(), "tool_b".into(), "tool_c".into()]);
            let mut runtime = Runtime::new(ledger, policy).unwrap();

            runtime
                .execute(McpRequest {
                    id: "1".into(),
                    tool: "tool_a".into(),
                    input: json!({}),
                })
                .unwrap();

            runtime
                .execute(McpRequest {
                    id: "2".into(),
                    tool: "tool_b".into(),
                    input: json!({}),
                })
                .unwrap();

            runtime
                .execute(McpRequest {
                    id: "3".into(),
                    tool: "tool_c".into(),
                    input: json!({}),
                })
                .unwrap();

            assert_eq!(runtime.sequence(), 3);
        } // Drop runtime (simulate crash)

        // Simulate recovery
        {
            let ledger = Ledger::new(path);
            let policy =
                AllowListPolicy::new(vec!["tool_a".into(), "tool_b".into(), "tool_c".into()]);
            let runtime = Runtime::new(ledger, policy).unwrap();

            // State is deterministically recovered
            assert_eq!(runtime.sequence(), 3);

            // All requests can be replayed
            let replayed = runtime.replay_all().unwrap();
            assert_eq!(replayed.len(), 3);
            assert_eq!(replayed[0].tool, "tool_a");
            assert_eq!(replayed[1].tool, "tool_b");
            assert_eq!(replayed[2].tool, "tool_c");
        }
    }

    #[test]
    fn test_ledger_sequence_integrity() {
        let temp_file = NamedTempFile::new().unwrap();
        let ledger = Ledger::new(temp_file.path().to_str().unwrap());
        let policy = AllowListPolicy::new(vec!["tool1".into(), "tool2".into()]);
        let mut runtime = Runtime::new(ledger, policy).unwrap();

        // Execute multiple requests
        for i in 0..10 {
            runtime
                .execute(McpRequest {
                    id: format!("{}", i),
                    tool: if i % 2 == 0 {
                        "tool1".into()
                    } else {
                        "tool2".into()
                    },
                    input: json!({"iteration": i}),
                })
                .unwrap();
        }

        // Verify sequence integrity
        assert_eq!(runtime.sequence(), 10);

        // Verify all entries are in ledger with correct sequence
        let entries = runtime.ledger().replay().unwrap();
        assert_eq!(entries.len(), 10);

        for (idx, entry) in entries.iter().enumerate() {
            assert_eq!(entry.sequence, idx as u64);
        }
    }

    #[test]
    fn test_policy_evaluation_with_multiple_policies() {
        let policy1 = AllowListPolicy::new(vec!["tool1".into(), "tool2".into()]);
        let policy2 = AllowListPolicy::new(vec!["tool2".into(), "tool3".into()]);

        // tool1 is in policy1 but not policy2
        assert_eq!(policy1.evaluate_simple("tool1"), PolicyDecision::Allow);
        assert!(policy2.evaluate_simple("tool1").is_deny());

        // tool2 is in both
        assert_eq!(policy1.evaluate_simple("tool2"), PolicyDecision::Allow);
        assert_eq!(policy2.evaluate_simple("tool2"), PolicyDecision::Allow);

        // tool3 is in policy2 but not policy1
        assert!(policy1.evaluate_simple("tool3").is_deny());
        assert_eq!(policy2.evaluate_simple("tool3"), PolicyDecision::Allow);
    }

    #[test]
    fn test_empty_ledger_replay() {
        let temp_file = NamedTempFile::new().unwrap();
        let ledger = Ledger::new(temp_file.path().to_str().unwrap());
        let policy = AllowListPolicy::new(vec!["tool1".into()]);
        let runtime = Runtime::new(ledger, policy).unwrap();

        // New runtime should have sequence 0
        assert_eq!(runtime.sequence(), 0);

        // Replaying empty ledger should return empty vector
        let replayed = runtime.replay_all().unwrap();
        assert_eq!(replayed.len(), 0);
    }

    #[test]
    fn test_policy_denial_reason_is_informative() {
        let temp_file = NamedTempFile::new().unwrap();
        let ledger = Ledger::new(temp_file.path().to_str().unwrap());
        let policy = AllowListPolicy::new(vec!["allowed_tool".into()]);
        let mut runtime = Runtime::new(ledger, policy).unwrap();

        let req = McpRequest {
            id: "1".into(),
            tool: "forbidden_tool".into(),
            input: json!({}),
        };

        let result = runtime.execute(req);
        assert!(result.is_err());

        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("forbidden_tool"));
        assert!(error_msg.contains("not permitted") || error_msg.contains("denied"));
    }
}
