// src/lsp/server.rs - Fusion Language Server Protocol Implementation
// Based on tower-lsp framework for robust LSP support

use crate::parser::Parser;
use crate::semantic_analyzer::SemanticAnalyzer;
use async_trait::async_trait;
use std::collections::HashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

/// Fusion Language Server - Provides IDE integration via LSP
pub struct FusionLanguageServer {
    client: Client,
    /// Storage for opened documents (URI -> source code)
    open_documents: std::sync::Arc<tokio::sync::RwLock<HashMap<String, String>>>,
    /// Symbol index for go-to-definition and completion
    #[allow(dead_code)]
    symbol_index: std::sync::Arc<tokio::sync::RwLock<HashMap<String, Location>>>,
}

impl FusionLanguageServer {
    pub fn new(client: Client) -> Self {
        FusionLanguageServer {
            client,
            open_documents: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            symbol_index: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    /// Analyzes source code and extracts diagnostics and symbols
    async fn analyze_document(&self, uri: &str, code: &str) {
        // Step 1: Parsing (includes lexing via logos)
        let mut parser = Parser::new(code);
        let _ast = match parser.parse_program() {
            Ok(ast) => ast,
            Err(error) => {
                // Publish parse error diagnostic
                let diagnostic = Diagnostic {
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 0,
                        },
                        end: Position {
                            line: 0,
                            character: 1,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some("fusion-parser".to_string()),
                    message: error,
                    related_information: None,
                    tags: None,
                    data: None,
                };

                self.client
                    .publish_diagnostics(Url::parse(uri).unwrap(), vec![diagnostic], None)
                    .await;
                return;
            }
        };

        // Step 2: Semantic Analysis
        let mut analyzer = SemanticAnalyzer::new();
        match analyzer.analyze(_ast) {
            Ok(_) => {
                // Clear diagnostics on successful analysis
                self.client
                    .publish_diagnostics(Url::parse(uri).unwrap(), vec![], None)
                    .await;

                // TODO: Update symbol index from analyzer's symbol table
            }
            Err(errors) => {
                // Publish semantic errors
                let diagnostics: Vec<Diagnostic> = errors
                    .into_iter()
                    .map(|err| Diagnostic {
                        range: Range {
                            start: Position {
                                line: 0,
                                character: 0,
                            },
                            end: Position {
                                line: 0,
                                character: 1,
                            },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: None,
                        code_description: None,
                        source: Some("fusion-semantic".to_string()),
                        message: err,
                        related_information: None,
                        tags: None,
                        data: None,
                    })
                    .collect();

                self.client
                    .publish_diagnostics(Url::parse(uri).unwrap(), diagnostics, None)
                    .await;
            }
        }
    }
}

#[async_trait]
impl LanguageServer for FusionLanguageServer {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "fusion-lsp".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Fusion Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.text_document.text;

        // Store document
        self.open_documents
            .write()
            .await
            .insert(uri.clone(), text.clone());

        //  Trigger analysis
        self.analyze_document(&uri, &text).await;

        self.client
            .log_message(MessageType::INFO, format!("Opened document: {}", uri))
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();

        // Get the new content (full sync mode)
        if let Some(change) = params.content_changes.first() {
            let text = &change.text;

            // Update stored document
            self.open_documents
                .write()
                .await
                .insert(uri.clone(), text.clone());

            // Re-analyze
            self.analyze_document(&uri, text).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();

        // Remove from storage
        self.open_documents.write().await.remove(&uri);

        self.client
            .log_message(MessageType::INFO, format!("Closed document: {}", uri))
            .await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let _uri = params.text_document_position.text_document.uri.to_string();

        // Enhanced stdlib and collections completions
        let mut items = vec![
            // Collections types
            CompletionItem {
                label: "HashMap".to_string(),
                kind: Some(CompletionItemKind::CLASS),
                detail: Some("HashMap<K, V>".to_string()),
                documentation: Some(Documentation::String(
                    "Generic hash table for key-value storage with O(1) average operations"
                        .to_string(),
                )),
                insert_text: Some("HashMap<${1:K}, ${2:V}>::new()".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "HashSet".to_string(),
                kind: Some(CompletionItemKind::CLASS),
                detail: Some("HashSet<T>".to_string()),
                documentation: Some(Documentation::String(
                    "Generic set of unique values with O(1) insertion and lookup".to_string(),
                )),
                insert_text: Some("HashSet<${1:T}>::new()".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "Iterator".to_string(),
                kind: Some(CompletionItemKind::INTERFACE),
                detail: Some("trait Iterator<T>".to_string()),
                documentation: Some(Documentation::String(
                    "Iteration trait with next() and has_next() methods".to_string(),
                )),
                ..Default::default()
            },
            // Original stdlib types
            CompletionItem {
                label: "Vector".to_string(),
                kind: Some(CompletionItemKind::CLASS),
                detail: Some("Vector<T>".to_string()),
                documentation: Some(Documentation::String(
                    "Generic dynamic array with push, pop, and indexing operations".to_string(),
                )),
                insert_text: Some("Vector<${1:T}>::new()".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "Option".to_string(),
                kind: Some(CompletionItemKind::ENUM),
                detail: Some("Option<T>".to_string()),
                documentation: Some(Documentation::String(
                    "Optional value: Option::Some(value) or Option::None".to_string(),
                )),
                ..Default::default()
            },
            CompletionItem {
                label: "Result".to_string(),
                kind: Some(CompletionItemKind::ENUM),
                detail: Some("Result<T, E>".to_string()),
                documentation: Some(Documentation::String(
                    "Error handling: Result::Ok(value) or Result::Err(error)".to_string(),
                )),
                ..Default::default()
            },
            // Common functions
            CompletionItem {
                label: "println".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("fn println(s: string) -> void".to_string()),
                documentation: Some(Documentation::String(
                    "Print a string to standard output with newline".to_string(),
                )),
                insert_text: Some("println(${1:message})".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "assert".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("fn assert(condition: bool, message: string)".to_string()),
                documentation: Some(Documentation::String(
                    "Assert that a condition is true, panic with message if false".to_string(),
                )),
                insert_text: Some("assert(${1:condition}, ${2:\"message\"})".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "range".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("fn range(start: int, end: int) -> RangeIterator".to_string()),
                documentation: Some(Documentation::String(
                    "Create an iterator over a range of integers".to_string(),
                )),
                insert_text: Some("range(${1:start}, ${2:end})".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            // Keywords
            CompletionItem {
                label: "fn".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Function declaration".to_string()),
                insert_text: Some("fn ${1:name}(${2:params}) -> ${3:type} {\n\t$0\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "class".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Class declaration".to_string()),
                insert_text: Some("class ${1:Name} {\n\t$0\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "impl".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Implementation block".to_string()),
                insert_text: Some("impl ${1:Type} {\n\t$0\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "trait".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Trait declaration".to_string()),
                insert_text: Some("trait ${1:Name} {\n\t$0\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
        ];

        // Add type keywords
        for type_name in &["int", "float", "bool", "string", "void"] {
            items.push(CompletionItem {
                label: type_name.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some(format!("Built-in type: {}", type_name)),
                ..Default::default()
            });
        }

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let _uri = params
            .text_document_position_params
            .text_document
            .uri
            .to_string();

        // TODO: Implement type information hovering
        // For now, return a placeholder

        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(
                "Fusion Type Information".to_string(),
            )),
            range: None,
        }))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .to_string();
        let position = params.text_document_position_params.position;

        // Get document content
        let documents = self.open_documents.read().await;
        if let Some(code) = documents.get(&uri) {
            let lines: Vec<&str> = code.lines().collect();
            if let Some(line) = lines.get(position.line as usize) {
                let word = extract_word_at_position(line, position.character as usize);

                // Search for function/class definition
                for (line_num, line_text) in lines.iter().enumerate() {
                    // Look for "fn word(" or "class word" or "trait word"
                    if line_text.contains(&format!("fn {}(", word))
                        || line_text.contains(&format!("class {}", word))
                        || line_text.contains(&format!("trait {}", word))
                    {
                        return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                            uri: Url::parse(&uri).unwrap(),
                            range: Range {
                                start: Position {
                                    line: line_num as u32,
                                    character: 0,
                                },
                                end: Position {
                                    line: line_num as u32,
                                    character: line_text.len() as u32,
                                },
                            },
                        })));
                    }
                }
            }
        }

        Ok(None)
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri.to_string();

        // Get document content
        let documents = self.open_documents.read().await;
        if let Some(code) = documents.get(&uri) {
            // Basic formatting: normalize indentation and spacing
            let formatted = format_fusion_code(code);

            if formatted != *code {
                // Return a single edit replacing entire document
                let lines: Vec<&str> = code.lines().collect();
                return Ok(Some(vec![TextEdit {
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 0,
                        },
                        end: Position {
                            line: lines.len() as u32,
                            character: lines.last().map(|l| l.len()).unwrap_or(0) as u32,
                        },
                    },
                    new_text: formatted,
                }]));
            }
            Ok(Some(vec![]))
        } else {
            Ok(None)
        }
    }
}

/// Creates and runs the Fusion Language Server
pub async fn run_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| FusionLanguageServer::new(client)).finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lsp_creation() {
        // Basic test to ensure LSP server can be created
        let (service, _socket) =
            LspService::build(|client| FusionLanguageServer::new(client)).finish();

        // If this compiles and runs, the structure is valid
        drop(service);
    }
}
