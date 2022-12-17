mod apis;
mod init;
mod middleware;
mod model;
mod router;
mod service;

use anyhow::Result;
use poem::{listener::TcpListener, middleware::AddData, EndpointExt, Server};
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // ========================= init =========================
    init::init().unwrap_or_else(|err| {
        tracing::error!("{}", err);
    });

    let port_str = std::env::var("PORT")?;

    // server start...
    Server::new(TcpListener::bind(format!("127.0.0.1:{port_str}")))
        .run_with_graceful_shutdown(
            router::init_router()
                .with(middleware::LogMiddleware)
                .with(middleware::ResponseMiddleware)
                .with(AddData::new(init::init_database().await?))
                .with(AddData::new(init::init_redis().await?)),
            async move {
                let _ = tokio::signal::ctrl_c().await;
            },
            Some(Duration::from_secs(5)),
        )
        .await?;

    Ok(())
}
