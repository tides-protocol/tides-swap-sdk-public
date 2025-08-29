import type {
  Transaction,
  TransactionArgument,
} from '@mysten/sui/transactions';
import type { SharedObject } from './_codegen/tides/sui/common/v1/v1.pb.ts';

export interface SharedObjectInfo {
  id: string
  initialSharedVersion: bigint
}

export function addSharedObject(
  tx: Transaction,
  object: SharedObjectInfo,
  mutable: boolean,
): TransactionArgument {
  return tx.sharedObjectRef({
    objectId: object.id,
    initialSharedVersion: object.initialSharedVersion.toString(),
    mutable,
  });
}

export function splitCoin(
  tx: Transaction,
  coinType: string,
  coin: TransactionArgument,
  amount: bigint,
): TransactionArgument {
  return tx.moveCall({
    package: '0x2',
    module: 'coin',
    function: 'split',
    typeArguments: [coinType],
    arguments: [coin, tx.pure.u64(amount)],
  });
}

export function destroyZeroCoin(
  tx: Transaction,
  coinType: string,
  coin: TransactionArgument,
): void {
  tx.moveCall({
    package: '0x2',
    module: 'coin',
    function: 'destroy_zero',
    typeArguments: [coinType],
    arguments: [coin],
  });
}

/**
 * Helper to convert Uint8Array to hex string for object ID
 */
export function objectIdFromUint8Array(bytes: Uint8Array): string {
  return `0x${Array.from(bytes)
    .map((byte) => byte.toString(16).padStart(2, '0'))
    .join('')}`;
}

export function u128FromLeBytes(bytes: Uint8Array): bigint {
  let res: bigint;
  if (bytes.length === 16) {
    // Convert from little-endian bytes to bigint
    const view = new DataView(
      bytes.buffer.slice(
        bytes.byteOffset,
        bytes.byteOffset + bytes.byteLength,
      ),
    );
    res
        = view.getBigUint64(0, true) + (view.getBigUint64(8, true) << 64n);
  } else {
    throw new Error(
      `Invalid length: expected 16 bytes, got ${bytes.length}`,
    );
  }
  return res;
}

/**
 * Helper to conver shared object structure, parsing the inner object ID into a hex string.
 */
export function parseSharedObjectInfo({ id, initialSharedVersion }: SharedObject): SharedObjectInfo {
  return {
    id: objectIdFromUint8Array(id),
    initialSharedVersion,
  };
}
