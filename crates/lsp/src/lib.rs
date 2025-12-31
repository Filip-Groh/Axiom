use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::net::SocketAddr;
use lsp_server::{Connection, Message, Notification, Request as ServerRequest, RequestId, Response};
use lsp_types::{DidChangeTextDocumentParams, DidOpenTextDocumentParams, Hover, HoverContents, HoverParams, HoverProviderCapability, InitializeParams, MarkedString, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, Uri};
use lsp_types::notification::{DidChangeTextDocument, DidOpenTextDocument, Notification as _};
use lsp_types::request::{HoverRequest, Request};

fn log(message: &str) -> Result<(), Box<dyn Error>> {
    std::io::stderr().write_all((message.to_owned() + "\n").as_bytes())?;
    Ok(())
}

pub fn start() -> Result<(), Box<dyn Error>> {
    log("[Axiom LSP] - LSP server startup")?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));
    let (connection, io_thread) = Connection::listen(addr)?;

    let capabilities = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        ..Default::default()
    };

    let init_value = serde_json::to_value(capabilities)?;

    let init_params = connection.initialize(init_value)?;
    lsp_loop(connection, init_params)?;
    io_thread.join()?;

    log("[Axiom LSP] - LSP server shutdown")?;
    Ok(())
}

fn lsp_loop(connection: Connection, params: serde_json::Value) -> Result<(), Box<dyn Error>> {
    log("[Axiom LSP] - Connected")?;
    let _init_params: InitializeParams = serde_json::from_value(params)?;
    let mut files: HashMap<Uri, String> = HashMap::new();

    for msg in &connection.receiver {
        log(format!("[Axiom LSP] - Message: {:?}", &msg).as_str())?;
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    break;
                }
                if let Err(err) = handle_request(&connection, &req, &mut files) {
                    log(format!("[Axiom LSP] - Request {} failed: {err}", &req.method).as_str())?;
                }
            }
            Message::Notification(note) => {
                if let Err(err) = handle_notification(&connection, &note, &mut files) {
                    log(format!("[Axiom LSP] - Notification {} failed: {err}", &note.method).as_str())?;
                }
            }
            Message::Response(resp) => {
                log(format!("[Axiom LSP] - Response received {resp:?}").as_str())?;
            }
        }
    }

    log("[Axiom LSP] - Done")?;

    Ok(())
}

fn handle_request(connection: &Connection, req: &ServerRequest, files: &mut HashMap<Uri, String>) -> Result<(), Box<dyn Error>> {
    match req.method.as_str() {
        HoverRequest::METHOD => {
            log("[Axiom LSP] - Hover")?;
            let params: HoverParams = serde_json::from_value(req.params.clone())?;
            let uri = params.text_document_position_params.text_document.uri;
            let position = params.text_document_position_params.position;

            let file_content = files.get(&uri).ok_or(anyhow!("File not found!"))?;

            let tokens = Lexer::new(file_content).parse().unwrap();

            let mut ast = Parser::new(tokens).parse()?;

            let mut symbol_table = SymbolTable::new();
            symbol_table.add_build_in_types();

            let mut errors = vec![];

            ast.analyze(&mut symbol_table, &mut errors);

            let hover_node = ast.get_node_at(&position.into());

            if let Some(node) = hover_node {
                let hover = Hover {
                    contents: HoverContents::Scalar(MarkedString::String(
                        format!("{}", node.data_type())
                    )),
                    range: Some(node.location().into()),
                };
                send_ok(connection, req.id.clone(), &hover)?;
            } else {
                send_ok(connection, req.id.clone(), &None::<String>)?;
            }
        }
        _ => {}
    }

    Ok(())
}

fn handle_notification(connection: &Connection, note: &Notification, files: &mut HashMap<Uri, String>) -> Result<(), Box<dyn Error>> {
    match note.method.as_str() {
        DidOpenTextDocument::METHOD => {
            log("[Axiom LSP] - Document Open")?;
            let params: DidOpenTextDocumentParams = serde_json::from_value(note.params.clone())?;
            let uri = params.text_document.uri;

            files.insert(uri.clone(), params.text_document.text);
            send_diagnostic(connection, &uri, files)?;
        }
        DidChangeTextDocument::METHOD => {
            log("[Axiom LSP] - Document Change")?;
            let params: DidChangeTextDocumentParams = serde_json::from_value(note.params.clone())?;

            if let Some(change) = params.content_changes.into_iter().next() {
                let uri = params.text_document.uri;

                files.insert(uri.clone(), change.text);
                send_diagnostic(connection, &uri, files)?;
            }
        }
        _ => {}
    }

    Ok(())
}


