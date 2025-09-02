use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use lsp_server::{Connection, Message, Notification, Request as ServerRequest, RequestId, Response};
use lsp_types::{CompletionOptions, DidChangeTextDocumentParams, DidOpenTextDocumentParams, Hover, HoverContents, HoverProviderCapability, InitializeParams, MarkedString, OneOf, PositionEncodingKind, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions, Uri};
use lsp_types::notification::{DidChangeTextDocument, DidOpenTextDocument, Notification as _};
use lsp_types::request::{HoverRequest, Request};

fn log(message: &str) -> Result<(), Box<dyn Error>> {
    std::io::stderr().write_all((message.to_owned() + "\n").as_bytes())?;
    Ok(())
}

pub fn start() -> Result<(), Box<dyn Error>> {
    log("[Axiom LSP] - LSP server startup")?;

    let (connection, io_thread) = Connection::stdio();

    let capabilities = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        completion_provider: Some(CompletionOptions::default()),
        definition_provider: Some(OneOf::Left(true)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        ..Default::default()
    };

    let init_value = serde_json::json!({
        "capabilities": capabilities,
        "offsetEncoding": ["utf-8"],
    });

    let init_params = connection.initialize(init_value)?;
    lsp_loop(connection, init_params)?;
    io_thread.join()?;

    log("[Axiom LSP] - LSP server shutdown")?;
    Ok(())
}

