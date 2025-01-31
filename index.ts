import { WasmIdGenerator } from "./pkg/db_id";

export class DBId<T extends string> implements DBId<T> {
  /**
   * Do not instantiate this directly, use the DBIdProvider to create it
   */
  constructor(
    public readonly role: T,
    public readonly id: bigint,
    public readonly tag: number,
    public readonly str: string
  ) {}

  toString() {
    return this.str;
  }
}

export class DBIdProviderError extends Error {}

export class DBIdProvider {
  private readonly rt;

  constructor(secret: string) {
    this.rt = new WasmIdGenerator(secret);
  }

  fromParts<T extends string>({ role, id, tag }: { role: T, id: bigint, tag?: number }) {
    if (tag && (tag % 1 != 0 || tag > 0xffffffff)) {
      throw new DBIdProviderError("Tag is not a 32 bit unsigned integer");
    }
    const str = this.rt.encode(role, id, tag || 0);
    return new DBId<T>(role, id, tag || 0, str);
  }

  fromString<T extends string>(input: string, assert_type?: T) {
    const out = this.rt.decode(input) as [string, bigint, number];
    if (assert_type && out[0] !== assert_type) {
      throw new DBIdProviderError("ID does not match expected type");
    }
    return new DBId<T>(out[0] as T, out[1], out[2], input);
  }
}