// use rustc_hash::FxHashMap; // fast hash map
// use std::process::Stdio;
//
// #[allow(clippy::print_stderr, clippy::disallowed_types, clippy::disallowed_methods)]
use anyhow::{Result, anyhow};
use lsp_types::{
    Diagnostic,
    DiagnosticSeverity,
    Position,
    PublishDiagnosticsParams,
    Range,
    notification::{PublishDiagnostics},
};
use compiler::lexer::Lexer;
use compiler::parser::Parser;
use compiler::utils::SymbolTable;
use compiler::analyzer::Analyzer;
use compiler::error::AxiomError;

// for METHOD consts
//
// fn handle_notification(
//     conn: &Connection,
//     note: &lsp_server::Notification,
//     docs: &mut HashMap<Uri, String>,
// ) -> Result<()> {
//     match note.method.as_str() {
//         DidOpenTextDocument::METHOD => {
//             let p: DidOpenTextDocumentParams = serde_json::from_value(note.params.clone())?;
//             let uri = p.text_document.uri;
//             docs.insert(uri.clone(), p.text_document.text);
//             publish_dummy_diag(conn, &uri)?;
//         }
//         DidChangeTextDocument::METHOD => {
//             let p: DidChangeTextDocumentParams = serde_json::from_value(note.params.clone())?;
//             if let Some(change) = p.content_changes.into_iter().next() {
//                 let uri = p.text_document.uri;
//                 docs.insert(uri.clone(), change.text);
//                 publish_dummy_diag(conn, &uri)?;
//             }
//         }
//         _ => {}
//     }
//     Ok(())
// }
//
// // =====================================================================
// // requests
// // =====================================================================
//
// fn handle_request(
//     conn: &Connection,
//     req: &ServerRequest,
//     docs: &mut HashMap<Uri, String>,
// ) -> Result<()> {
//     match req.method.as_str() {
//         GotoDefinition::METHOD => {
//             send_ok(conn, req.id.clone(), &lsp_types::GotoDefinitionResponse::Array(Vec::new()))?;
//         }
//         Completion::METHOD => {
//             let item = CompletionItem {
//                 label: "HelloFromLSP".into(),
//                 kind: Some(CompletionItemKind::FUNCTION),
//                 detail: Some("dummy completion".into()),
//                 ..Default::default()
//             };
//             send_ok(conn, req.id.clone(), &CompletionResponse::Array(vec![item]))?;
//         }
//         HoverRequest::METHOD => {
//             let hover = Hover {
//                 contents: HoverContents::Scalar(MarkedString::String(
//                     "Hello from *minimal_lsp*".into(),
//                 )),
//                 range: None,
//             };
//             send_ok(conn, req.id.clone(), &hover)?;
//         }
//         _ => send_err(
//             conn,
//             req.id.clone(),
//             lsp_server::ErrorCode::MethodNotFound,
//             "unhandled method",
//         )?,
//     }
//     Ok(())
// }
//
// // =====================================================================
// // diagnostics
// // =====================================================================

fn analyze_for_errors(content: &String) -> Vec<AxiomError> {
    let tokens = Lexer::new(content).parse().unwrap();

    let mut ast = match Parser::new(tokens).parse() {
        Ok(ast) => ast,
        Err(error) => return vec![error]
    };

    let mut symbol_table = SymbolTable::new();
    symbol_table.add_build_in_types();

    let mut errors = vec![];

    ast.analyze(&mut symbol_table, &mut errors);
    errors
}

