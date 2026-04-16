# Trust-Box

Production-oriented MVP scaffold for a micro-escrow SaaS targeting Nigerian freelancer-client transactions.

## Stack
- Backend: Rust + Axum + SQLx + JWT + Utoipa Swagger
- Frontend: Next.js 14 App Router + Tailwind + Zod
- DB: PostgreSQL (schema and migrations included)

## Escrow State Machine
`CREATED -> FUNDED -> IN_PROGRESS -> (COMPLETED | DISPUTED) -> RELEASED`

Transitions are enforced in `transition_state_atomic` with row-level SQL lock + audit insert in one ACID transaction.

## Run locally
```bash
cd trust-box
docker compose up --build
```

- API: http://localhost:8080
- Swagger: http://localhost:8080/docs
- Web app: http://localhost:3000

## Key endpoints
- `POST /auth/signup`
- `POST /auth/login`
- `POST /transactions`
- `POST /transactions/:id/upload`
- `POST /transactions/:id/accept`
- `POST /transactions/:id/dispute`
- `POST /webhooks/paystack`

## Notes
- Payment provider integration is webhook-first for reliable state confirmation.
- Audit trail signs each state change hash for tamper-evident logging.
- Fee range constrained to 2.5%–5% (250-500 bps).
