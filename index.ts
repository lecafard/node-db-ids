import { WasmIdGenerator } from "./pkg/db_id";

export function ISDBId(obj: unknown) {
  return obj instanceof DBIdObject;
}

export interface DBId<T extends string> {
  readonly scope: T;
  readonly id: bigint;
  readonly tag: number;
  readonly payload: string;

  toString: () => string;
}

class DBIdObject<T extends string> implements DBId<T> {
  /**
   * Do not instantiate this directly, use the DBIdProvider to create it
   */
  constructor(
    public readonly scope: T,
    public readonly id: bigint,
    public readonly tag: number,
    public readonly payload: string
  ) {}

  toString() {
    return this.payload;
  }
}

export class DBIdProviderError extends Error {}

export class DBIdProvider {
  private readonly rt;

  constructor(secret: string) {
    this.rt = new WasmIdGenerator(secret);
  }

  fromParts<T extends string>({ scope, id, tag }: { scope: T, id: bigint, tag?: number }) {
    if (tag && (tag % 1 != 0 || tag > 0xffffffff)) {
      throw new DBIdProviderError("Tag is not a 32 bit unsigned integer");
    }
    const payload = this.rt.encode(scope, id, tag || 0);
    return new DBIdObject<T>(scope, id, tag || 0, payload);
  }

  fromString<T extends string>(input: string, assert_type?: T) {
    const out = this.rt.decode(input) as [string, bigint, number];
    if (assert_type && out[0] !== assert_type) {
      throw new DBIdProviderError("ID does not match expected type");
    }
    return new DBIdObject<T>(out[0] as T, out[1], out[2], input);
  }
}
