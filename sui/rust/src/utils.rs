use crate::SharedObjectInfo;
use std::str::FromStr;
use sui_sdk_types::{Address, Argument, ObjectId};
use sui_transaction_builder::TransactionBuilder;

use anyhow::Result;
use base64::Engine;
use sui_transaction_builder::unresolved::{Input, InputKind, Value};

pub(crate) fn add_bytes_to_tx(tx: &mut TransactionBuilder, bytes: &[u8]) -> Argument {
    add_vector(tx, bytes)
}

pub(crate) fn add_shared_object(
    tx: &mut TransactionBuilder,
    shared_object: &SharedObjectInfo,
    mutable: bool,
) -> Result<Argument> {
    let input = Input::shared(
        shared_object.id.into(),
        shared_object.initial_shared_version,
        mutable,
    );

    Ok(tx.input(input))
}

pub(crate) fn add_vector<T: serde::Serialize>(
    tx: &mut TransactionBuilder,
    value: &[T],
) -> Argument {
    let value = bcs::to_bytes(value).unwrap();
    tx.input(Input {
        kind: Some(InputKind::Pure),
        value: Some(Value::String(
            base64::engine::general_purpose::STANDARD.encode(value),
        )),
        ..Default::default()
    })
}

pub(crate) fn add_u64(tx: &mut TransactionBuilder, amount: u64) -> Argument {
    tx.input(Input {
        kind: Some(InputKind::Pure),
        value: Some(Value::String(
            base64::engine::general_purpose::STANDARD.encode(amount.to_le_bytes()),
        )),
        ..Default::default()
    })
}

pub(crate) fn add_u128(tx: &mut TransactionBuilder, amount: u128) -> Argument {
    tx.input(Input {
        kind: Some(InputKind::Pure),
        value: Some(Value::String(
            base64::engine::general_purpose::STANDARD.encode(amount.to_le_bytes()),
        )),
        ..Default::default()
    })
}

pub(crate) fn add_clock(tx: &mut TransactionBuilder) -> Argument {
    let input = Input::shared(ObjectId::from_str("0x6").unwrap(), 1, false);

    tx.input(input)
}

pub(crate) fn add_system_state(tx: &mut TransactionBuilder) -> Argument {
    let input = Input::shared(ObjectId::from_str("0x5").unwrap(), 1, true);

    tx.input(input)
}

pub(crate) fn add_address_to_tx(tx: &mut TransactionBuilder, address: Address) -> Argument {
    tx.input(Input {
        kind: Some(InputKind::Pure),
        value: Some(Value::String(
            base64::engine::general_purpose::STANDARD.encode(address),
        )),
        ..Default::default()
    })
}

pub fn address_from_bytes(value: &[u8]) -> Result<Address> {
    let id_raw: [u8; Address::LENGTH] = value.try_into()?;
    Ok(id_raw.into())
}

/// Convert timestamp in Unix ms into prost timestamp
pub fn timestamp_into_proto(timestamp_ms: u64) -> pbjson_types::Timestamp {
    pbjson_types::Timestamp {
        seconds: (timestamp_ms / 1000) as i64,
        nanos: ((timestamp_ms % 1000) * 1_000_000) as i32,
    }
}
