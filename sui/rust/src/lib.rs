mod _codegen;
mod proto_to_quote;
mod pyth;
mod suilend;
mod tides;
mod utils;
mod wormhole;

use crate::_codegen::tides::sui::common::v1::{DeploymentInfo, SharedObject};
use crate::_codegen::tides::sui::hub::v1::Rfq;
use crate::_codegen::tides::sui::hub::v1::hub_service_client::HubServiceClient;
use crate::_codegen::tides::sui::hub::v1::{
    DeploymentInfoRequest, QuoteTradeExactInRequest, QuoteTradeExactOutRequest,
};
use crate::pyth::{UpdatePriceFeeds, update_price_feeds};
use crate::suilend::{UpdateSuilendReserves, update_suilend_reserve_prices};
use crate::tides::{TidesSwap, execute_swap};
use crate::utils::{
    add_address_to_tx, add_clock, add_shared_object, address_from_bytes, timestamp_into_proto,
};
use anyhow::Result;
use std::str::FromStr;
use sui_sdk_types::{Address, Argument, StructTag, TypeTag};
use sui_transaction_builder::TransactionBuilder;
use tonic::transport::Channel;

pub struct TidesClient {
    hub: HubServiceClient<Channel>,
    chain_id: String,
    deployment_info: DeploymentInfo,
}

impl TidesClient {
    /// Creates a new TidesClient connected to the specified gRPC endpoint.
    /// Automatically fetches deployment information including chain ID and package addresses.
    pub async fn connect(endpoint: &'static str) -> Result<Self> {
        let mut client = HubServiceClient::connect(endpoint).await?;
        let resp = client
            .deployment_info(DeploymentInfoRequest {})
            .await?
            .into_inner();
        Ok(Self {
            hub: client,
            chain_id: resp.chain_id,
            deployment_info: resp
                .deployment_info
                .ok_or_else(|| anyhow::anyhow!("no deployment info received"))?,
        })
    }

    /// Gets a trading quote for swapping an exact input amount to the output token.
    /// Specify minimum_out to set a slippage limit, or None to accept any output amount.
    /// Returns None if no viable trade route exists for the given parameters.
    pub async fn quote_exact_in(
        &mut self,
        input: TypeTag,
        input_amount: u64,
        output: TypeTag,
        min_output_amount: Option<u64>,
        quote_expiry_ms: Option<u64>,
        rfq_timeout_ms: Option<u64>,
    ) -> Result<Option<Quote>> {
        let quote_resp = self
            .hub
            .quote_trade_exact_in(QuoteTradeExactInRequest {
                input_type: input.to_string(),
                input_amount,
                output_type: output.to_string(),
                min_output_amount,
                quote_expiry: quote_expiry_ms.map(timestamp_into_proto),
                rfq_timeout: rfq_timeout_ms.map(timestamp_into_proto),
            })
            .await?
            .into_inner();

        self.proto_rfq_to_quote(input, output, quote_resp.rfq)
    }

    /// Gets a trading quote for receiving an exact output amount from the input token.
    /// Specify max_input to limit how much you're willing to spend, or None for no limit.
    /// Returns None if no viable trade route exists for the given parameters.
    pub async fn quote_exact_out(
        &mut self,
        output: TypeTag,
        output_amount: u64,
        input: TypeTag,
        max_input_amount: Option<u64>,
        quote_expiry_ms: Option<u64>,
        rfq_timeout_ms: Option<u64>,
    ) -> Result<Option<Quote>> {
        let quote_resp = self
            .hub
            .quote_trade_exact_out(QuoteTradeExactOutRequest {
                input_type: input.to_string(),
                max_input_amount,
                output_type: output.to_string(),
                output_amount,
                quote_expiry: quote_expiry_ms.map(timestamp_into_proto),
                rfq_timeout: rfq_timeout_ms.map(timestamp_into_proto),
            })
            .await?
            .into_inner();

        self.proto_rfq_to_quote(input, output, quote_resp.rfq)
    }

    fn proto_rfq_to_quote(
        &self,
        input: TypeTag,
        output: TypeTag,
        rfq: Option<Rfq>,
    ) -> Result<Option<Quote>> {
        let rfq = rfq.ok_or(anyhow::anyhow!("Rfq is empty"))?;
        match rfq.quote {
            None => Ok(None),
            Some(_) => Ok(Quote::from_proto(
                input,
                output,
                address_from_bytes(self.deployment_info.package.as_ref())?,
                rfq,
            )
            .map(Some)?),
        }
    }

    /// Encodes the quote to bytes.
    pub fn encode_quote_to_bytes(&self, quote: &Quote) -> Result<Vec<u8>> {
        use prost::Message;
        Ok(quote.original.encode_to_vec())
    }

