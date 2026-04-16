mod api;
mod auth;
mod db;
mod domain;
mod services;

use axum::{
    routing::{get, post},
    Router,
};
use db::AppState;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::health,
        api::signup,
        api::login,
        api::create_transaction,
        api::client_accept,
        api::dispute,
        api::upload_final_file,
        api::paystack_webhook,
    ),
    components(schemas(
        domain::UserRole,
        domain::Transaction,
        domain::TransactionState,
        api::AuthRequest,
        api::AuthResponse,
        api::CreateTransactionRequest,
        api::ClientActionRequest,
        api::WebhookEvent,
    )),
    tags((name = "trust-box", description = "Micro-escrow API"))
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState::from_env().await?;
    let app = Router::new()
        .route("/health", get(api::health))
        .route("/auth/signup", post(api::signup))
        .route("/auth/login", post(api::login))
        .route("/transactions", post(api::create_transaction))
        .route("/transactions/:id/upload", post(api::upload_final_file))
        .route("/transactions/:id/accept", post(api::client_accept))
        .route("/transactions/:id/dispute", post(api::dispute))
        .route("/webhooks/paystack", post(api::paystack_webhook))
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    tracing::info!("trust-box backend listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
