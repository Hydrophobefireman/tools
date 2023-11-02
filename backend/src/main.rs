mod err;
mod ip_handler;

use axum::{
    body::{Body, BoxBody},
    extract::{ConnectInfo, Query},
    http::{HeaderMap, Request, Response},
    routing::get,
    Json, Router,
};
use axum_server::Handle;
use ip_handler::{fetch_ip_details, IPInfo, IPPayload};
use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
    time::Duration,
};
use tokio::time::sleep;
use tower_http::trace::TraceLayer;
use tracing::Span;

// basic handler that responds with a static string
async fn root(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> axum::response::Result<Json<IPInfo>> {
    let maybe_ip = headers
        .get("Fly-Client-IP")
        .map(|x| IpAddr::from_str(x.to_str().unwrap()));
    fetch_ip_details(maybe_ip.unwrap_or_else(|| Ok(addr.ip())).unwrap())
}

async fn get_ip_details(Query(payload): Query<IPPayload>) -> axum::response::Result<Json<IPInfo>> {
    fetch_ip_details(IpAddr::from_str(&payload.ip).unwrap())
}

#[tokio::main]
async fn main() {
    let handle = Handle::new();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    // Spawn a task to gracefully shutdown server.
    tokio::spawn(graceful_shutdown(handle.clone()));
    let app = Router::new()
        .route("/ip", get(root))
        .route("/ip/q", get(get_ip_details))
        .layer(
            TraceLayer::new_for_http()
                .on_request(|request: &Request<Body>, _span: &Span| {
                    let query = match request.uri().query() {
                        Some(q) => format!("?{q}"),
                        _ => "".into(),
                    };
                    let headers = request.headers();
                    let user_agent = match headers.get("user-agent") {
                        Some(agent) => agent.to_str().unwrap(),
                        _ => "unknown",
                    };
                    tracing::info!(
                        "request: {} {}{} - {}",
                        request.method(),
                        request.uri().path(),
                        query,
                        user_agent
                    );
                })
                .on_response(
                    |response: &Response<BoxBody>, latency: Duration, _span: &Span| {
                        tracing::info!("response: {} {:?}", response.status(), latency)
                    },
                ),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    axum_server::bind(addr)
        .handle(handle)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn graceful_shutdown(handle: Handle) {
    sleep(Duration::from_secs(600)).await;

    println!("sending graceful shutdown signal");

    // Signal the server to shutdown using Handle.
    handle.graceful_shutdown(Some(Duration::from_secs(30)));

    // Print alive connection count every second.
    loop {
        sleep(Duration::from_secs(1)).await;

        println!("alive connections: {}", handle.connection_count());
    }
}
