import type {
  Transaction,
  TransactionArgument,
  TransactionObjectArgument,
} from '@mysten/sui/transactions';
import type { Rfq } from './_codegen/tides/sui/hub/v1/v1.pb.ts';
import type { SharedObjectInfo } from './utils';
import { Rfq as RfqDecoder } from './_codegen/tides/sui/hub/v1/v1.pb.ts';
import { updatePriceFeeds } from './pyth';
import { updateSuilendReservePrices } from './suilend';
import { executeSwap } from './tides';
import { addSharedObject, objectIdFromUint8Array, parseSharedObjectInfo, u128FromLeBytes } from './utils';

/**
 * Quote class for Tides swap execution with all necessary data and methods
 */
export class Quote {
  input!: string;
  output!: string;
  tidesPackage!: string;
  suilendPackage!: string;
  pythPackageId!: string;
  wormholePackageId!: string;
  suilendMarket!: SharedObjectInfo;
  wormholeState!: SharedObjectInfo;
  pythState!: SharedObjectInfo;
  protectedMarginAccount!: SharedObjectInfo;
  rfqAccount!: SharedObjectInfo;
  pythAccumulatorMessage!: Uint8Array;
  vaa!: Uint8Array;
  priceInfoObjects!: {
    reserveArrayIndex: bigint
    priceInfoObject: SharedObjectInfo
  }[];

  updatePriceFee!: bigint;
  tradeId!: bigint;
  nonce!: bigint;
  expiryTimeMs!: bigint;
  signature!: Uint8Array;
  inputAmount!: bigint;
  outputAmount!: bigint;
  outputFloor!: bigint;

  // Store the original RFQ for wire encoding
  private original!: Rfq;

  private constructor(data: {
    input: string
    output: string
    tidesPackage: string
    suilendPackage: string
    pythPackageId: string
    wormholePackageId: string
    suilendMarket: SharedObjectInfo
    wormholeState: SharedObjectInfo
    pythState: SharedObjectInfo
    protectedMarginAccount: SharedObjectInfo
    rfqAccount: SharedObjectInfo
    pythAccumulatorMessage: Uint8Array
    vaa: Uint8Array
    priceInfoObjects: {
      reserveArrayIndex: bigint
      priceInfoObject: SharedObjectInfo
    }[]
    updatePriceFee: bigint
    tradeId: bigint
    nonce: bigint
    expiryTimeMs: bigint
    signature: Uint8Array
    inputAmount: bigint
    outputAmount: bigint
    outputFloor: bigint
    original: Rfq
  }) {
    Object.assign(this, data);
  }

  /**
   * Converts protobuf bytes to our Quote class
   */
  static fromProtoBytes(
    input: string,
    output: string,
    tidesPackage: string,
    protoBytes: Uint8Array,
  ): Quote {
    // Decode the protobuf bytes
    const rfq = RfqDecoder.decode(protoBytes);

    // Use the existing fromProto method
    return this.fromProto(input, output, tidesPackage, rfq);
  }

