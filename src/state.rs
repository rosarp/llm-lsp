use crate::configs::{Command, LspConfig};
use async_lsp::{router::Router, ClientSocket, LanguageServer, ResponseError};
use futures::future::BoxFuture;
use lsp_types::{
    CodeActionParams, CodeActionResponse, CompletionItem, CompletionOptions, CompletionParams,
    CompletionResponse, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, ExecuteCommandOptions,
};
use lsp_types::{
    DidChangeConfigurationParams, InitializeParams, InitializeResult, ServerCapabilities,
};
use std::ops::ControlFlow;
use tracing::log::info;

pub struct ServerState<'a> {
    client: ClientSocket,
    commands: Vec<Command<'a>>,
    trigger_characters: Vec<&'a str>,
}

impl<'a> LanguageServer for ServerState<'a> {
    type Error = ResponseError;
    type NotifyResult = ControlFlow<async_lsp::Result<()>>;

    fn initialize(
        &mut self,
        _params: InitializeParams,
    ) -> BoxFuture<'static, Result<InitializeResult, Self::Error>> {
        let trigger_characters = self
            .trigger_characters
            .iter()
            .map(|s| (*s).into())
            .collect();
        let commands = self.commands.iter().map(|c| c.key.to_owned()).collect();
        Box::pin(async move {
            Ok(InitializeResult {
                capabilities: ServerCapabilities {
                    completion_provider: Some(CompletionOptions {
                        resolve_provider: Some(false),
                        trigger_characters: Some(trigger_characters),
                        ..Default::default()
                    }),
                    execute_command_provider: Some(ExecuteCommandOptions {
                        commands,
                        ..Default::default()
                    }),
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

    fn did_open(&mut self, _params: DidOpenTextDocumentParams) -> Self::NotifyResult {
        ControlFlow::Continue(())
    }

    fn did_change(&mut self, _params: DidChangeTextDocumentParams) -> Self::NotifyResult {
        ControlFlow::Continue(())
    }

    fn did_close(&mut self, _params: DidCloseTextDocumentParams) -> Self::NotifyResult {
        ControlFlow::Continue(())
    }

    fn completion(
        &mut self,
        params: CompletionParams,
    ) -> BoxFuture<'static, Result<Option<CompletionResponse>, ResponseError>> {
        info!("completion...");
        if let Some(context) = params.context {
            if let Some(trigger_character) = context.trigger_character {
                info!("trigger: {trigger_character}");
            }
        }
        let completion_response = vec![CompletionItem::new_simple(
            "label".to_owned(),
            "description".to_owned(),
        )];
        Box::pin(async move { Ok(Some(CompletionResponse::Array(completion_response))) })
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

pub struct TickEvent;

impl<'a> ServerState<'a> {
    pub fn new_router(client: ClientSocket, lsp_config: LspConfig<'a>) -> Router<Self> {
        let mut router = Router::from_language_server(Self {
            client,
            commands: lsp_config.commands,
            trigger_characters: lsp_config.trigger_characters,
        });
        router.event(Self::on_tick);
        router
    }

    fn on_tick(&mut self, _: TickEvent) -> ControlFlow<async_lsp::Result<()>> {
        ControlFlow::Continue(())
    }
}
