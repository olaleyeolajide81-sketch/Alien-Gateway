use soroban_sdk::{contracttype, BytesN, Env};

/// Event structure for a scheduled payment.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchedPayEvent {
    /// The unique identifier of the payment.
    pub payment_id: u32,
    /// The source vault commitment.
    pub from: BytesN<32>,
    /// The destination commitment.
    pub to: BytesN<32>,
    /// The amount scheduled.
    pub amount: i128,
    /// The timestamp for release.
    pub release_at: u64,
}

pub struct EscrowEvents;

impl EscrowEvents {
    /// Emits a scheduled payment event.
    ///
    /// This uses the `publish` method with `#[allow(deprecated)]` to maintain
    /// compatibility with the current SDK while the transition to `#[contractevent]` is finalized.
    pub fn emit_sched_pay(
        env: &Env,
        payment_id: u32,
        from: BytesN<32>,
        to: BytesN<32>,
        amount: i128,
        release_at: u64,
    ) {
        let topics = ("SCHED_PAY", payment_id);
        #[allow(deprecated)]
        env.events().publish(
            topics,
            SchedPayEvent {
                payment_id,
                from,
                to,
                amount,
                release_at,
            },
        );
    }
}
