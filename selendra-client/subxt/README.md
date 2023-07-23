# subxt &middot; ![build](https://github.com/paritytech/subxt/workflows/Rust/badge.svg) [![Latest Version](https://img.shields.io/crates/v/subxt.svg)](https://crates.io/crates/subxt) [![Documentation](https://docs.rs/subxt/badge.svg)](https://docs.rs/subxt)

A library to **sub**mit e**xt**rinsics to a [substrate](https://github.com/paritytech/substrate) node via RPC.

## Usage

Take a look in the [examples](./examples/examples) folder for various `subxt` usage examples.

### Downloading metadata from a Substrate node

Use the [`subxt-cli`](./cli) tool to download the metadata for your target runtime from a node.

1. Install:
```bash
cargo install subxt-cli
```
2. Save the encoded metadata to a file:
```bash
subxt metadata -f bytes > metadata.scale
```

This defaults to querying the metadata of a locally running node on the default `http://localhost:9933/`. If querying
a different node then the `metadata` command accepts a `--url` argument.

## Subxt Documentation

For more details regarding utilizing subxt, please visit the [documentation](https://docs.rs/subxt/latest/subxt/).

## Integration Testing

Most tests require a running substrate node to communicate with. This is done by spawning an instance of the
substrate node per test. It requires an up-to-date `substrate` executable on your path.

This can be installed from source via cargo:

```bash
cargo install --git https://github.com/paritytech/substrate node-cli --force
```

**Alternatives**

[substrate-api-client](https://github.com/scs/substrate-api-client) provides similar functionality.

#### License

The entire code within this repository is dual licensed under the _GPL-3.0_ or _Apache-2.0_ licenses. See [the LICENSE](./LICENSE) file for more details.

Please <a href="https://www.parity.io/contact/">contact us</a> if you have questions about the licensing of our products.
