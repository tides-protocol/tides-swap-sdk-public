import type {
  Transaction,
  TransactionArgument,
} from '@mysten/sui/transactions';
import type { SharedObjectInfo } from './utils';
import { addSharedObject, destroyZeroCoin, splitCoin } from './utils';
import { verifyVaa } from './wormhole';

/**
 * Updates price feeds using Pyth accumulator message
 * Returns an array of updated price info objects
 */
export function updatePriceFeeds(
  tx: Transaction,
  req: {
    pythPackage: string
    wormholePackage: string
    wormholeState: SharedObjectInfo
    pythState: SharedObjectInfo
    pythAccumulatorMessage: Uint8Array
    vaa: Uint8Array
    clock: TransactionArgument
    feeCoin: TransactionArgument
    feeCoinType: string
    priceInfoObjects: SharedObjectInfo[]
    updatePriceFee: bigint
  },
): TransactionArgument[] {
  // Verify VAA first
  const verifiedVaaArg = verifyVaa(tx, {
    wormholePackage: req.wormholePackage,
    wormholeState: req.wormholeState,
    clock: req.clock,
    vaa: req.vaa,
  });

  // Create pyth state arg
  const pythStateArg = addSharedObject(tx, req.pythState, false);

  // Get price updates hot potato
  let priceUpdatesHotPotatoArg = createAuthenticatedPriceInfosUsingAccumulator(
    tx,
    req,
    pythStateArg,
    verifiedVaaArg,
  );

  // Do price updates
  const priceInfoObjects: TransactionArgument[] = [];
  for (const priceInfoObject of req.priceInfoObjects) {
    const [priceInfoObjectArg, updatedPriceUpdatesHotPotatoArg]
      = updateSinglePricefeed(
        tx,
        req,
        pythStateArg,
        priceInfoObject,
        priceUpdatesHotPotatoArg,
      );

    priceInfoObjects.push(priceInfoObjectArg);
    priceUpdatesHotPotatoArg = updatedPriceUpdatesHotPotatoArg;
  }

  // Destroy zeroed fee coin
  destroyZeroCoin(tx, req.feeCoinType, req.feeCoin);

  // Destroy hot potato object
  destroyHotPotatoVector(tx, req, priceUpdatesHotPotatoArg);

  return priceInfoObjects;
}

/**
 * Creates authenticated price infos using accumulator
 * Returns HotPotatoVector<PriceInfo>
 */
function createAuthenticatedPriceInfosUsingAccumulator(
  tx: Transaction,
  req: {
    pythPackage: string
    pythAccumulatorMessage: Uint8Array
    clock: TransactionArgument
  },
  pythStateArg: TransactionArgument,
  verifiedVaaArg: TransactionArgument,
): TransactionArgument {
  const accumulatorMessageArg = tx.pure.vector('u8', req.pythAccumulatorMessage);

  return tx.moveCall({
    package: req.pythPackage,
    module: 'pyth',
    function: 'create_authenticated_price_infos_using_accumulator',
    typeArguments: [],
    arguments: [pythStateArg, accumulatorMessageArg, verifiedVaaArg, req.clock],
  });
}

/**
 * Updates a single price feed
 * Returns tuple of (price_info_object, updated_hot_potato)
 */
function updateSinglePricefeed(
  tx: Transaction,
  req: {
    pythPackage: string
    feeCoin: TransactionArgument
    feeCoinType: string
    updatePriceFee: bigint
    clock: TransactionArgument
  },
  pythStateArg: TransactionArgument,
  priceInfoObject: SharedObjectInfo,
  priceUpdatesHotPotatoArg: TransactionArgument,
): [TransactionArgument, TransactionArgument] {
  const feeForSingleFeed = splitCoin(
    tx,
    req.feeCoinType,
    req.feeCoin,
    req.updatePriceFee,
  );

  const priceInfoObjectArg = addSharedObject(tx, priceInfoObject, true);

  // Update price feed
  const extendedPriceUpdatesArg = tx.moveCall({
    package: req.pythPackage,
    module: 'pyth',
    function: 'update_single_price_feed',
    typeArguments: [],
    arguments: [
      pythStateArg,
      priceUpdatesHotPotatoArg,
      priceInfoObjectArg,
      feeForSingleFeed,
      req.clock,
    ],
  });

  return [priceInfoObjectArg, extendedPriceUpdatesArg];
}

/**
 * Destroys the hot potato vector
 */
function destroyHotPotatoVector(
  tx: Transaction,
  req: {
    pythPackage: string
  },
  hotPotatoVectorArg: TransactionArgument,
): void {
  const priceInfoType = `${req.pythPackage}::price_info::PriceInfo`;

  tx.moveCall({
    package: req.pythPackage,
    module: 'hot_potato_vector',
    function: 'destroy',
    typeArguments: [priceInfoType],
    arguments: [hotPotatoVectorArg],
  });
}
