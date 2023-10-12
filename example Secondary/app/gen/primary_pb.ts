// @generated by protoc-gen-es v1.3.1 with parameter "target=ts"
// @generated from file primary.proto (package verified.primary.v1, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import type { BinaryReadOptions, FieldList, JsonReadOptions, JsonValue, PartialMessage, PlainMessage } from "@bufbuild/protobuf";
import { Message, proto3, protoInt64 } from "@bufbuild/protobuf";

/**
 * @generated from message verified.primary.v1.Pools
 */
export class Pools extends Message<Pools> {
  /**
   * @generated from field: repeated verified.primary.v1.Pool pools = 1;
   */
  pools: Pool[] = [];

  constructor(data?: PartialMessage<Pools>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "verified.primary.v1.Pools";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "pools", kind: "message", T: Pool, repeated: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): Pools {
    return new Pools().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): Pools {
    return new Pools().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): Pools {
    return new Pools().fromJsonString(jsonString, options);
  }

  static equals(a: Pools | PlainMessage<Pools> | undefined, b: Pools | PlainMessage<Pools> | undefined): boolean {
    return proto3.util.equals(Pools, a, b);
  }
}

/**
 * @generated from message verified.primary.v1.Pool
 */
export class Pool extends Message<Pool> {
  /**
   * @generated from field: bytes pool_address = 1;
   */
  poolAddress = new Uint8Array(0);

  constructor(data?: PartialMessage<Pool>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "verified.primary.v1.Pool";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "pool_address", kind: "scalar", T: 12 /* ScalarType.BYTES */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): Pool {
    return new Pool().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): Pool {
    return new Pool().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): Pool {
    return new Pool().fromJsonString(jsonString, options);
  }

  static equals(a: Pool | PlainMessage<Pool> | undefined, b: Pool | PlainMessage<Pool> | undefined): boolean {
    return proto3.util.equals(Pool, a, b);
  }
}

/**
 * @generated from message verified.primary.v1.Subscriptions
 */
export class Subscriptions extends Message<Subscriptions> {
  /**
   * @generated from field: repeated verified.primary.v1.Subscription subscriptions = 1;
   */
  subscriptions: Subscription[] = [];

  constructor(data?: PartialMessage<Subscriptions>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "verified.primary.v1.Subscriptions";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "subscriptions", kind: "message", T: Subscription, repeated: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): Subscriptions {
    return new Subscriptions().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): Subscriptions {
    return new Subscriptions().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): Subscriptions {
    return new Subscriptions().fromJsonString(jsonString, options);
  }

  static equals(a: Subscriptions | PlainMessage<Subscriptions> | undefined, b: Subscriptions | PlainMessage<Subscriptions> | undefined): boolean {
    return proto3.util.equals(Subscriptions, a, b);
  }
}

/**
 * @generated from message verified.primary.v1.Subscription
 */
export class Subscription extends Message<Subscription> {
  /**
   * @generated from field: bytes assetIn_address = 1;
   */
  assetInAddress = new Uint8Array(0);

  /**
   * @generated from field: bytes assetOut_address = 2;
   */
  assetOutAddress = new Uint8Array(0);

  /**
   * @generated from field: uint64 subscription_amount = 3;
   */
  subscriptionAmount = protoInt64.zero;

  /**
   * @generated from field: bytes investor_address = 4;
   */
  investorAddress = new Uint8Array(0);

  /**
   * @generated from field: uint64 price = 5;
   */
  price = protoInt64.zero;

  /**
   * @generated from field: uint64 execution_date = 6;
   */
  executionDate = protoInt64.zero;

  constructor(data?: PartialMessage<Subscription>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "verified.primary.v1.Subscription";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "assetIn_address", kind: "scalar", T: 12 /* ScalarType.BYTES */ },
    { no: 2, name: "assetOut_address", kind: "scalar", T: 12 /* ScalarType.BYTES */ },
    { no: 3, name: "subscription_amount", kind: "scalar", T: 4 /* ScalarType.UINT64 */ },
    { no: 4, name: "investor_address", kind: "scalar", T: 12 /* ScalarType.BYTES */ },
    { no: 5, name: "price", kind: "scalar", T: 4 /* ScalarType.UINT64 */ },
    { no: 6, name: "execution_date", kind: "scalar", T: 4 /* ScalarType.UINT64 */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): Subscription {
    return new Subscription().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): Subscription {
    return new Subscription().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): Subscription {
    return new Subscription().fromJsonString(jsonString, options);
  }

  static equals(a: Subscription | PlainMessage<Subscription> | undefined, b: Subscription | PlainMessage<Subscription> | undefined): boolean {
    return proto3.util.equals(Subscription, a, b);
  }
}

