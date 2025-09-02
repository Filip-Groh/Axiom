import { exec, spawn } from 'child_process'
import * as vscode from 'vscode'

import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from 'vscode-languageclient/node'

let client: LanguageClient
let terminal: vscode.Terminal | undefined

export function activate(context: vscode.ExtensionContext) {
	console.log("Axiom Language Extension activated!")

	let disposable = vscode.commands.registerCommand('axiom-language-extension.runScript', () => {
		const editor = vscode.window.activeTextEditor
		if (!editor) {
			return
		}

		const document = editor.document
		const path = document.uri.path

		const command = `/mnt/d/Projects/Axiom/target/debug/Axiom run ${path}`

		if (!terminal || terminal.exitStatus) {
			terminal = vscode.window.createTerminal({ name: "Axiom" })
		}
		
		terminal.show(true)
		terminal.sendText(command)
	})

	context.subscriptions.push(disposable)

	let serverOptions: ServerOptions = {
		command: "/mnt/d/Projects/Axiom/target/debug/Axiom",
		args: ["lsp"],
	}

	let clientOptions: LanguageClientOptions = {
		documentSelector: [
			{
				scheme: "file",
				language: "axiom"
			}
		],
		synchronize: {
			fileEvents: vscode.workspace.createFileSystemWatcher("**/.clientrc")
		}
	}

	client = new LanguageClient("axiomLSPServer", "Axiom LSP Server", serverOptions, clientOptions)

	client.start()
}

export function deactivate() {
	if (client) {
		client.stop()
	}
}
