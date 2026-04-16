# Concurrency and Latency Notes

## Runtime
- Axum + Tokio for async I/O and high connection concurrency.
- Postgres connection pool capped to prevent DB overload.

## Why no mutex around transitions?
A process-local mutex does not protect across replicas. We rely on Postgres row-level locking to preserve ordering globally.

## <50ms lookup target
- Index on `(state, updated_at)` for operational dashboards.
- Query by primary key for transaction lookups.
- Keep state payloads narrow and append-only audit trail in separate table.