fn lsp_loop(connection: Connection, params: serde_json::Value) -> Result<(), Box<dyn Error>> {
    log("[Axiom LSP] - Connected")?;
    let init_params: InitializeParams = serde_json::from_value(params)?;
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

// fn handle_request(connection: &Connection, req: &ServerRequest, files: &mut HashMap<Uri, String>) -> Result<(), Box<dyn Error>> {
//     match req.method.as_str() {
//         HoverRequest::METHOD => {
//             log("[Axiom LSP] - Hover")?;
//             let hover = Hover {
//                 contents: HoverContents::Scalar(MarkedString::String(
//                     "Hello from *Axiom LSP*".into(),
//                 )),
//                 range: None,
//             };
//             send_ok(connection, req.id.clone(), &hover)?;
//         }
//         _ => {}
//     }
//
//     Ok(())
// }
//
// fn handle_notification(connection: &Connection, note: &Notification, files: &mut HashMap<Uri, String>) -> Result<(), Box<dyn Error>> {
//     match note.method.as_str() {
//         DidOpenTextDocument::METHOD => {
//             log("[Axiom LSP] - Document Open")?;
//             let params: DidOpenTextDocumentParams = serde_json::from_value(note.params.clone())?;
//             let uri = params.text_document.uri;
//
//             files.insert(uri.clone(), params.text_document.text);
//         }
//         DidChangeTextDocument::METHOD => {
//             log("[Axiom LSP] - Document Change")?;
//             let params: DidChangeTextDocumentParams = serde_json::from_value(note.params.clone())?;
//
//             if let Some(change) = params.content_changes.into_iter().next() {
//                 let uri = params.text_document.uri;
//
//                 files.insert(uri.clone(), change.text);
//             }
//         }
//         _ => {}
//     }
//
//     Ok(())
// }


use rustc_hash::FxHashMap; // fast hash map
use std::process::Stdio;

#[allow(clippy::print_stderr, clippy::disallowed_types, clippy::disallowed_methods)]
use anyhow::{Context, Result, anyhow, bail};
use lsp_types::notification::Notification as _; // for METHOD consts
use lsp_types::request::Request as _;
use lsp_types::{
    CompletionItem,
    CompletionItemKind,
    // capability helpers
    CompletionResponse,
    Diagnostic,
    DiagnosticSeverity,
    DocumentFormattingParams,
    // core
    Position,
    PublishDiagnosticsParams,
    Range,
    TextEdit,
    // notifications
    notification::{PublishDiagnostics},
    // requests
    request::{Completion, Formatting, GotoDefinition},
}; // for METHOD consts

fn handle_notification(
    conn: &Connection,
    note: &lsp_server::Notification,
    docs: &mut HashMap<Uri, String>,
) -> Result<()> {
    match note.method.as_str() {
        DidOpenTextDocument::METHOD => {
            let p: DidOpenTextDocumentParams = serde_json::from_value(note.params.clone())?;
            let uri = p.text_document.uri;
            docs.insert(uri.clone(), p.text_document.text);
            publish_dummy_diag(conn, &uri)?;
        }
        DidChangeTextDocument::METHOD => {
            let p: DidChangeTextDocumentParams = serde_json::from_value(note.params.clone())?;
            if let Some(change) = p.content_changes.into_iter().next() {
                let uri = p.text_document.uri;
                docs.insert(uri.clone(), change.text);
                publish_dummy_diag(conn, &uri)?;
            }
        }
        _ => {}
    }
    Ok(())
}

// =====================================================================
// requests
// =====================================================================

fn handle_request(
    conn: &Connection,
    req: &ServerRequest,
    docs: &mut HashMap<Uri, String>,
) -> Result<()> {
    match req.method.as_str() {
        GotoDefinition::METHOD => {
            send_ok(conn, req.id.clone(), &lsp_types::GotoDefinitionResponse::Array(Vec::new()))?;
        }
        Completion::METHOD => {
            let item = CompletionItem {
                label: "HelloFromLSP".into(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("dummy completion".into()),
                ..Default::default()
            };
            send_ok(conn, req.id.clone(), &CompletionResponse::Array(vec![item]))?;
        }
        HoverRequest::METHOD => {
            let hover = Hover {
                contents: HoverContents::Scalar(MarkedString::String(
                    "Hello from *minimal_lsp*".into(),
                )),
                range: None,
            };
            send_ok(conn, req.id.clone(), &hover)?;
        }
        _ => send_err(
            conn,
            req.id.clone(),
            lsp_server::ErrorCode::MethodNotFound,
            "unhandled method",
        )?,
    }
    Ok(())
}

// =====================================================================
// diagnostics
// =====================================================================
fn publish_dummy_diag(conn: &Connection, uri: &Uri) -> Result<()> {
    let diag = Diagnostic {
        range: Range::new(Position::new(0, 0), Position::new(0, 1)),
        severity: Some(DiagnosticSeverity::INFORMATION),
        code: None,
        code_description: None,
        source: Some("minimal_lsp".into()),
        message: "dummy diagnostic".into(),
        related_information: None,
        tags: None,
        data: None,
    };
    let params =
        PublishDiagnosticsParams { uri: uri.clone(), diagnostics: vec![diag], version: None };
    conn.sender.send(Message::Notification(lsp_server::Notification::new(
        PublishDiagnostics::METHOD.to_owned(),
        params,
    )))?;
    Ok(())
}

// =====================================================================
// helpers
// =====================================================================

fn full_range(text: &str) -> Range {
    let last_line_idx = text.lines().count().saturating_sub(1) as u32;
    let last_col = text.lines().last().map_or(0, |l| l.chars().count()) as u32;
    Range::new(Position::new(0, 0), Position::new(last_line_idx, last_col))
}

fn send_ok<T: serde::Serialize>(conn: &Connection, id: RequestId, result: &T) -> Result<()> {
    let resp = Response { id, result: Some(serde_json::to_value(result)?), error: None };
    conn.sender.send(Message::Response(resp))?;
    Ok(())
}

fn send_err(
    conn: &Connection,
    id: RequestId,
    code: lsp_server::ErrorCode,
    msg: &str,
) -> Result<()> {
    let resp = Response {
        id,
        result: None,
        error: Some(lsp_server::ResponseError {
            code: code as i32,
            message: msg.into(),
            data: None,
        }),
    };
    conn.sender.send(Message::Response(resp))?;
    Ok(())
}