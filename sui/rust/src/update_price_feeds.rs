use anyhow::Result;
use sui_sdk_types::{Address, Argument};
use sui_transaction_builder::TransactionBuilder;

use crate::SharedObjectInfo;
use crate::tides::{
    TidesCheckIfPricesStale, TidesMaybeCreatePythPriceInfos, TidesMaybeDestroyPythPriceInfos,
    TidesMaybeUpdateSinglePythPriceFeed, check_if_prices_stale, maybe_create_pyth_price_infos,
    maybe_destroy_pyth_price_infos, maybe_update_single_pyth_price_feed,
};
use crate::utils::{add_shared_object, add_vector};

pub(crate) struct UpdatePriceFeeds<'a> {
    pub(crate) tides_package: Address,
    pub(crate) wormhole_state: &'a SharedObjectInfo,
    pub(crate) pyth_state: &'a SharedObjectInfo,
    pub(crate) pyth_accumulator_message: &'a [u8],
    pub(crate) vaa: &'a [u8],
    pub(crate) clock: Argument,
    pub(crate) fee_coin: Argument,
    pub(crate) fee_surplus_recipient_arg: Argument,
    pub(crate) price_info_objects: Vec<&'a SharedObjectInfo>,
    pub(crate) update_price_fee: u64,
}

pub(crate) fn update_price_feeds(
    tx: &mut TransactionBuilder,
    req: &UpdatePriceFeeds,
) -> Result<Vec<Argument>> {
    // Check if posted pyth prices are stale to be used by suilend market
    let stale_flags: Vec<bool> = vec![];
    let stale_map_arg = add_vector(tx, &stale_flags);
    let mut price_info_object_args = vec![];
    for price_info_object in &req.price_info_objects {
        let price_info_object_arg = add_shared_object(tx, price_info_object, true)?;

        // update the stale check result with the one for current price info obj
        check_if_prices_stale(
            tx,
            TidesCheckIfPricesStale {
                tides_package: req.tides_package,
                value_arg: stale_map_arg,
                price_info_object_arg,
                clock_arg: req.clock,
            },
        )?;

        price_info_object_args.push(price_info_object_arg);
    }

    // Use the fetched update bytes from quote payload, if update is needed
    let pyth_state_arg = add_shared_object(tx, req.pyth_state, false)?;

    let mut maybe_price_infos_arg = maybe_create_pyth_price_infos(
        tx,
        TidesMaybeCreatePythPriceInfos {
            tides_package: req.tides_package,
            stale_map_arg,
            wormhole_state: req.wormhole_state,
            pyth_state_arg,
            vaa_buf: req.vaa,
            pyth_accumulator_message: req.pyth_accumulator_message,
            clock_arg: req.clock,
        },
    )?;

    // Apply the updates, if necessary
    for (idx, price_info_object_arg) in price_info_object_args.iter().cloned().enumerate() {
        maybe_price_infos_arg = maybe_update_single_pyth_price_feed(
            tx,
            TidesMaybeUpdateSinglePythPriceFeed {
                tides_package: req.tides_package,
                stale_map_arg,
                stale_map_idx: idx as u64,
                pyth_state_arg,
                maybe_price_infos_arg,
                price_info_object_arg,
                max_fee_coin_arg: req.fee_coin,
                fee: req.update_price_fee,
                clock_arg: req.clock,
            },
        )?;
    }

    // Destroy the updated price infos hot potato
    maybe_destroy_pyth_price_infos(
        tx,
        TidesMaybeDestroyPythPriceInfos {
            tides_package: req.tides_package,
            maybe_price_infos_arg,
        },
    )?;

    // Move the surplus of the fee coin back to the owner
    tx.transfer_objects(vec![req.fee_coin], req.fee_surplus_recipient_arg);

    Ok(price_info_object_args)
}
