use anyhow::Result;
use sui_sdk_types::{Address, Argument, TypeTag};
use sui_transaction_builder::{Function, TransactionBuilder};

use crate::{
    SharedObjectInfo,
    utils::{add_bytes_to_tx, add_shared_object, add_u64, add_u128},
};

pub(crate) struct TidesSwap<'a> {
    pub(crate) tides_package: Address,
    pub(crate) input_type: &'a TypeTag,
    pub(crate) output_type: &'a TypeTag,
    pub(crate) protected_margin_account: &'a SharedObjectInfo,
    pub(crate) lending_market_arg: Argument,
    pub(crate) rfq_account: &'a SharedObjectInfo,
    pub(crate) trade_id: u128,
    pub(crate) nonce: u128,
    pub(crate) expiry_time_unix_ms: u64,
    pub(crate) signature: &'a [u8],
    pub(crate) input_amount: u64,
    pub(crate) output_amount: u64,
    pub(crate) output_floor: u64,
    pub(crate) input_coin_arg: Argument,
    pub(crate) clock_arg: Argument,
}

pub(crate) fn execute_swap(tx: &mut TransactionBuilder, req: TidesSwap) -> Result<Argument> {
    let function = Function::new(
        req.tides_package,
        "suilend_margin_account_swap".parse()?,
        "swap".parse()?,
        vec![req.input_type.clone(), req.output_type.clone()],
    );

    let pma_arg = add_shared_object(tx, req.protected_margin_account, true)?;
    let rfq_arg = add_shared_object(tx, req.rfq_account, true)?;

    let trade_id_arg = add_u128(tx, req.trade_id);
    let nonce_arg = add_u128(tx, req.nonce);

    let expiry_time_unix_ms_arg = add_u64(tx, req.expiry_time_unix_ms);
    let signature_arg = add_bytes_to_tx(tx, req.signature);

    let input_amount_arg = add_u64(tx, req.input_amount);
    let output_amount_arg = add_u64(tx, req.output_amount);
    let output_floor_arg = add_u64(tx, req.output_floor);

    let balance_out_arg = tx.move_call(
        function,
        vec![
            pma_arg,
            req.lending_market_arg,
            rfq_arg,
            trade_id_arg,
            nonce_arg,
            expiry_time_unix_ms_arg,
            signature_arg,
            input_amount_arg,
            output_amount_arg,
            output_floor_arg,
            req.input_coin_arg,
            req.clock_arg,
        ],
    );
    Ok(balance_out_arg)
}