  /**
   * Converts a protobuf RFQ to our Quote class
   */
  static fromProto(
    input: string,
    output: string,
    tidesPackage: string,
    rfq: Rfq,
  ): Quote {
    if (!rfq.quote) {
      throw new Error('Empty quote in RFQ');
    }

    const protoQuote = rfq.quote;
    // Extract suilend payload - we don't support native payload
    if (!protoQuote.suilendPayload) {
      throw new Error(
        'Native payload not supported - only suilend payload is supported',
      );
    }

    const suilendPayload = protoQuote.suilendPayload;

    if (!suilendPayload.pythConfig) {
      throw new Error('No pyth config found in suilend payload');
    }

    if (!suilendPayload.priceUpdatesPayload) {
      throw new Error('No price updates payload found in suilend payload');
    }

    const pythConfig = suilendPayload.pythConfig;
    const priceUpdatesPayload = suilendPayload.priceUpdatesPayload;

    if (!suilendPayload.suilendLendingMarketId) {
      throw new Error('Empty suilend market id');
    }

    if (!pythConfig.wormholeStateId) {
      throw new Error('Empty wormhole state');
    }

    if (!pythConfig.pythStateId) {
      throw new Error('Empty pyth state');
    }

    if (!protoQuote.protectedMarginAccountId) {
      throw new Error('Empty protected margin account');
    }

    if (!protoQuote.rfqAccountId) {
      throw new Error('Empty rfq account');
    }

    // Convert price info objects
    const priceInfoObjects = priceUpdatesPayload.priceUpdates.map((update) => {
      if (!update.priceInfoObjectId) {
        throw new Error('Empty price info object');
      }

      return {
        reserveArrayIndex: update.reserveArrayIndex,
        priceInfoObject: parseSharedObjectInfo(update.priceInfoObjectId),
      } satisfies Quote['priceInfoObjects'][number];
    });

    // Convert trade ID from bytes to bigint
    const tradeId = u128FromLeBytes(rfq.tradeId);
    // Convert nonce from bytes to number
    const nonce = u128FromLeBytes(protoQuote.nonce);

    return new Quote({
      input,
      output,
      tidesPackage,
      suilendPackage: objectIdFromUint8Array(suilendPayload.suilendPackageId),
      pythPackageId: objectIdFromUint8Array(pythConfig.pythPackageId),
      wormholePackageId: objectIdFromUint8Array(pythConfig.wormholePackageId),
      suilendMarket: parseSharedObjectInfo(suilendPayload.suilendLendingMarketId),
      wormholeState: parseSharedObjectInfo(pythConfig.wormholeStateId),
      pythState: parseSharedObjectInfo(pythConfig.pythStateId),
      protectedMarginAccount: parseSharedObjectInfo(protoQuote.protectedMarginAccountId),
      rfqAccount: parseSharedObjectInfo(protoQuote.rfqAccountId),
      pythAccumulatorMessage: priceUpdatesPayload.update,
      vaa: priceUpdatesPayload.vaa,
      priceInfoObjects,
      updatePriceFee: priceUpdatesPayload.updatePriceFee,
      tradeId,
      nonce,
      expiryTimeMs: protoQuote.expiryTimestampUnixMs,
      signature: protoQuote.signature,
      inputAmount: protoQuote.inputAmount,
      outputAmount: protoQuote.outputAmount,
      outputFloor: protoQuote.outputFloor,
      original: rfq,
    });
  }

  /**
   * Applies all necessary Move commands to execute the swap and transfers the resulting
   * output coin directly to the recipient address. This is a convenience method that
   * combines applySwapToTx() with a transfer operation.
   */
  applySwapToTxAndTransferCoin(
    tx: Transaction,
    req: {
      inputCoin: TransactionObjectArgument
      pythFeeCoin: TransactionObjectArgument
      recipient: string
      clock?: TransactionObjectArgument
    },
  ): void {
    const output = this.applySwapToTx(tx, {
      inputCoin: req.inputCoin,
      pythFeeCoin: req.pythFeeCoin,
      clock: req.clock,
    });

    tx.transferObjects([output], req.recipient);
  }

  /**
   * Applies all necessary Move commands to execute the swap, including price updates
   * and Suilend reserve refreshes. Returns the output balance argument for further use.
   * Use this when you want to handle the output balance yourself instead of transferring.
   */
  applySwapToTx(
    tx: Transaction,
    req: {
      inputCoin: TransactionArgument
      pythFeeCoin: TransactionArgument
      clock?: TransactionArgument
    },
  ): TransactionObjectArgument {
    const clock = req.clock || tx.object.clock();
    const suiTypeTag = '0x2::sui::SUI';

    // Update price feeds
    const updatedPrices = updatePriceFeeds(tx, {
      pythPackage: this.pythPackageId,
      wormholePackage: this.wormholePackageId,
      wormholeState: this.wormholeState,
      pythState: this.pythState,
      pythAccumulatorMessage: this.pythAccumulatorMessage,
      vaa: this.vaa,
      clock,
      feeCoin: req.pythFeeCoin,
      feeCoinType: suiTypeTag,
      priceInfoObjects: this.priceInfoObjects.map((p) => p.priceInfoObject),
      updatePriceFee: this.updatePriceFee,
    });

    // Update Suilend reserves
    const suilendMarketArg = addSharedObject(tx, this.suilendMarket, true);
    const mainPoolTypeTag = `${this.suilendPackage}::suilend::MAIN_POOL`;

    updateSuilendReservePrices(tx, {
      suilendPackage: this.suilendPackage,
      lendingMarket: suilendMarketArg,
      mainPoolTypeTag,
      priceInfoObjects: updatedPrices.map((priceInfo, idx) => ({
        reserveIdx: this.priceInfoObjects[idx].reserveArrayIndex,
        priceInfoObject: priceInfo,
      })),
      clock,
    });

    // Execute the swap
    return executeSwap(tx, {
      tidesPackage: this.tidesPackage,
      inputType: this.input,
      outputType: this.output,
      protectedMarginAccount: this.protectedMarginAccount,
      lendingMarketArg: suilendMarketArg,
      rfqAccount: this.rfqAccount,
      tradeId: this.tradeId,
      nonce: this.nonce,
      expiryTimeUnixMs: this.expiryTimeMs,
      signature: this.signature,
      inputAmount: this.inputAmount,
      outputAmount: this.outputAmount,
      outputFloor: this.outputFloor,
      inputCoinArg: req.inputCoin,
      clockArg: clock,
    });
  }

