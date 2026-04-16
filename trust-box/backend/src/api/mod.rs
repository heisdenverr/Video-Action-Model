use crate::{
    auth::{self, AuthUser},
    db::AppState,
    domain::{Transaction, TransactionState, UserRole},
    services::{self, TransitionRequest},
};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct AuthRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub role: Option<UserRole>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateTransactionRequest {
    pub amount_kobo: i64,
    #[validate(length(min = 10, max = 500))]
    pub delivery_description: String,
    pub fee_bps: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ClientActionRequest {
    pub actor_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct WebhookEvent {
    pub event: String,
    pub reference: String,
    pub metadata: serde_json::Value,
}

#[utoipa::path(get, path = "/health", responses((status = 200, body = String)))]
pub async fn health() -> &'static str {
    "ok"
}

#[utoipa::path(post, path = "/auth/signup", request_body = AuthRequest, responses((status = 201, body = AuthResponse)))]
pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    payload
        .validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let user_id = Uuid::new_v4();
    let hash = auth::hash_password(&payload.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let role = payload.role.unwrap_or(UserRole::Freelancer);

    let role_db = match role {
        UserRole::Freelancer => "freelancer",
        UserRole::Client => "client",
        UserRole::Admin => "admin",
    };

    sqlx::query(
        "INSERT INTO users (id, email, password_hash, role) VALUES ($1, $2, $3, $4::user_role)",
    )
    .bind(user_id)
    .bind(payload.email)
    .bind(hash)
    .bind(role_db)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let role_name = match role {
        UserRole::Freelancer => "freelancer",
        UserRole::Client => "client",
        UserRole::Admin => "admin",
    };
    let token = auth::issue_token(user_id, role_name, &state.jwt_secret)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::CREATED, Json(AuthResponse { token })))
}

#[utoipa::path(post, path = "/auth/login", request_body = AuthRequest, responses((status = 200, body = AuthResponse)))]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let row = sqlx::query(
        "SELECT id, password_hash, role::text AS role_text FROM users WHERE email = $1",
    )
    .bind(payload.email)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| (StatusCode::UNAUTHORIZED, "invalid credentials".to_owned()))?;

    let hash: String = row.get("password_hash");
    if !auth::verify_password(&payload.password, &hash) {
        return Err((StatusCode::UNAUTHORIZED, "invalid credentials".to_owned()));
    }

    let user_id: Uuid = row.get("id");
    let role_text: String = row.get("role_text");
    let token = auth::issue_token(user_id, &role_text, &state.jwt_secret)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse { token }))
}

#[utoipa::path(post, path = "/transactions", request_body = CreateTransactionRequest, responses((status = 201, body = Transaction)))]
pub async fn create_transaction(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateTransactionRequest>,
) -> Result<(StatusCode, Json<Transaction>), (StatusCode, String)> {
    payload
        .validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    if auth_user.0.role != "freelancer" {
        return Err((
            StatusCode::FORBIDDEN,
            "only freelancers can create links".to_owned(),
        ));
    }

    let tx_id = Uuid::new_v4();
    let now = Utc::now();
    let signed_hash =
        services::kernel_sign(tx_id, None, TransactionState::Created, "freelancer", now);

    sqlx::query("INSERT INTO transactions (id, freelancer_id, amount_kobo, fee_bps, delivery_description, state, state_signed_hash, created_at, updated_at) VALUES ($1,$2,$3,$4,$5,$6::transaction_state,$7,$8,$9)")
        .bind(tx_id)
        .bind(auth_user.0.sub)
        .bind(payload.amount_kobo)
        .bind(payload.fee_bps)
        .bind(payload.delivery_description.clone())
        .bind("CREATED")
        .bind(signed_hash.clone())
        .bind(now)
        .bind(now)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let tx = Transaction {
        id: tx_id,
        freelancer_id: auth_user.0.sub,
        client_id: None,
        amount_kobo: payload.amount_kobo,
        fee_bps: payload.fee_bps,
        delivery_description: payload.delivery_description,
        payment_reference: None,
        final_file_url: None,
        state: TransactionState::Created,
        state_signed_hash: signed_hash,
        created_at: now,
        updated_at: now,
    };

    Ok((StatusCode::CREATED, Json(tx)))
}

#[utoipa::path(post, path = "/transactions/{id}/upload", request_body = String, responses((status = 200)))]
pub async fn upload_final_file(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(file_url): Json<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("UPDATE transactions SET final_file_url = $2 WHERE id = $1")
        .bind(id)
        .bind(file_url)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    services::transition_state_atomic(
        &state.pool,
        TransitionRequest {
            transaction_id: id,
            actor_id: None,
            actor_type: "freelancer".into(),
            to_state: TransactionState::Completed,
        },
    )
    .await
    .map_err(|e| (StatusCode::CONFLICT, e.to_string()))?;

    Ok(StatusCode::OK)
}

#[utoipa::path(post, path = "/transactions/{id}/accept", request_body = ClientActionRequest, responses((status = 200)))]
pub async fn client_accept(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ClientActionRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    services::transition_state_atomic(
        &state.pool,
        TransitionRequest {
            transaction_id: id,
            actor_id: Some(payload.actor_id),
            actor_type: "client".into(),
            to_state: TransactionState::Released,
        },
    )
    .await
    .map_err(|e| (StatusCode::CONFLICT, e.to_string()))?;

    Ok(StatusCode::OK)
}

#[utoipa::path(post, path = "/transactions/{id}/dispute", request_body = ClientActionRequest, responses((status = 200)))]
pub async fn dispute(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ClientActionRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    services::transition_state_atomic(
        &state.pool,
        TransitionRequest {
            transaction_id: id,
            actor_id: Some(payload.actor_id),
            actor_type: "client".into(),
            to_state: TransactionState::Disputed,
        },
    )
    .await
    .map_err(|e| (StatusCode::CONFLICT, e.to_string()))?;
    Ok(StatusCode::OK)
}

#[utoipa::path(post, path = "/webhooks/paystack", request_body = WebhookEvent, responses((status = 200)))]
pub async fn paystack_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(event): Json<WebhookEvent>,
) -> Result<StatusCode, (StatusCode, String)> {
    let signature = headers
        .get("x-paystack-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "missing signature".to_owned()))?;

    let payload =
        serde_json::to_string(&event).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    if !services::verify_paystack_signature(&state.webhook_secret, &payload, signature) {
        return Err((StatusCode::UNAUTHORIZED, "invalid signature".to_owned()));
    }

    if event.event == "charge.success" {
        let tx_id = event
            .metadata
            .get("transaction_id")
            .and_then(|v| v.as_str())
            .ok_or((StatusCode::BAD_REQUEST, "transaction_id missing".to_owned()))?;
        let tx_id = Uuid::parse_str(tx_id).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

        services::transition_state_atomic(
            &state.pool,
            TransitionRequest {
                transaction_id: tx_id,
                actor_id: None,
                actor_type: "webhook".into(),
                to_state: TransactionState::Funded,
            },
        )
        .await
        .map_err(|e| (StatusCode::CONFLICT, e.to_string()))?;

        services::transition_state_atomic(
            &state.pool,
            TransitionRequest {
                transaction_id: tx_id,
                actor_id: None,
                actor_type: "system".into(),
                to_state: TransactionState::InProgress,
            },
        )
        .await
        .map_err(|e| (StatusCode::CONFLICT, e.to_string()))?;
    }

    Ok(StatusCode::OK)
}
