use crate::utils::{add_bytes_to_tx, add_shared_object};
use sui_sdk_types::Argument;
use sui_transaction_builder::{Function, TransactionBuilder};

use crate::pyth::UpdatePriceFeeds;
use anyhow::Result;

/// Returns VAA
pub(crate) fn verify_vaas(tx: &mut TransactionBuilder, req: &UpdatePriceFeeds) -> Result<Argument> {
    let wormhole_state_arg = add_shared_object(tx, req.wormhole_state, false)?;

    let vaa_arg = add_bytes_to_tx(tx, req.vaa);

    let function = Function::new(
        req.wormhole_package,
        "vaa".parse()?,
        "parse_and_verify".parse()?,
        vec![],
    );

    let result = tx.move_call(function, vec![wormhole_state_arg, vaa_arg, req.clock]);

    Ok(result)
}
