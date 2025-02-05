# node-db-ids

A library to generate encrypted database IDs. It uses a WASM-based implementation of AES for
insanely fast performance.

The primary use case for this library is to obfuscate internal incrementing database IDs when
exposing them to external systems or customers. By encrypting your IDs, you can maintain database
efficiency (keeping IDs as int or bigint) while reducing the risk of enumeration attacks.

Also the IDs generated are self-describing and shorter than UUIDs.

## Usage

### Creating a Provider

Create a DBIdProvider instance with a secret key:
```js
const { DBIdProvider } = require('node-db-id');

// Create a provider with the secret key "aaaaaaa"
const provider = new DBIdProvider("aaaaaaa");
```

### Encrypting an ID

Encrypt up to a 96 bit ID using the secret key and a scope.

```js
const encrypted = provider.fromParts({ id: 1n, scope: "tbl" });
console.log("Encrypted:", encrypted);
```

### Decrypting an ID

Decode a serialized ID using the secret key. This will return an object containing the scope
and the original ID.

```js
const decrypted = provider.fromString(encrypted.payload);
console.log("Decrypted:", decrypted);
```

## Benchmark

A single-core benchmark was run using Node.js v23.4.0 on a Ryzen 9 5900X:

```js
const { DBIdProvider } = require('node-db-id');
(function () {
  const a = new DBIdProvider("aaaaaaa");
  const encrypted = a.fromParts({ id: 1n, scope: "tbl" });
  console.log("Encrypted:", encrypted);
  const decrypted = a.fromString(encrypted.payload);
  console.log("Decrypted:", decrypted);

  let start = performance.now();
  for (let i = 0; i < 10 ** 7; i++) {
    a.fromParts({ id: 1n, scope: "tbl" });
  }
  console.log("Encrypt (ops/s):", 10 ** 10 / (performance.now() - start));

  start = performance.now();
  for (let i = 0; i < 10 ** 7; i++) {
    a.fromString(encrypted.payload);
  }
  console.log("Decrypt (ops/s):", 10 ** 10 / (performance.now() - start));
})();
```

### Results

```
Encrypted: DBId {
  scope: 'tbl',
  id: 1n,
  payload: 'tbl_uoXRuMlMYhM2esa1eGZey'
}
Decrypted: DBId {
  scope: 'tbl',
  id: 1n,
  payload: 'tbl_uoXRuMlMYhM2esa1eGZey'
}
Encrypt (ops/s): 1394990.0318232172
Decrypt (ops/s): 1081373.6576548016
```

## Caveats
- **Key Rotation**: Since IDs need to be durable, rotating the secret keys used for encryption can
  be challenging, if not impossible. To mitigate this, consider using a separate secret key for
  each scope.
- **Security**: Ensure your secret keys are stored securely and never exposed in client-side code.

## License
This project is licensed under the MIT License. See the LICENSE file for details.