use crate::{
    providers::llm_api::{CompletionRequest, LlmClientApi},
    server::LlmLanguageServer,
};
use async_lsp::{
    lsp_types::{
        CodeActionParams, CodeActionProviderCapability, CodeActionResponse, CompletionOptions,
        CompletionParams, CompletionResponse, DidChangeConfigurationParams,
        DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
        ExecuteCommandOptions, InitializeParams, InitializeResult, ServerCapabilities,
        TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions, Url,
    },
    LanguageServer, ResponseError,
};
use futures::future::BoxFuture;
use ropey::Rope;
use std::ops::ControlFlow;
use tracing::info;

impl<T> LanguageServer for LlmLanguageServer<'_, T>
where
    T: LlmClientApi,
{
    type Error = ResponseError;
    type NotifyResult = ControlFlow<async_lsp::Result<()>>;

    fn initialize(
        &mut self,
        params: InitializeParams,
    ) -> BoxFuture<'static, Result<InitializeResult, Self::Error>> {
        let trigger_characters = self
            .trigger_characters
            .iter()
            .map(|&s| (*s).into())
            .collect();
        let commands = self.commands.iter().map(|c| c.key.to_owned()).collect();
        let unknown = "unknown".to_owned();
        if let Some(client_info) = params.client_info {
            let client_version = client_info.version.unwrap_or(unknown);
            self.state
                .update_client_info(client_info.name, client_version);
        } else {
            self.state.update_client_info("web".to_owned(), unknown);
        };
        Box::pin(async move {
            Ok(InitializeResult {
                capabilities: ServerCapabilities {
                    code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                    completion_provider: Some(CompletionOptions {
                        resolve_provider: Some(false),
                        trigger_characters: Some(trigger_characters),
                        ..Default::default()
                    }),
                    execute_command_provider: Some(ExecuteCommandOptions {
                        commands,
                        ..Default::default()
                    }),
                    text_document_sync: Some(TextDocumentSyncCapability::Options(
                        TextDocumentSyncOptions {
                            change: Some(TextDocumentSyncKind::INCREMENTAL),
                            open_close: Some(true),
                            ..Default::default()
                        },
                    )),
                    ..Default::default()
                },
                server_info: None,
            })
        })
    }

    fn did_change_configuration(
        &mut self,
        _: DidChangeConfigurationParams,
    ) -> ControlFlow<async_lsp::Result<()>> {
        ControlFlow::Continue(())
    }

    fn did_open(&mut self, params: DidOpenTextDocumentParams) -> Self::NotifyResult {
        let uri = params.text_document.uri;
        let content = Rope::from(params.text_document.text);
        let language_id = params.text_document.language_id;

        self.state.upsert_file(&uri, content, Some(language_id));
        ControlFlow::Continue(())
    }

    fn did_change(&mut self, params: DidChangeTextDocumentParams) -> Self::NotifyResult {
        let uri = params.text_document.uri;

        if !params.content_changes.is_empty() {
            self.state.change_file(&uri, params.content_changes);
        }
        ControlFlow::Continue(())
    }

    fn did_close(&mut self, params: DidCloseTextDocumentParams) -> Self::NotifyResult {
        let uri = params.text_document.uri;
        self.state.remove_file(&uri);
        ControlFlow::Continue(())
    }

    fn completion(
        &mut self,
        params: CompletionParams,
    ) -> BoxFuture<'static, Result<Option<CompletionResponse>, ResponseError>> {
        if let Some(context) = params.context {
            if let Some(_trigger_character) = context.trigger_character {
                // info!("trigger: {trigger_character}");
            }
        }
        let filepath = params
            .text_document_position
            .text_document
            .uri
            .path()
            .to_owned();
        let position = params.text_document_position.position;
        let contents = self
            .state
            .get_contents(&Url::from_file_path(&filepath).unwrap());
        let language_id = self
            .state
            .get_language_id(&Url::from_file_path(&filepath).unwrap());
        self.llm_client.completion(CompletionRequest {
            contents,
            filepath,
            language_id,
            position,
            suggestions: 3,
            client_name: self.state.client_info.name.clone(),
            client_version: self.state.client_info.version.clone(),
        })
    }

    fn code_action(
        &mut self,
        _params: CodeActionParams,
    ) -> BoxFuture<'static, Result<Option<CodeActionResponse>, ResponseError>> {
        Box::pin(async move { Ok(None) })
    }

    fn shutdown(&mut self, _: ()) -> BoxFuture<'static, Result<(), ResponseError>> {
        info!("shutdown...");
        Box::pin(async move { Ok(()) })
    }
}
