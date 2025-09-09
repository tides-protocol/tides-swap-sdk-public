use anyhow::{Result, anyhow, bail};
use sui_sdk_types::{Address, TypeTag};

use crate::_codegen::tides::sui::hub::v1::Rfq;
use crate::_codegen::tides::sui::hub::v1::quote::Payload;
use crate::utils::address_from_bytes;
use crate::{Quote, SharedObjectInfo};

impl Quote {
    pub fn from_proto(
        input: TypeTag,
        output: TypeTag,
        tides_package: Address,
        rfq: Rfq,
    ) -> Result<Self> {
        let proto_quote = rfq.quote.as_ref().ok_or(anyhow!("empty quote"))?;
        let suilend_payload = match proto_quote
            .payload
            .as_ref()
            .ok_or(anyhow!("empty payload"))?
        {
            Payload::NativePayload(_) => bail!("native payload not supported"),
            Payload::SuilendPayload(suilend) => suilend,
        };

        let pyth_config = suilend_payload
            .pyth_config
            .as_ref()
            .ok_or(anyhow!("no pyth config"))?;

        let price_updates_payload = suilend_payload
            .price_updates_payload
            .as_ref()
            .ok_or(anyhow!("no price updates"))?;

        let quote = Self {
            input,
            output,
            tides_package,
            suilend_package: address_from_bytes(&suilend_payload.suilend_package_id)?,
            suilend_market: suilend_payload
                .suilend_lending_market_id
                .clone()
                .ok_or(anyhow!("empty suilend market id"))?
                .into(),
            wormhole_state: pyth_config
                .wormhole_state_id
                .clone()
                .ok_or(anyhow!("empty wormhole state"))?
                .into(),
            pyth_state: pyth_config
                .pyth_state_id
                .clone()
                .ok_or(anyhow!("empty pyth state"))?
                .into(),
            protected_margin_account: proto_quote
                .protected_margin_account_id
                .clone()
                .ok_or(anyhow!("empty protected margin account"))?
                .into(),
            rfq_account: proto_quote
                .rfq_account_id
                .clone()
                .ok_or(anyhow!("empty rfq account"))?
                .into(),
            pyth_accumulator_message: price_updates_payload.update.clone().into(),
            vaa: price_updates_payload.vaa.clone().into(),
            price_info_objects: price_updates_payload
                .price_updates
                .iter()
                .map(|v| -> Result<_> {
                    let price_info_object: SharedObjectInfo = v
                        .price_info_object_id
                        .clone()
                        .ok_or(anyhow!("empty price info object"))?
                        .into();

                    let reserve_array_index = v.reserve_array_index;

                    Ok((reserve_array_index, price_info_object.clone()))
                })
                .collect::<Result<Vec<_>>>()?,
            update_price_fee: price_updates_payload.update_price_fee,
            trade_id: u128::from_le_bytes(rfq.trade_id.as_ref().try_into()?),
            nonce: u128::from_le_bytes(proto_quote.nonce.as_ref().try_into()?),
            expiry_time_ms: proto_quote.expiry_timestamp_unix_ms,
            signature: proto_quote.signature.clone().into(),
            input_amount: proto_quote.input_amount,
            output_amount: proto_quote.output_amount,
            output_floor: proto_quote.output_floor,
            original: rfq,
        };

        Ok(quote)
    }
}
