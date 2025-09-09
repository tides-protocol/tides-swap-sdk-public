import type {
  Transaction,
  TransactionArgument,
  TransactionObjectArgument,
} from '@mysten/sui/transactions';
import type { SharedObjectInfo } from './utils';
import { addSharedObject } from './utils';

/**
 * Executes a swap through the Tides protocol
 * Returns the output balance
 */
export function executeSwap(
  tx: Transaction,
  req: {
    tidesPackage: string
    inputType: string
    outputType: string
    protectedMarginAccount: SharedObjectInfo
    lendingMarketArg: TransactionArgument
    rfqAccount: SharedObjectInfo
    tradeId: bigint
    nonce: bigint
    expiryTimeUnixMs: bigint
    signature: Uint8Array
    inputAmount: bigint
    outputAmount: bigint
    outputFloor: bigint
    inputCoinArg: TransactionArgument
    clockArg: TransactionArgument
    systemStateArg: TransactionArgument
  },
): TransactionObjectArgument {
  const pmaArg = addSharedObject(tx, req.protectedMarginAccount, true);
  const rfqArg = addSharedObject(tx, req.rfqAccount, true);

  const tradeIdArg = tx.pure.u128(req.tradeId);
  const nonceArg = tx.pure.u128(req.nonce);

  const expiryTimeUnixMsArg = tx.pure.u64(req.expiryTimeUnixMs);
  const signatureArg = tx.pure.vector('u8', req.signature);

  const inputAmountArg = tx.pure.u64(req.inputAmount);
  const outputAmountArg = tx.pure.u64(req.outputAmount);
  const outputFloorArg = tx.pure.u64(req.outputFloor);

  return tx.moveCall({
    package: req.tidesPackage,
    module: 'suilend_margin_account_swap',
    function: 'swap',
    typeArguments: [req.inputType, req.outputType],
    arguments: [
      pmaArg,
      req.lendingMarketArg,
      rfqArg,
      tradeIdArg,
      nonceArg,
      expiryTimeUnixMsArg,
      signatureArg,
      inputAmountArg,
      outputAmountArg,
      outputFloorArg,
      req.inputCoinArg,
      req.clockArg,
      req.systemStateArg,
    ],
  });
}

export function checkIfPricesStale(tx: Transaction, req: {
  tidesPackage: string
  valueArg: TransactionArgument
  priceInfoObjectArg: TransactionArgument
  clockArg: TransactionArgument
}): void {
  tx.moveCall({
    package: req.tidesPackage,
    module: 'pyth_price_update',
    function: 'check_if_prices_stale',
    arguments: [
      req.valueArg,
      req.priceInfoObjectArg,
      req.clockArg,
    ],
  });
}

export function maybeCreatePythPriceInfos(tx: Transaction, req: {
  tidesPackage: string
  staleMapArg: TransactionArgument
  wormholeState: SharedObjectInfo
  pythStateArg: TransactionArgument
  vaaBuf: Uint8Array
  pythAccumulatorMessage: Uint8Array
  clockArg: TransactionArgument
}): TransactionArgument {
  const wormholeStateArg = addSharedObject(tx, req.wormholeState, false);

  const vaaBufArg = tx.pure.vector('u8', req.vaaBuf);
  const pythAccumulatorMessageArg = tx.pure.vector('u8', req.pythAccumulatorMessage);

  const maybePriceInfosArg = tx.moveCall({
    package: req.tidesPackage,
    module: 'pyth_price_update',
    function: 'maybe_create_pyth_price_infos',
    arguments: [
      req.staleMapArg,
      wormholeStateArg,
      req.pythStateArg,
      vaaBufArg,
      pythAccumulatorMessageArg,
      req.clockArg,
    ],
  });

  return maybePriceInfosArg;
}

export function maybeUpdateSinglePythPriceFeed(tx: Transaction, req: {
  tidesPackage: string
  staleMapArg: TransactionArgument
  staleMapIdx: bigint | number
  pythStateArg: TransactionArgument
  maybePriceInfosArg: TransactionArgument
  priceInfoObjectArg: TransactionArgument
  maxFeeCoinArg: TransactionArgument
  fee: bigint | number
  clockArg: TransactionArgument
}): TransactionArgument {
  const staleMapIdxArg = tx.pure.u64(req.staleMapIdx);
  const feeArg = tx.pure.u64(req.fee);

  const updatedMaybePriceInfosArg = tx.moveCall({
    package: req.tidesPackage,
    module: 'pyth_price_update',
    function: 'maybe_update_single_pyth_price_feed',
    arguments: [
      req.staleMapArg,
      staleMapIdxArg,
      req.pythStateArg,
      req.maybePriceInfosArg,
      req.priceInfoObjectArg,
      req.maxFeeCoinArg,
      feeArg,
      req.clockArg,
    ],
  });

  return updatedMaybePriceInfosArg;
}

export function maybeDestroyPythPriceInfos(tx: Transaction, req: {
  tidesPackage: string
  maybePriceInfosArg: TransactionArgument
}): void {
  tx.moveCall({
    package: req.tidesPackage,
    module: 'pyth_price_update',
    function: 'maybe_destroy_pyth_price_infos',
    arguments: [
      req.maybePriceInfosArg,
    ],
  });
}
