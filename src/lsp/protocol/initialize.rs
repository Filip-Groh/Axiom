use serde::{Deserialize, Serialize};
use crate::lsp::protocol::structure::{ClientCapabilities, DocumentUri, ServerCapabilities, TraceValue, WorkspaceFolder};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeRequest {
    pub id: i32,
    pub method: String,
    pub params: InitializeParams
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResponse {
    pub capabilities: ServerCapabilities,
    pub server_info: Option<ServerInfo>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    pub process_id: i32,
    pub client_info: Option<ClientInfo>,
    pub locale: Option<String>,
    pub root_path: Option<String>,
    pub root_uri: Option<DocumentUri>,
    pub capabilities: ClientCapabilities,
    pub trace: Option<TraceValue>,
    pub workspace_folders: Option<Vec<WorkspaceFolder>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    pub name: String,
    pub version: Option<String>,
}