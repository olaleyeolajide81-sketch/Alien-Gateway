# Escrow Contract Specification

The Escrow contract facilitates time-locked payments between identity-committed vaults. Funds are reserved at the time of scheduling and released only after a specified ledger timestamp.

## Function: `schedule_payment`

Schedules a transfer of funds from the sender's vault to a recipient.

### Interface
```rust
pub fn schedule_payment(
    env: Env,
    from: BytesN<32>,
    to: BytesN<32>,
    amount: i128,
    release_at: u64,
) -> Result<u32, EscrowError>
```

### Requirements & Validation
- **Authentication**: The caller must provide a valid signature for the `owner` address associated with the `from` vault (`Address::require_auth`).
- **Amount**: Must be strictly positive (`amount > 0`).
- **Balance**: The `from` vault must have enough funds (`balance >= amount`).
- **Timing (Scheduling)**: The `release_at` value must be strictly greater than the current ledger timestamp (`release_at > env.ledger().timestamp()`). 
- **Timing (Execution)**: Execution is permitted only when the current ledger timestamp is equal to or greater than the `release_at` value (`now >= release_at`).

### State Changes
1. **Balance Reservation**: The requested `amount` is immediately deducted from the `from` vault's `VaultState.balance`.
2. **ID Generation**: A unique `payment_id` is generated using a global auto-incrementing counter.
3. **Storage**: A `ScheduledPayment` record is created in persistent storage with `executed: false`.

### Implementation Details
- **Vaults**: Stored in persistent storage indexed by `DataKey::Vault(BytesN<32>)`.
- **Payments**: Stored in persistent storage indexed by `DataKey::ScheduledPayment(u32)`.
- **Counter**: Maintained in instance storage at `DataKey::PaymentCounter`.
- **Events**: Emits a `SCHED_PAY` event (topics: `SCHED_PAY`, `payment_id`).

### Security Considerations
- **Reentrancy**: Not applicable as no external calls are made during scheduling.
- **Authorization**: Hardened by host-level `require_auth` on the vault owner.
- **Overflow**: Payment counter increments are protected by `checked_add` with explicit error handling.
- **Fairness**: Funds are locked immediately upon scheduling, preventing double-spending from the same vault balance.
