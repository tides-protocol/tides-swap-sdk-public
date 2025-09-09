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
    pub(crate) system_state_arg: Argument,
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
            req.system_state_arg,
        ],
    );
    Ok(balance_out_arg)
}

pub(crate) struct TidesCheckIfPricesStale {
    pub(crate) tides_package: Address,
    pub(crate) value_arg: Argument,
    pub(crate) price_info_object_arg: Argument,
    pub(crate) clock_arg: Argument,
}

/// Checks if the provided `PriceInfoObject` is stale to be used by Suilend market,
/// writing the result flag into the `value_arg` vector.
pub(crate) fn check_if_prices_stale(
    tx: &mut TransactionBuilder,
    req: TidesCheckIfPricesStale,
) -> Result<()> {
    let function = ::sui_transaction_builder::Function::new(
        req.tides_package,
        "pyth_price_update".parse()?,
        "check_if_prices_stale".parse()?,
        vec![],
    );

    tx.move_call(
        function,
        vec![req.value_arg, req.price_info_object_arg, req.clock_arg],
    );
    Ok(())
}

pub(crate) struct TidesMaybeCreatePythPriceInfos<'a> {
    pub(crate) tides_package: Address,
    pub(crate) stale_map_arg: Argument,
    pub(crate) wormhole_state: &'a SharedObjectInfo,
    pub(crate) pyth_state_arg: Argument,
    pub(crate) vaa_buf: &'a [u8],
    pub(crate) pyth_accumulator_message: &'a [u8],
    pub(crate) clock_arg: Argument,
}

/// If at least one price is stale, creates a vector of price infos to use
/// for price info objects' update; otherwise, returns None.
///
/// Returns `Option<HotPotatoVector<PriceInfo>>` argument
pub(crate) fn maybe_create_pyth_price_infos(
    tx: &mut TransactionBuilder,
    req: TidesMaybeCreatePythPriceInfos,
) -> Result<Argument> {
    let function = ::sui_transaction_builder::Function::new(
        req.tides_package,
        "pyth_price_update".parse()?,
        "maybe_create_pyth_price_infos".parse()?,
        vec![],
    );

    let wormhole_state_arg = add_shared_object(tx, req.wormhole_state, false)?;

    let vaa_buf_arg = add_bytes_to_tx(tx, &req.vaa_buf);
    let pyth_accumulator_message_arg = add_bytes_to_tx(tx, &req.pyth_accumulator_message);

    let maybe_price_infos_arg = tx.move_call(
        function,
        vec![
            req.stale_map_arg,
            wormhole_state_arg,
            req.pyth_state_arg,
            vaa_buf_arg,
            pyth_accumulator_message_arg,
            req.clock_arg,
        ],
    );
    Ok(maybe_price_infos_arg)
}

pub(crate) struct TidesMaybeUpdateSinglePythPriceFeed {
    pub(crate) tides_package: Address,
    pub(crate) stale_map_arg: Argument,
    pub(crate) stale_map_idx: u64,
    pub(crate) pyth_state_arg: Argument,
    pub(crate) maybe_price_infos_arg: Argument,
    pub(crate) price_info_object_arg: Argument,
    pub(crate) max_fee_coin_arg: Argument,
    pub(crate) fee: u64,
    pub(crate) clock_arg: Argument,
}

/// Updates the price feed, corresponding to the provided `price_info_object_arg`, if its price is stale.
///
/// If the `maybe_price_infos_arg` vector is provided (i.e. if an update is needed),
/// updates the `price_info_object`'s price, consuming the corresponding item of the vector,
/// returning the rest of it; otherwise, returns None.
///
/// Returns `Option<HotPotatoVector<PriceInfo>>` argument
pub(crate) fn maybe_update_single_pyth_price_feed(
    tx: &mut TransactionBuilder,
    req: TidesMaybeUpdateSinglePythPriceFeed,
) -> Result<Argument> {
    let function = ::sui_transaction_builder::Function::new(
        req.tides_package,
        "pyth_price_update".parse()?,
        "maybe_update_single_pyth_price_feed".parse()?,
        vec![],
    );

    let stale_map_idx_arg = add_u64(tx, req.stale_map_idx);
    let fee_arg = add_u64(tx, req.fee);

    let updated_maybe_price_infos_arg = tx.move_call(
        function,
        vec![
            req.stale_map_arg,
            stale_map_idx_arg,
            req.pyth_state_arg,
            req.maybe_price_infos_arg,
            req.price_info_object_arg,
            req.max_fee_coin_arg,
            fee_arg,
            req.clock_arg,
        ],
    );
    Ok(updated_maybe_price_infos_arg)
}

pub(crate) struct TidesMaybeDestroyPythPriceInfos {
    pub(crate) tides_package: Address,
    pub(crate) maybe_price_infos_arg: Argument,
}

/// If the `maybe_price_infos_arg` vector is provided (i.e. if an update was applied),
/// destroys the underlying vector used for price updates.
pub(crate) fn maybe_destroy_pyth_price_infos(
    tx: &mut TransactionBuilder,
    req: TidesMaybeDestroyPythPriceInfos,
) -> Result<()> {
    let function = ::sui_transaction_builder::Function::new(
        req.tides_package,
        "pyth_price_update".parse()?,
        "maybe_destroy_pyth_price_infos".parse()?,
        vec![],
    );

    tx.move_call(function, vec![req.maybe_price_infos_arg]);
    Ok(())
}
