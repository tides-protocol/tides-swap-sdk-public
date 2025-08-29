use anyhow::Result;
use sui_sdk_types::{Address, Argument, StructTag, TypeTag};
use sui_transaction_builder::{Function, TransactionBuilder};

use crate::SharedObjectInfo;
use crate::utils::{add_bytes_to_tx, add_shared_object, destroy_zero_coin, split_coin};
use crate::wormhole::verify_vaas;

pub(crate) struct UpdatePriceFeeds<'a> {
    pub(crate) pyth_package: Address,
    pub(crate) wormhole_package: Address,
    pub(crate) wormhole_state: &'a SharedObjectInfo,
    pub(crate) pyth_state: &'a SharedObjectInfo,
    pub(crate) pyth_accumulator_message: &'a [u8],
    pub(crate) vaa: &'a [u8],
    pub(crate) clock: Argument,
    pub(crate) fee_coin: Argument,
    pub(crate) fee_coin_type: &'a TypeTag,
    pub(crate) price_info_objects: Vec<&'a SharedObjectInfo>,
    pub(crate) update_price_fee: u64,
}

/// Returns Vec<Argument<PriceInfoObject>>
pub(crate) fn update_price_feeds(
    tx: &mut TransactionBuilder,
    req: &UpdatePriceFeeds,
) -> Result<Vec<Argument>> {
    let verified_vaa_arg = verify_vaas(tx, req)?;

    // create pyth state arg
    let pyth_state_arg = add_shared_object(tx, req.pyth_state, false)?;

    // get price updates hot potato
    let mut price_updates_hot_potato_arg = create_authenticated_price_infos_using_accumulator(
        tx,
        req,
        pyth_state_arg,
        verified_vaa_arg,
    )?;

    // do price updates
    let mut price_info_objects = vec![];
    for price_info_object in &req.price_info_objects {
        let (price_info_object_arg, updated_price_updates_hot_potato_arg) =
            update_single_pricefeed(
                tx,
                req,
                pyth_state_arg,
                price_info_object,
                price_updates_hot_potato_arg,
            )?;

        price_info_objects.push(price_info_object_arg);
        price_updates_hot_potato_arg = updated_price_updates_hot_potato_arg;
    }

    // destroy zeroed fee coin
    destroy_zero_coin(tx, req.fee_coin_type, req.fee_coin)?;

    // destroy hot potato object
    destroy_hot_potato_vector(tx, req, price_updates_hot_potato_arg)?;

    Ok(price_info_objects)
}

/// Returns HowPotatoVector<PriceInfo>
fn create_authenticated_price_infos_using_accumulator(
    tx: &mut TransactionBuilder,
    req: &UpdatePriceFeeds,
    pyth_state_arg: Argument,
    verified_vaa_arg: Argument,
) -> Result<Argument> {
    let function = Function::new(
        req.pyth_package,
        "pyth".parse()?,
        "create_authenticated_price_infos_using_accumulator".parse()?,
        vec![],
    );

    let accumulator_message_arg = add_bytes_to_tx(tx, req.pyth_accumulator_message);

    Ok(tx.move_call(
        function,
        vec![
            pyth_state_arg,
            accumulator_message_arg,
            verified_vaa_arg,
            req.clock,
        ],
    ))
}

fn update_single_pricefeed(
    tx: &mut TransactionBuilder,
    req: &UpdatePriceFeeds,
    pyth_state_arg: Argument,
    price_info_object: &SharedObjectInfo,
    price_updates_hot_potato_arg: Argument,
) -> Result<(Argument, Argument)> {
    let fee_for_single_feed =
        split_coin(tx, req.fee_coin_type, req.fee_coin, req.update_price_fee)?;

    let price_info_object = add_shared_object(tx, price_info_object, true)?;
    // update price feed
    let function = Function::new(
        req.pyth_package,
        "pyth".parse()?,
        "update_single_price_feed".parse()?,
        vec![],
    );

    let extended_price_updates_arg = tx.move_call(
        function,
        vec![
            pyth_state_arg,
            price_updates_hot_potato_arg,
            price_info_object,
            fee_for_single_feed,
            req.clock,
        ],
    );

    Ok((price_info_object, extended_price_updates_arg))
}

fn destroy_hot_potato_vector(
    tx: &mut TransactionBuilder,
    req: &UpdatePriceFeeds,
    hot_potato_vector_arg: Argument,
) -> Result<()> {
    let function = Function::new(
        req.pyth_package,
        "hot_potato_vector".parse()?,
        "destroy".parse()?,
        vec![TypeTag::Struct(Box::new(StructTag {
            address: req.pyth_package,
            module: "price_info".parse()?,
            name: "PriceInfo".parse()?,
            type_params: vec![],
        }))],
    );

    tx.move_call(function, vec![hot_potato_vector_arg]);

    Ok(())
}
