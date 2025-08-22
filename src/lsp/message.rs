use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub id: i32,
    pub method: String,
    pub params: Value
}