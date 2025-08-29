import type { Transaction, TransactionArgument } from '@mysten/sui/transactions';
import type { SharedObjectInfo } from './utils.ts';
import { addSharedObject } from './utils.ts';

export function verifyVaa(
  tx: Transaction,
  req: {
    wormholePackage: string
    wormholeState: SharedObjectInfo
    clock: TransactionArgument
    vaa: Uint8Array
  },
): TransactionArgument {
  const wormholeStateArg = addSharedObject(tx, req.wormholeState, false);
  const vaaArg = tx.pure.vector('u8', req.vaa);

  return tx.moveCall({
    package: req.wormholePackage,
    module: 'vaa',
    function: 'parse_and_verify',
    typeArguments: [],
    arguments: [wormholeStateArg, vaaArg, req.clock],
  });
}
