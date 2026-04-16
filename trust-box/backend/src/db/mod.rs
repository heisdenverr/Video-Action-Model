use crate::services::PaymentGateway;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, sync::Arc};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
    pub webhook_secret: String,
    pub payment: Arc<PaymentGateway>,
}

impl AppState {
    pub async fn from_env() -> anyhow::Result<Self> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/trust_box".to_owned());
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(&database_url)
            .await?;

        Ok(Self {
            pool,
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret".to_owned()),
            webhook_secret: env::var("PAYSTACK_WEBHOOK_SECRET")
                .unwrap_or_else(|_| "dev-webhook-secret".to_owned()),
            payment: Arc::new(PaymentGateway::from_env()),
        })
    }
}
