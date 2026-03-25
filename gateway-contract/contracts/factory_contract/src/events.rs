use soroban_sdk::{symbol_short, Address, BytesN, Env, Symbol};

pub const USERNAME_DEPLOYED: Symbol = symbol_short!("USR_DEP");
pub const OWNERSHIP_TRANSFERRED: Symbol = symbol_short!("OWN_TRF");

pub fn emit_username_deployed(
    env: &Env,
    username_hash: &BytesN<32>,
    owner: &Address,
    registered_at: u64,
) {
    env.events().publish(
        (USERNAME_DEPLOYED,),
        (username_hash.clone(), owner.clone(), registered_at),
    );
}

pub fn emit_ownership_transferred(
    env: &Env,
    username_hash: &BytesN<32>,
    old_owner: &Address,
    new_owner: &Address,
) {
    env.events().publish(
        (OWNERSHIP_TRANSFERRED,),
        (username_hash.clone(), old_owner.clone(), new_owner.clone()),
    );
}
