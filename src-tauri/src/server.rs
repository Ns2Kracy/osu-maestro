use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::Router;
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, cors::CorsLayer};

use crate::graceful_shutdown::shutdown_signal;

pub async fn init_server() -> anyhow::Result<()> {
    let app = Router::new()
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive());

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let listener = TcpListener::bind(address).await?;

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
