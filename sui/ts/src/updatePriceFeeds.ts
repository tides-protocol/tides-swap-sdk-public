import type {
  Transaction,
  TransactionArgument,
  TransactionObjectArgument,
} from '@mysten/sui/transactions';
import type { SharedObjectInfo } from './utils';
import { checkIfPricesStale, maybeCreatePythPriceInfos, maybeDestroyPythPriceInfos, maybeUpdateSinglePythPriceFeed } from './tides';
import { addSharedObject } from './utils';

/**
 * Updates price feeds using Pyth accumulator message
 * Returns an array of updated price info objects
 */
export function updatePriceFeeds(
  tx: Transaction,
  req: {
    tidesPackage: string
    wormholeState: SharedObjectInfo
    pythState: SharedObjectInfo
    pythAccumulatorMessage: Uint8Array
    vaa: Uint8Array
    clock: TransactionObjectArgument
    feeCoin: TransactionObjectArgument
    feeSurplusRecipient: TransactionArgument | string
    priceInfoObjects: SharedObjectInfo[]
    updatePriceFee: bigint
  },
): TransactionObjectArgument[] {
  // Check if posted pyth prices are stale to be used by suilend market
  const staleMapArg = tx.pure.vector('bool', []);
  const priceInfoObjectArgs = [];

  for (const priceInfoObject of req.priceInfoObjects) {
    const priceInfoObjectArg = addSharedObject(tx, priceInfoObject, true);

    // update the stale check result with the one for current price info obj
    checkIfPricesStale(tx, {
      tidesPackage: req.tidesPackage,
      valueArg: staleMapArg,
      priceInfoObjectArg,
      clockArg: req.clock,
    });

    priceInfoObjectArgs.push(priceInfoObjectArg);
  }

  // Use the fetched update bytes from quote payload, if update is needed
  const pythStateArg = addSharedObject(tx, req.pythState, false);

  let maybePriceInfosArg = maybeCreatePythPriceInfos(tx, {
    tidesPackage: req.tidesPackage,
    staleMapArg,
    wormholeState: req.wormholeState,
    pythStateArg,
    vaaBuf: req.vaa,
    pythAccumulatorMessage: req.pythAccumulatorMessage,
    clockArg: req.clock,
  });

  // Apply the updates, if necessary
  for (const [idx, priceInfoObjectArg] of priceInfoObjectArgs.entries()) {
    maybePriceInfosArg = maybeUpdateSinglePythPriceFeed(tx, {
      tidesPackage: req.tidesPackage,
      staleMapArg,
      staleMapIdx: idx,
      pythStateArg,
      maybePriceInfosArg,
      priceInfoObjectArg,
      maxFeeCoinArg: req.feeCoin,
      fee: req.updatePriceFee,
      clockArg: req.clock,
    });
  }

  // Destroy the updated price infos hot potato
  maybeDestroyPythPriceInfos(tx, {
    tidesPackage: req.tidesPackage,
    maybePriceInfosArg,
  });

  // Move the surplus of the fee coin back to the owner
  tx.transferObjects([
    req.feeCoin,
  ], req.feeSurplusRecipient);

  return priceInfoObjectArgs;
}
