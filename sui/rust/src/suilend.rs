use crate::utils::add_u64;
use sui_sdk_types::{Address, Argument, TypeTag};
use sui_transaction_builder::{Function, TransactionBuilder};

use anyhow::Result;

pub(crate) struct UpdateSuilendReserves<'a> {
    pub(crate) suilend_package: Address,
    pub(crate) lending_market: Argument,
    pub(crate) main_pool_type_tag: &'a TypeTag,
    pub(crate) price_info_objects: Vec<(u64, Argument)>,
    pub(crate) clock: Argument,
}

pub(crate) fn update_suilend_reserve_prices(
    tx: &mut TransactionBuilder,
    req: &UpdateSuilendReserves,
) -> Result<()> {
    for (reserve_idx, price_info_object) in &req.price_info_objects {
        let function = Function::new(
            req.suilend_package,
            "lending_market".parse()?,
            "refresh_reserve_price".parse()?,
            vec![req.main_pool_type_tag.clone()],
        );
        let reserve_idx_arg = add_u64(tx, *reserve_idx);
        tx.move_call(
            function,
            vec![
                req.lending_market,
                reserve_idx_arg,
                req.clock,
                price_info_object.clone(),
            ],
        );
    }

    Ok(())
}
