CREATE TYPE transaction_state AS ENUM (
  'CREATED',
  'FUNDED',
  'IN_PROGRESS',
  'DISPUTED',
  'COMPLETED',
  'RELEASED'
);

CREATE TYPE user_role AS ENUM ('freelancer', 'client', 'admin');

CREATE TABLE users (
  id UUID PRIMARY KEY,
  email TEXT UNIQUE NOT NULL,
  password_hash TEXT NOT NULL,
  role user_role NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE transactions (
  id UUID PRIMARY KEY,
  freelancer_id UUID NOT NULL,
  client_id UUID,
  amount_kobo BIGINT NOT NULL CHECK (amount_kobo > 0),
  fee_bps INTEGER NOT NULL CHECK (fee_bps BETWEEN 250 AND 500),
  delivery_description TEXT NOT NULL,
  payment_reference TEXT,
  final_file_url TEXT,
  state transaction_state NOT NULL,
  state_signed_hash TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE state_audit_log (
  id UUID PRIMARY KEY,
  transaction_id UUID NOT NULL REFERENCES transactions(id),
  from_state transaction_state,
  to_state transaction_state NOT NULL,
  actor_id UUID,
  actor_type TEXT NOT NULL,
  signed_hash TEXT NOT NULL,
  occurred_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_transactions_state_updated_at ON transactions(state, updated_at DESC);
CREATE INDEX idx_state_audit_transaction_id ON state_audit_log(transaction_id, occurred_at DESC);
