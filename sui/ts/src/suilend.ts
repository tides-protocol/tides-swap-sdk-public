import type {
  Transaction,
  TransactionArgument,
} from '@mysten/sui/transactions';

/**
 * Updates Suilend reserve prices using price info objects
 */
export function updateSuilendReservePrices(
  tx: Transaction,
  req: {
    suilendPackage: string
    lendingMarket: TransactionArgument
    mainPoolTypeTag: string
    priceInfoObjects: {
      reserveIdx: bigint
      priceInfoObject: TransactionArgument
    }[]
    clock: TransactionArgument
  },
): void {
  for (const { reserveIdx, priceInfoObject } of req.priceInfoObjects) {
    const reserveIdxArg = tx.pure.u64(reserveIdx);

    tx.moveCall({
      package: req.suilendPackage,
      module: 'lending_market',
      function: 'refresh_reserve_price',
      typeArguments: [req.mainPoolTypeTag],
      arguments: [req.lendingMarket, reserveIdxArg, req.clock, priceInfoObject],
    });
  }
}
