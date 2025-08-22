mod message;
mod protocol;

use std::collections::HashMap;
use std::io::{stdin, stdout, BufRead, BufReader, Read, Write};
use std::fs;
use serde_json::Value;
use crate::lsp::message::Request;
use crate::lsp::protocol::{InitializeRequest, InitializeResponse};
use crate::lsp::protocol::structure::{ServerCapabilities, TextDocumentSyncOptions};

pub struct LSP {

}

impl LSP {
    pub fn new() -> LSP {
        LSP {

        }
    }

    pub fn start(&mut self) {
        loop {
            let mut reader = BufReader::new(stdin());

            let mut headers = HashMap::new();
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();

                if line.trim().is_empty() {
                    break;
                }

                if let Some((key, value)) = line.split_once(':') {
                    headers.insert(key.trim().to_string(), value.trim().to_string());
                }
            }

            let mut body_buffer = vec![0; headers.get("Content-Length").unwrap().parse::<usize>().unwrap()];
            reader.read_exact(&mut body_buffer).unwrap();

            let body = String::from_utf8(body_buffer).unwrap();

            fs::write("/mnt/d/Projects/Axiom/lsp_input.txt", &body).unwrap();

            let parsed_body: Request = serde_json::from_str(&*body).unwrap();

            match parsed_body.method.as_str() {
                "initialize" => {
                    let initialize_request: InitializeRequest = serde_json::from_str(&*body).unwrap();
                    let initialize_response = InitializeResponse {
                        capabilities: ServerCapabilities {
                            text_document_sync: Some(TextDocumentSyncOptions {
                                open_close: Some(true),
                                change: Some(1)
                            })
                        },
                        server_info: None
                    };

                    let parsed_response = serde_json::to_string(&initialize_response).unwrap();

                    stdout().write_all(&*parsed_response.into_bytes()).unwrap();
                    stdout().flush().unwrap();
                },
                _ => panic!()
            }

            stdout().write_all(&*body.into_bytes()).unwrap();
            stdout().flush().unwrap();
        }
    }
}