fn send_diagnostic(conn: &Connection, uri: &Uri, files: &mut HashMap<Uri, String>) -> Result<()> {
    let file_content = files.get(uri).ok_or(anyhow!("File not found!"))?;
    let errors = analyze_for_errors(file_content);

    let diagnostics: Vec<Diagnostic> = errors.iter().map(|error| {
        match error {
            AxiomError::UnexpectedEOF(position) => Diagnostic {
                range: Range::new(Position::new(position.line as u32, position.column as u32), Position::new(position.line as u32, position.column as u32 + 1)),
                severity: Some(DiagnosticSeverity::ERROR),
                code: None,
                code_description: None,
                source: Some("Axiom".into()),
                message: "Unexpected EOF".into(),
                related_information: None,
                tags: None,
                data: None,
            },
            AxiomError::SyntaxError(location, message) => Diagnostic {
                range: location.clone().into(),
                severity: Some(DiagnosticSeverity::ERROR),
                code: None,
                code_description: None,
                source: Some("Axiom".into()),
                message: format!("SyntaxError: {}", message),
                related_information: None,
                tags: None,
                data: None,
            },
            AxiomError::DuplicatedIdentifier(location, identifier) => Diagnostic {
                range: location.clone().into(),
                severity: Some(DiagnosticSeverity::ERROR),
                code: None,
                code_description: None,
                source: Some("Axiom".into()),
                message: format!("Duplicated identifier: {}", identifier),
                related_information: None,
                tags: None,
                data: None,
            },
            AxiomError::IdentifierUsedBeforeDeclaration(location, identifier) => Diagnostic {
                range: location.clone().into(),
                severity: Some(DiagnosticSeverity::ERROR),
                code: None,
                code_description: None,
                source: Some("Axiom".into()),
                message: format!("Identifier used before declaration: {}", identifier),
                related_information: None,
                tags: None,
                data: None,
            },
            AxiomError::WrongDataType(location, expected, received) => Diagnostic {
                range: location.clone().into(),
                severity: Some(DiagnosticSeverity::ERROR),
                code: None,
                code_description: None,
                source: Some("Axiom".into()),
                message: format!("Expected DataType: {}, but found: {}", expected, received),
                related_information: None,
                tags: None,
                data: None,
            },
            AxiomError::NotAFunction(location, identifier) => Diagnostic {
                range: location.clone().into(),
                severity: Some(DiagnosticSeverity::ERROR),
                code: None,
                code_description: None,
                source: Some("Axiom".into()),
                message: format!("{} is not a function", identifier),
                related_information: None,
                tags: None,
                data: None,
            },
            AxiomError::MismatchedNumberOfParameters(location, identifier, function_parameter_count, call_parameter_count) => Diagnostic {
                range: location.clone().into(),
                severity: Some(DiagnosticSeverity::ERROR),
                code: None,
                code_description: None,
                source: Some("Axiom".into()),
                message: format!("Mismatched number of parameters, function {} takes {} parameters, but given {}", identifier, function_parameter_count, call_parameter_count),
                related_information: None,
                tags: None,
                data: None,
            },
            AxiomError::NotAType(location, identifier) => Diagnostic {
                range: location.clone().into(),
                severity: Some(DiagnosticSeverity::ERROR),
                code: None,
                code_description: None,
                source: Some("Axiom".into()),
                message: format!("{} is not a type", identifier),
                related_information: None,
                tags: None,
                data: None,
            },
        }
    }).collect();

    log(format!("[Axiom LSP] - Diagnostics: {:?}", diagnostics).as_str()).unwrap();

    let params =
        PublishDiagnosticsParams { uri: uri.clone(), diagnostics, version: None };
    conn.sender.send(Message::Notification(lsp_server::Notification::new(
        PublishDiagnostics::METHOD.to_owned(),
        params,
    )))?;
    Ok(())
}
//
// // =====================================================================
// // helpers
// // =====================================================================
//
// fn full_range(text: &str) -> Range {
//     let last_line_idx = text.lines().count().saturating_sub(1) as u32;
//     let last_col = text.lines().last().map_or(0, |l| l.chars().count()) as u32;
//     Range::new(Position::new(0, 0), Position::new(last_line_idx, last_col))
// }
//
fn send_ok<T: serde::Serialize>(conn: &Connection, id: RequestId, result: &T) -> Result<()> {
    let resp = Response { id, result: Some(serde_json::to_value(result)?), error: None };
    conn.sender.send(Message::Response(resp))?;
    Ok(())
}
//
// fn send_err(
//     conn: &Connection,
//     id: RequestId,
//     code: lsp_server::ErrorCode,
//     msg: &str,
// ) -> Result<()> {
//     let resp = Response {
//         id,
//         result: None,
//         error: Some(lsp_server::ResponseError {
//             code: code as i32,
//             message: msg.into(),
//             data: None,
//         }),
//     };
//     conn.sender.send(Message::Response(resp))?;
//     Ok(())
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