  /**
   * Returns the TypeTag for the output token that will be received from this swap.
   */
  getOutput(): string {
    return this.output;
  }

  /**
   * Returns the TypeTag for the input token that will be consumed by this swap.
   */
  getInput(): string {
    return this.input;
  }

  /**
   * Returns the exact input amount that this quote was calculated for.
   * This is the amount of input tokens that should be provided to execute the swap.
   */
  getInputAmount(): bigint {
    return this.inputAmount;
  }

  /**
   * Returns the minimum input amount needed for this RFQ to be valid.
   * Any input below this amount would result in taking less than the minimum output_floor.
   */
  getInputFloor(): bigint {
    // Calculate minimum input: (output_floor * max_input) / max_output
    return (this.outputFloor * this.inputAmount) / this.outputAmount;
  }

  /**
   * Returns the minimum output amount that can be taken from this RFQ.
   * This represents the minimum take amount for the quote.
   */
  getOutputFloor(): bigint {
    return this.outputFloor;
  }

  /**
   * Returns the maximum output amount that can be taken from this RFQ.
   * This is achieved when providing the full inputAmount().
   */
  getOutputAmount(): bigint {
    return this.outputAmount;
  }

  /**
   * Returns the expiry time of this RFQ in Unix milliseconds.
   * The quote becomes invalid after this timestamp.
   */
  getExpiryTimeMs(): bigint {
    return this.expiryTimeMs;
  }

  /**
   * Returns the total SUI fees required to update Pyth price feeds before executing the swap.
   * This cost is in addition to the input tokens and must be paid separately.
   */
  getPythPriceFees(): bigint {
    const priceObjectCount = BigInt(this.priceInfoObjects.length);
    return priceObjectCount * this.updatePriceFee;
  }

  /**
   * Calculates the output amount for a given input amount using this RFQ's exchange rate.
   * Returns null if the calculated output would be below output_floor (minimum takeable amount)
   * or above output_ceiling (maximum available amount). Use this for partial fills.
   */
  calculateOutputAmount(inputAmount: bigint): bigint | null {
    if (inputAmount === 0n) {
      return null;
    }

    // Calculate output using RFQ's exchange rate: output = (input * max_output) / max_input
    const calculatedOutput
      = (inputAmount * this.outputAmount) / this.inputAmount;

    // Check bounds: must be within [output_floor, output_amount] range
    if (
      calculatedOutput >= this.outputFloor
      && calculatedOutput <= this.outputAmount
    ) {
      return calculatedOutput;
    }

    return null;
  }

  /**
   * Checks if this quote is still valid based on the current timestamp.
   * Returns true if the quote has not expired, false otherwise.
   */
  isValid(currentTimeMs?: bigint): boolean {
    const currentTime = currentTimeMs || BigInt(Date.now());
    return currentTime < this.expiryTimeMs;
  }

  /**
   * Returns a human-readable summary of this quote for debugging/logging purposes.
   */
  getSummary(): string {
    return `Quote: ${this.inputAmount} ${this.input} → ${this.outputAmount} ${this.output} (floor: ${this.outputFloor}, expires: ${this.expiryTimeMs})`;
  }

  /**
   * Returns the original RFQ object for wire encoding purposes.
   */
  getOriginal(): Rfq {
    return this.original;
  }
}
