import { WasmIdGenerator } from "./pkg/db_id";

export class DBId<T extends string> {
  public readonly tag: number;

  constructor(
    public readonly tbl: T,
    public readonly id: bigint,
    tag?: number,
  ) {
    if (tag && (tag % 1 != 0 || tag > 0xffffffff)) {
      throw new Error("Tag is not a 32 bit unsigned integer");
    }
    this.tag = tag || 0;
  }
}

export class DBIdProviderError extends Error {}

export class DBIdProvider {
  private readonly rt;

  constructor(secret: string) {
    this.rt = new WasmIdGenerator(secret);
  }

  encode(input: DBId<string>) {
    if (input.tag && (input.tag % 1 != 0 || input.tag > 0xffffffff)) {
      throw new DBIdProviderError("Tag is not a 32 bit unsigned integer");
    }
    return this.rt.encode(input.tbl, input.id, input.tag || 0);
  }

  decode<T extends string>(input: string, assert_type?: T) {
    const out = this.rt.decode(input) as [string, bigint, number];
    if (assert_type && out[0] !== assert_type) {
      throw new DBIdProviderError("ID does not match expected type");
    }
    return new DBId(out[0], out[1], out[2]);
  }
}
