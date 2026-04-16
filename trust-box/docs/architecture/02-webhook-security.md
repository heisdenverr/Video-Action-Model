# Webhook Security Design

## Provider choice
MVP uses Paystack-compatible webhook flow with HMAC SHA-512 signature validation.

## Controls
1. Verify `x-paystack-signature` against raw request payload.
2. Ignore unknown event names.
3. Use idempotent state transition gate (invalid transitions are rejected).
4. Record webhook-driven state change in audit table.

## Risk notes
- Production should include replay protection (`event_id` de-dup key).
- Production should pin provider IP ranges and enforce TLS.
