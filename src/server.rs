use crate::{
    configs::{Command, LspConfig},
    providers::llm_api::LlmClientApi,
    state::LanguageState,
};
use async_lsp::{
    client_monitor::ClientProcessMonitorLayer, concurrency::ConcurrencyLayer,
    panic::CatchUnwindLayer, router::Router, server::LifecycleLayer, tracing::TracingLayer,
    ClientSocket,
};
use std::{ops::ControlFlow, time::Duration};
use tower::ServiceBuilder;
use tracing::Level;

pub struct LlmLanguageServer<'a, T>
where
    T: LlmClientApi,
{
    #[allow(unused)]
    pub client: ClientSocket,
    pub commands: Vec<Command<'a>>,
    pub trigger_characters: Vec<&'a str>,
    pub state: LanguageState,
    pub llm_client: T,
}

pub struct TickEvent;

impl<'a, T: LlmClientApi + 'static> LlmLanguageServer<'a, T> {
    pub fn new_router(
        client: ClientSocket,
        lsp_config: LspConfig<'a>,
        llm_client: T,
    ) -> Router<Self> {
        let mut router = Router::from_language_server(Self {
            client,
            commands: lsp_config.commands,
            trigger_characters: lsp_config.trigger_characters,
            state: LanguageState::new(),
            llm_client,
        });
        router.event(Self::on_tick);
        router
    }

    fn on_tick(&mut self, _: TickEvent) -> ControlFlow<async_lsp::Result<()>> {
        ControlFlow::Continue(())
    }

    pub async fn run(llm_client: T)
    where
        T: LlmClientApi,
    {
        let lsp_config = LspConfig::init();

        let (server, _) = async_lsp::MainLoop::new_server(|client| {
            tokio::spawn({
                let client = client.clone();
                async move {
                    let mut interval = tokio::time::interval(Duration::from_secs(1));
                    loop {
                        interval.tick().await;
                        if client.emit(TickEvent).is_err() {
                            break;
                        }
                    }
                }
            });

            ServiceBuilder::new()
                .layer(TracingLayer::default())
                .layer(LifecycleLayer::default())
                .layer(CatchUnwindLayer::default())
                .layer(ConcurrencyLayer::default())
                .layer(ClientProcessMonitorLayer::new(client.clone()))
                .service(LlmLanguageServer::new_router(
                    client, lsp_config, llm_client,
                ))
        });

        tracing_subscriber::fmt()
            .with_max_level(Level::INFO)
            .without_time()
            .with_ansi(false)
            .with_writer(std::io::stderr)
            .init();

        // Prefer truly asynchronous piped stdin/stdout without blocking tasks.
        #[cfg(unix)]
        let (stdin, stdout) = (
            async_lsp::stdio::PipeStdin::lock_tokio().unwrap(),
            async_lsp::stdio::PipeStdout::lock_tokio().unwrap(),
        );
        // Fallback to spawn blocking read/write otherwise.
        #[cfg(not(unix))]
        let (stdin, stdout) = (
            tokio_util::compat::TokioAsyncReadCompatExt::compat(tokio::io::stdin()),
            tokio_util::compat::TokioAsyncWriteCompatExt::compat_write(tokio::io::stdout()),
        );

        server.run_buffered(stdin, stdout).await.unwrap();
    }
}
