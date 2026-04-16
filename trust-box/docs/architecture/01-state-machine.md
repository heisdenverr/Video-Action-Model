# Trust-Box State Machine

## Why strict states?
Escrow systems fail catastrophically when release logic is ambiguous. We encode states as a database enum and allow transitions only through one atomic service function.

```text
CREATED -> FUNDED -> IN_PROGRESS -> (COMPLETED | DISPUTED)
COMPLETED -> RELEASED
```

## Atomic transition strategy
- `SELECT ... FOR UPDATE` on transaction row.
- Validate transition in Rust domain logic.
- Update transaction + append immutable audit entry in the same SQL transaction.
- Commit once all writes succeed.

This removes race conditions where two clients click accept/dispute simultaneously.
