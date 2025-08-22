import * as vscode from 'vscode'

import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind
} from 'vscode-languageclient/node'

let client: LanguageClient

export function activate(context: vscode.ExtensionContext) {
	console.log("Axiom Language Extension activated!")

	let serverOptions: ServerOptions = {
		command: "/mnt/d/Projects/Axiom/target/debug/Axiom",
		args: ["lsp"]
	}

	let clientOptions: LanguageClientOptions = {
		documentSelector: [
			{
				language: "axiom"
			}
		]
	}

	client = new LanguageClient("Axiom LSP Server", serverOptions, clientOptions)

	client.start()
}

export function deactivate() {
	if (client) {
		client.stop()
	}
}
