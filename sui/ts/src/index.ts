import type { grpc } from '@improbable-eng/grpc-web';
import type { DeploymentInfo } from './_codegen/tides/sui/common/v1/v1.pb';
import type { Rfq } from './_codegen/tides/sui/hub/v1/v1.pb.ts';
import {
  DeploymentInfoRequest,
  GrpcWebImpl,
  HubServiceClientImpl,
  QuoteTradeExactInRequest,
  QuoteTradeExactOutRequest,
  Rfq as RfqDecoder,
} from './_codegen/tides/sui/hub/v1/v1.pb';
import { Quote } from './quote';
import { objectIdFromUint8Array } from './utils';

export class TidesClient {
  hubClient: HubServiceClientImpl;
  deploymentInfo: DeploymentInfo;
  chainId: string;

  private constructor(
    hubClient: HubServiceClientImpl,
    deploymentInfo: DeploymentInfo,
    chainId: string,
  ) {
    this.hubClient = hubClient;
    this.deploymentInfo = deploymentInfo;
    this.chainId = chainId;
  }

  public static async connect(
    endpoint: string,
    options?: {
      transport?: grpc.TransportFactory
      streamingTransport?: grpc.TransportFactory
      debug?: boolean
      metadata?: grpc.Metadata
      upStreamRetryCodes?: number[]
    },
  ): Promise<TidesClient> {
    const rpc = new GrpcWebImpl(endpoint, options || {});
    const hubClient = new HubServiceClientImpl(rpc);
    const deploymentInfo = await hubClient.deploymentInfo(
      DeploymentInfoRequest.fromPartial({}),
    );
    if (!deploymentInfo.deploymentInfo) {
      throw new Error(
        'DeploymentInfoResponse: DeploymentInfoRequest not found',
      );
    }

    return new TidesClient(
      hubClient,
      deploymentInfo.deploymentInfo,
      deploymentInfo.chainId,
    );
  }

  /**
   * Gets a trading quote for swapping an exact input amount to the output token.
   * Specify minOutputAmount to set a slippage limit, or undefined to accept any output amount.
   * Returns null if no viable trade route exists for the given parameters.
   */
  async getSwapExactInQuote(params: {
    inputType: string
    inputAmount: bigint
    outputType: string
    minOutputAmount?: bigint
    quoteExpiry?: Date
    rfqTimeout?: Date
  }): Promise<Quote | null> {
    const req = QuoteTradeExactInRequest.create(params);
    const res = await this.hubClient.quoteTradeExactIn(req);
    return this.processQuoteResponse(
      res.rfq,
      params.inputType,
      params.outputType,
    );
  }

  /**
   * Gets a trading quote for receiving an exact output amount from the input token.
   * Specify maxInputAmount to limit how much you're willing to spend, or undefined for no limit.
   * Returns null if no viable trade route exists for the given parameters.
   */
  async getSwapExactOutQuote(params: {
    outputType: string
    outputAmount: bigint
    inputType: string
    maxInputAmount?: bigint
    quoteExpiry?: Date
    rfqTimeout?: Date
  }): Promise<Quote | null> {
    const req = QuoteTradeExactOutRequest.create(params);
    const res = await this.hubClient.quoteTradeExactOut(req);
    return this.processQuoteResponse(
      res.rfq,
      params.inputType,
      params.outputType,
    );
  }

  /**
   * Processes a quote response and converts it to a Quote object
   * @private
   */
  private processQuoteResponse(
    rfq: Rfq | undefined,
    inputType: string,
    outputType: string,
  ): Quote | null {
    if (!rfq || !rfq.quote) {
      return null;
    }

    const tidesPackageId = objectIdFromUint8Array(this.deploymentInfo.package);

    return Quote.fromProto(inputType, outputType, tidesPackageId, rfq);
  }

  /**
   * Encodes the quote to bytes
   */
  encodeQuoteToBytes(quote: Quote): Uint8Array {
    return RfqDecoder.encode(quote.getOriginal()).finish();
  }

  /**
   * Decodes the quote from bytes
   */
  decodeQuoteFromBytes(
    inputType: string,
    outputType: string,
    bytes: Uint8Array,
  ): Quote | null {
    try {
      const decoded = RfqDecoder.decode(bytes);
      return this.processQuoteResponse(decoded, inputType, outputType);
    } catch (error) {
      throw new Error(`Failed to decode quote from bytes: ${error}`);
    }
  }
}

// Export only the public API - Quote class and TidesClient
export { Quote } from './quote.ts';
