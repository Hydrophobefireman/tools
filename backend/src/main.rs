mod err;
mod ip_handler;

use axum::{
    body::{Body, BoxBody},
    extract::{ConnectInfo, Query},
    http::{HeaderMap, Request, Response},
    routing::get,
    Json, Router,
};
use ip_handler::{fetch_ip_details, IPInfo, IPPayload};
use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
    time::Duration,
};
use tower_http::trace::TraceLayer;
use tracing::Span;

// basic handler that responds with a static string
async fn get_client_ip_details(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> axum::response::Result<Json<IPInfo>> {
    let maybe_ip = headers
        .get("Fly-Client-IP")
        .map(|x| IpAddr::from_str(x.to_str().unwrap()));
    fetch_ip_details(maybe_ip.unwrap_or_else(|| Ok(addr.ip())).unwrap())
}

async fn get_ip_details_from_q(
    Query(payload): Query<IPPayload>,
) -> axum::response::Result<Json<IPInfo>> {
    fetch_ip_details(IpAddr::from_str(&payload.ip).unwrap())
}

async fn get_client_ip_details_raw(
    conn: ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> axum::response::Result<String> {
    let x = get_client_ip_details(conn, headers).await?;
    Ok(x.ip.clone())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    // Spawn a task to gracefully shutdown server.

    let app = Router::new()
        .route("/ip", get(get_client_ip_details))
        .route("/ip/q", get(get_ip_details_from_q))
        .route("/ip/raw", get(get_client_ip_details_raw))
        .layer(
            TraceLayer::new_for_http()
                .on_request(|request: &Request<Body>, _span: &Span| {
                    let query = match request.uri().query() {
                        Some(q) => format!("?{q}"),
                        _ => "".into(),
                    };
                    let remote_addr = request
                        .headers()
                        .get("Fly-Client-Ip")
                        .map(|x| x.to_str().unwrap().to_string())
                        .unwrap_or_else(|| {
                            request
                                .extensions()
                                .get::<ConnectInfo<SocketAddr>>()
                                .map(|x| x.to_string())
                                .unwrap()
                        });

                    let headers = request.headers();
                    let user_agent = match headers.get("user-agent") {
                        Some(agent) => agent.to_str().unwrap(),
                        _ => "unknown",
                    };
                    tracing::info!(
                        "request: {} {} {}{} - {}",
                        remote_addr,
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
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
