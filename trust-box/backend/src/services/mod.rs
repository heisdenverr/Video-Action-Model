use crate::domain::{AuditEntry, TransactionState};
use chrono::Utc;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Postgres, Row, Transaction};
use uuid::Uuid;

type HmacSha512 = Hmac<sha2::Sha512>;

#[derive(Clone)]
pub struct PaymentGateway {
    pub provider: String,
    pub secret_key: String,
}

impl PaymentGateway {
    pub fn from_env() -> Self {
        Self {
            provider: std::env::var("PAYMENT_PROVIDER").unwrap_or_else(|_| "paystack".into()),
            secret_key: std::env::var("PAYSTACK_SECRET_KEY").unwrap_or_else(|_| "sk_test".into()),
        }
    }
}

pub fn verify_paystack_signature(secret: &str, payload: &str, signature: &str) -> bool {
    let mut mac = HmacSha512::new_from_slice(secret.as_bytes()).expect("valid key");
    mac.update(payload.as_bytes());
    hex::encode(mac.finalize().into_bytes()) == signature
}

pub fn kernel_sign(
    transaction_id: Uuid,
    from: Option<TransactionState>,
    to: TransactionState,
    actor: &str,
    occurred_at: chrono::DateTime<Utc>,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(transaction_id.as_bytes());
    hasher.update(format!("{:?}", from).as_bytes());
    hasher.update(format!("{:?}", to).as_bytes());
    hasher.update(actor.as_bytes());
    hasher.update(occurred_at.timestamp_millis().to_le_bytes());
    hex::encode(hasher.finalize())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransitionRequest {
    pub transaction_id: Uuid,
    pub actor_id: Option<Uuid>,
    pub actor_type: String,
    pub to_state: TransactionState,
}

pub async fn transition_state_atomic(
    pool: &PgPool,
    req: TransitionRequest,
) -> anyhow::Result<AuditEntry> {
    let mut tx: Transaction<'_, Postgres> = pool.begin().await?;

    let row = sqlx::query("SELECT state::text AS state FROM transactions WHERE id = $1 FOR UPDATE")
        .bind(req.transaction_id)
        .fetch_one(&mut *tx)
        .await?;
    let from_state = parse_state(row.get::<String, _>("state").as_str())?;

    if !from_state.can_transition(req.to_state.clone()) {
        anyhow::bail!(
            "invalid state transition: {:?} -> {:?}",
            from_state,
            req.to_state
        );
    }

    let now = Utc::now();
    let signed_hash = kernel_sign(
        req.transaction_id,
        Some(from_state.clone()),
        req.to_state.clone(),
        &req.actor_type,
        now,
    );

    sqlx::query("UPDATE transactions SET state = $2::transaction_state, updated_at = $3, state_signed_hash = $4 WHERE id = $1")
    .bind(req.transaction_id)
    .bind(format_state(&req.to_state))
    .bind(now)
    .bind(signed_hash.clone())
    .execute(&mut *tx)
    .await?;

    let audit_id = Uuid::new_v4();
    sqlx::query("INSERT INTO state_audit_log (id, transaction_id, from_state, to_state, actor_id, actor_type, signed_hash, occurred_at) VALUES ($1,$2,$3::transaction_state,$4::transaction_state,$5,$6,$7,$8)")
    .bind(audit_id)
    .bind(req.transaction_id)
    .bind(format_state(&from_state))
    .bind(format_state(&req.to_state))
    .bind(req.actor_id)
    .bind(req.actor_type.clone())
    .bind(signed_hash.clone())
    .bind(now)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(AuditEntry {
        id: audit_id,
        transaction_id: req.transaction_id,
        from_state: Some(from_state),
        to_state: req.to_state,
        actor_id: req.actor_id,
        actor_type: req.actor_type,
        signed_hash,
        occurred_at: now,
    })
}

fn parse_state(value: &str) -> anyhow::Result<TransactionState> {
    match value {
        "CREATED" => Ok(TransactionState::Created),
        "FUNDED" => Ok(TransactionState::Funded),
        "IN_PROGRESS" => Ok(TransactionState::InProgress),
        "DISPUTED" => Ok(TransactionState::Disputed),
        "COMPLETED" => Ok(TransactionState::Completed),
        "RELEASED" => Ok(TransactionState::Released),
        _ => anyhow::bail!("unknown transaction state: {value}"),
    }
}

fn format_state(value: &TransactionState) -> &'static str {
    match value {
        TransactionState::Created => "CREATED",
        TransactionState::Funded => "FUNDED",
        TransactionState::InProgress => "IN_PROGRESS",
        TransactionState::Disputed => "DISPUTED",
        TransactionState::Completed => "COMPLETED",
        TransactionState::Released => "RELEASED",
    }
}
