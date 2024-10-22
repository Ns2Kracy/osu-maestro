use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::Router;
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};

use crate::utils::graceful_shutdown::shutdown_signal;

pub async fn init_server() -> anyhow::Result<()> {
    let app = Router::new()
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        );

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6500);
    let listener = TcpListener::bind(address).await?;

    tracing::info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
