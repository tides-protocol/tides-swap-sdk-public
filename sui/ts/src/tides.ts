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
    ],
  });
}
