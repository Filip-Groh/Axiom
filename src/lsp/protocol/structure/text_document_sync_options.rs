use serde::{Deserialize, Serialize};
use crate::lsp::protocol::structure::TextDocumentSyncType;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentSyncOptions {
    pub open_close: Option<bool>,
    pub change: Option<TextDocumentSyncType>
}