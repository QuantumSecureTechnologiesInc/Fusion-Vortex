//! Language Server Protocol (LSP) Implementation for Fusion.
//! Allows editors like VS Code to display real-time errors, types, and completion.
use crate::types::*;
use std::collections::HashMap;

use crate::parser;
use crate::sema;

// Simple JSON serialization helpers (placeholder for standard JSON lib)
fn escape_json_string(s: &str) -> FString {
    s.replace("\"", "\\\"").replace("\n", "\\n")
}

pub struct LanguageServer {
    documents: FMap<FString, FString>,
}

impl LanguageServer {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
        }
    }

    /// Main event loop reading JSON-RPC from stdin
    pub fn run(&mut self) {
        // In a real environment, this loops reading Content-Length headers,
        // then parses the JSON-RPC payload.
        // For standard io integration:
        loop {
            let req = self.read_message();
            if req.is_empty() { break; }
            self.handle_message(req);
        }
    }

    fn read_message(&mut self) -> FString {
        // Read "Content-Length: X\r\n\r\n{...}"
        // Stub for compilation
        "".to_string()
    }

    fn send_message(&self, json: &str) {
        let _content_length = json.len();
        // Native printf hook
        // printf("Content-Length: %d\r\n\r\n%s", content_length, json);
    }

    fn handle_message(&mut self, msg: FString) {
        // Very basic dispatch based on string matching (assumes robust JSON parser later)
        if msg.contains("\"method\":\"initialize\"") {
            self.handle_initialize();
        } else if msg.contains("\"method\":\"textDocument/didOpen\"") {
            self.handle_did_open(msg);
        } else if msg.contains("\"method\":\"textDocument/didChange\"") {
            self.handle_did_change(msg);
        }
    }

    fn handle_initialize(&self) {
        let response = r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "capabilities": {
                    "textDocumentSync": 1,
                    "hoverProvider": true,
                    "definitionProvider": true
                }
            }
        }"#;
        self.send_message(response);
    }

    fn handle_did_open(&mut self, _msg: FString) {
        // Extract URI and text...
        let uri = "dummy_uri"; // Stub
        let text = "dummy_text"; // Stub
        self.documents.insert(uri.to_string(), text.to_string());
        self.publish_diagnostics(uri.to_string(), text.to_string());
    }

    fn handle_did_change(&mut self, _msg: FString) {
        let uri = "dummy_uri"; // Stub
        let text = "dummy_text"; // Stub
        self.documents.insert(uri.to_string(), text.to_string());
        self.publish_diagnostics(uri.to_string(), text.to_string());
    }

    fn publish_diagnostics(&self, uri: FString, source: FString) {
        let parse_out = parser::parse_output(&source);
        let mut diags_json = FString::from("[");
        
        let mut has_items = false;

        for err in &parse_out.errors {
            if has_items { diags_json.push_str(","); }
            diags_json.push_str(&format!(
                r#"{{"range": {{"start":{{"line":0, "character":0}},"end":{{"line":0, "character":0}}}},"severity":1,"message":"{}"}}"#,
                escape_json_string(err)
            ));
            has_items = true;
        }

        if let Some(prog) = parse_out.program {
            let mut analyzer = sema::Analyzer::new();
            let sema_out = analyzer.analyze_output(prog);
            
            for err in &sema_out.errors {
                if has_items { diags_json.push_str(","); }
                diags_json.push_str(&format!(
                    r#"{{"range": {{"start":{{"line":0, "character":0}},"end":{{"line":0, "character":0}}}},"severity":1,"message":"{}"}}"#,
                    escape_json_string(err)
                ));
                has_items = true;
            }
        }

        diags_json.push(']');

        let notification = format!(
            r#"{{"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics", "params": {{"uri": "{}", "diagnostics": {}}}}}"#,
            uri, diags_json
        );
        self.send_message(&notification);
    }
}