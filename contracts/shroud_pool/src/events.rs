use soroban_sdk::{Address, BytesN, Env, Symbol};

pub(crate) fn publish_deposit(
    env: &Env,
    asset_id: Address,
    depositor: Address,
    amount: i128,
    commitment: BytesN<32>,
    new_root: BytesN<32>,
) {
    env.events().publish(
        (Symbol::new(env, "deposit"), asset_id),
        (depositor, amount, commitment, new_root),
    );
}

pub(crate) fn publish_transfer(
    env: &Env,
    nullifier: BytesN<32>,
    output_commitment: BytesN<32>,
    new_root: BytesN<32>,
) {
    env.events().publish(
        (Symbol::new(env, "transfer"), nullifier),
        (output_commitment, new_root),
    );
}

pub(crate) fn publish_withdraw(
    env: &Env,
    asset_id: Address,
    recipient: Address,
    amount: i128,
    nullifier: BytesN<32>,
) {
    env.events().publish(
        (Symbol::new(env, "withdraw"), asset_id),
        (recipient, amount, nullifier),
    );
}

pub(crate) fn publish_paused(env: &Env, paused: bool) {
    env.events()
        .publish((Symbol::new(env, "paused"),), paused);
}
