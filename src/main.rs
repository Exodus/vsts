use axum::{
    handler::Handler,
    http::StatusCode,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[macro_use(lazy_static)]
extern crate lazy_static;

mod error;
mod model;
mod settings;
use settings::CONFIG;
mod handler;

#[tokio::main]
async fn main() {
    // Setup Debugging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("LOG_LEVEL").unwrap_or_else(|_| "LOG_LEVEL=info,tower_http=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let app = Router::new()
        .fallback(fallback.into_service())
        .route("/healthz", get(handler::healthz))
        .route("/gen", get(handler::create_jwt))
        .route("/auth", get(handler::auth_with_header))
        .route("/auth/:token", get(handler::auth_with_path))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .include_headers(true)
                        .level(tracing::Level::INFO),
                )
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        );

    runserver(app).await;
}

async fn runserver(routes: axum::routing::Router) {
    let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.server.port));
    // tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .with_graceful_shutdown(signal_shutdown())
        .await
        .unwrap()
}

async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route to {uri}"))
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn signal_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}
