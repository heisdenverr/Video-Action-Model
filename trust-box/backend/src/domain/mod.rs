use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Type, PartialEq, Eq)]
#[sqlx(type_name = "transaction_state", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionState {
    Created,
    Funded,
    InProgress,
    Disputed,
    Completed,
    Released,
}

impl TransactionState {
    pub fn can_transition(self, to: TransactionState) -> bool {
        matches!(
            (self, to),
            (TransactionState::Created, TransactionState::Funded)
                | (TransactionState::Funded, TransactionState::InProgress)
                | (TransactionState::InProgress, TransactionState::Completed)
                | (TransactionState::InProgress, TransactionState::Disputed)
                | (TransactionState::Completed, TransactionState::Released)
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Freelancer,
    Client,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Transaction {
    pub id: Uuid,
    pub freelancer_id: Uuid,
    pub client_id: Option<Uuid>,
    pub amount_kobo: i64,
    pub fee_bps: i32,
    pub delivery_description: String,
    pub payment_reference: Option<String>,
    pub final_file_url: Option<String>,
    pub state: TransactionState,
    pub state_signed_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub from_state: Option<TransactionState>,
    pub to_state: TransactionState,
    pub actor_id: Option<Uuid>,
    pub actor_type: String,
    pub signed_hash: String,
    pub occurred_at: DateTime<Utc>,
}
