use serde::{Deserialize, Serialize};
use crate::lsp::protocol::structure::TextDocumentSyncOptions;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
    pub text_document_sync: Option<TextDocumentSyncOptions>
}