    /// Decodes the quote from bytes.
    pub fn decode_quote_from_bytes(
        &self,
        input: TypeTag,
        output: TypeTag,
        bytes: &[u8],
    ) -> Result<Option<Quote>> {
        use prost::Message;
        let decode = Rfq::decode(bytes)?;
        self.proto_rfq_to_quote(input, output, Some(decode))
    }
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub struct SharedObjectInfo {
    pub id: Address,
    pub initial_shared_version: u64,
}

impl From<SharedObject> for SharedObjectInfo {
    fn from(value: SharedObject) -> Self {
        Self {
            id: Address::from_bytes(value.id).unwrap(),
            initial_shared_version: value.initial_shared_version,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Quote {
    input: TypeTag,
    output: TypeTag,

    tides_package: Address,
    suilend_package: Address,
    pyth_package_id: Address,
    wormhole_package_id: Address,

    suilend_market: SharedObjectInfo,
    wormhole_state: SharedObjectInfo,
    pyth_state: SharedObjectInfo,

    protected_margin_account: SharedObjectInfo,
    rfq_account: SharedObjectInfo,

    pyth_accumulator_message: Vec<u8>,
    vaa: Vec<u8>,
    price_info_objects: Vec<(u64, SharedObjectInfo)>,
    update_price_fee: u64,

    // trade data
    trade_id: u128,
    nonce: u128,
    expiry_time_ms: u64,
    signature: Vec<u8>,
    input_amount: u64,
    output_amount: u64,
    output_floor: u64,

    original: Rfq,
}

impl Quote {
    /// Applies all necessary Move commands to execute the swap and transfers the resulting
    /// output coin directly to the recipient address. This is a convenience method that
    /// combines apply_swap_to_tx() with a transfer operation.
    pub fn apply_swap_to_tx_and_transfer_coin(
        &self,
        tx: &mut TransactionBuilder,
        input_coin: Argument,
        pay_suilend_fees_coin: Argument,
        recipient: Address,
        clock: Option<Argument>,
    ) -> Result<()> {
        let output = self.apply_swap_to_tx(tx, input_coin, pay_suilend_fees_coin, clock)?;

        let address_arg = add_address_to_tx(tx, recipient);
        tx.transfer_objects(vec![output], address_arg);

        Ok(())
    }
    /// Applies all necessary Move commands to execute the swap, including price updates
    /// and Suilend reserve refreshes. Returns the output balance argument for further use.
    /// Use this when you want to handle the output balance yourself instead of transferring.
    pub fn apply_swap_to_tx(
        &self,
        tx: &mut TransactionBuilder,
        input_coin: Argument,
        pay_suilend_fees_coin: Argument,
        clock: Option<Argument>,
    ) -> Result<Argument> {
        let clock = clock.unwrap_or_else(|| add_clock(tx));
        let sui_type_tag = TypeTag::from_str("0x2::sui::SUI")?;

        let update_price = UpdatePriceFeeds {
            pyth_package: self.pyth_package_id,
            wormhole_package: self.wormhole_package_id,
            wormhole_state: &self.wormhole_state,
            pyth_state: &self.pyth_state,
            pyth_accumulator_message: &self.pyth_accumulator_message,
            vaa: &self.vaa,
            clock,
            fee_coin: pay_suilend_fees_coin,
            fee_coin_type: &sui_type_tag,
            price_info_objects: self.price_info_objects.iter().map(|v| &v.1).collect(),
            update_price_fee: self.update_price_fee,
        };

        let updated_prices = update_price_feeds(tx, &update_price)?;

        // now update suilend reserves.
        let suilend_market_arg = add_shared_object(tx, &self.suilend_market, true)?;
        let update_suilend_reserves = UpdateSuilendReserves {
            suilend_package: self.suilend_package,
            lending_market: suilend_market_arg,
            main_pool_type_tag: &TypeTag::Struct(Box::new(StructTag {
                address: self.suilend_package,
                module: "suilend".parse()?,
                name: "MAIN_POOL".parse()?,
                type_params: vec![],
            })),
            price_info_objects: updated_prices
                .into_iter()
                .enumerate()
                .map(|(idx, arg)| (self.price_info_objects.get(idx).unwrap().0, arg))
                .collect(),
            clock,
        };

        update_suilend_reserve_prices(tx, &update_suilend_reserves)?;

        // now do the swap
        let swap = TidesSwap {
            tides_package: self.tides_package,
            input_type: &self.input,
            output_type: &self.output,
            protected_margin_account: &self.protected_margin_account,
            lending_market_arg: suilend_market_arg,
            rfq_account: &self.rfq_account,
            trade_id: self.trade_id,
            nonce: self.nonce,
            expiry_time_unix_ms: self.expiry_time_ms,
            signature: &self.signature,
            input_amount: self.input_amount,
            output_amount: self.output_amount,
            output_floor: self.output_floor,
            input_coin_arg: input_coin,
            clock_arg: clock,
        };

        execute_swap(tx, swap)
    }

    /// Returns the TypeTag for the output token that will be received from this swap.
    pub fn output(&self) -> &TypeTag {
        &self.output
    }

    /// Returns the TypeTag for the input token that will be consumed by this swap.
    pub fn input(&self) -> &TypeTag {
        &self.input
    }

    /// Returns the exact input amount that this quote was calculated for.
    /// This is the amount of input tokens that should be provided to execute the swap.
    pub fn input_amount(&self) -> u64 {
        self.input_amount
    }

    /// Returns the minimum input amount needed for this RFQ to be valid.
    /// Any input below this amount would result in taking less than the minimum output_floor.
    pub fn input_floor(&self) -> u64 {
        // Calculate minimum input: (output_floor * max_input) / max_output
        // Use u128 to avoid overflow, then convert back
        let min_input =
            (self.output_floor as u128 * self.input_amount as u128) / self.output_amount as u128;
        min_input as u64 // Safe conversion since result should be <= input_amount
    }

    /// Returns the minimum output amount that can be taken from this RFQ.
    /// This represents the minimum take amount for the quote.
    pub fn output_floor(&self) -> u64 {
        self.output_floor
    }

    /// Returns the maximum output amount that can be taken from this RFQ.
    /// This is achieved when providing the full input_amount().
    pub fn output_ceiling(&self) -> u64 {
        self.output_amount
    }

    /// Returns the expiry time of this RFQ in Unix milliseconds.
    /// The quote becomes invalid after this timestamp.
    pub fn expiry_time_ms(&self) -> u64 {
        self.expiry_time_ms
    }

    /// Returns the total SUI fees required to update Pyth price feeds before executing the swap.
    /// This cost is in addition to the input tokens and must be paid separately.
    pub fn pyth_price_fees(&self) -> u64 {
        self.price_info_objects.len() as u64 * self.update_price_fee
    }

    /// Calculates the output amount for a given input amount using this RFQ's exchange rate.
    /// Returns None if the calculated output would be below output_floor (minimum takeable amount)
    /// or above output_ceiling (maximum available amount). Use this for partial fills.
    pub fn calculate_output_amount(&self, input_amount: u64) -> Option<u64> {
        if input_amount == 0 {
            return None;
        }

        // Calculate output using RFQ's exchange rate: output = (input * max_output) / max_input
        // Use u128 to avoid overflow during multiplication
        let calculated_output =
            (input_amount as u128 * self.output_amount as u128) / self.input_amount as u128;

        // Convert back to u64, checking for overflow
        let calculated_output = u64::try_from(calculated_output).ok()?;

        // Check bounds: must be within [output_floor, output_amount] range
        if calculated_output >= self.output_floor && calculated_output <= self.output_amount {
            Some(calculated_output)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn it_builds() {}

    #[test]
    fn test_calculate_output_amount() {
        // Create a mock quote with 1000 input -> 2000 output (2:1 ratio)
        let quote = Quote {
            input: TypeTag::from_str("0x2::sui::SUI").unwrap(),
            output: TypeTag::from_str("0x2::coin::COIN").unwrap(),
            tides_package: Address::from_str("0x1").unwrap(),
            suilend_package: Address::from_str("0x1").unwrap(),
            pyth_package_id: Address::from_str("0x1").unwrap(),
            wormhole_package_id: Address::from_str("0x1").unwrap(),
            suilend_market: SharedObjectInfo {
                id: Address::from_str("0x1").unwrap(),
                initial_shared_version: 1,
            },
            wormhole_state: SharedObjectInfo {
                id: Address::from_str("0x1").unwrap(),
                initial_shared_version: 1,
            },
            pyth_state: SharedObjectInfo {
                id: Address::from_str("0x1").unwrap(),
                initial_shared_version: 1,
            },
            protected_margin_account: SharedObjectInfo {
                id: Address::from_str("0x1").unwrap(),
                initial_shared_version: 1,
            },
            rfq_account: SharedObjectInfo {
                id: Address::from_str("0x1").unwrap(),
                initial_shared_version: 1,
            },
            pyth_accumulator_message: vec![],
            vaa: vec![],
            price_info_objects: vec![],
            update_price_fee: 100,
            trade_id: 123,
            nonce: 456,
            expiry_time_ms: 1000000,
            signature: vec![],
            input_amount: 1000,  // max input
            output_amount: 2000, // max output
            output_floor: 500,   // min output
            original: Default::default(),
        };

        // Test input_floor calculation
        assert_eq!(quote.input_floor(), 250); // 500 (output_floor) * 1000 / 2000 = 250

        // Test expiry time
        assert_eq!(quote.expiry_time_ms(), 1000000);

        // Test exact max input -> max output
        assert_eq!(quote.calculate_output_amount(1000), Some(2000));

        // Test half input -> half output
        assert_eq!(quote.calculate_output_amount(500), Some(1000));

        // Test minimum viable input (should give output_floor)
        let min_input = quote.input_floor();
        assert_eq!(quote.calculate_output_amount(min_input), Some(500));

        // Test input too small (below input_floor)
        assert_eq!(quote.calculate_output_amount(200), None);

        // Test input too large (above max)
        assert_eq!(quote.calculate_output_amount(1500), None);

        // Test zero input
        assert_eq!(quote.calculate_output_amount(0), None);
    }
}
