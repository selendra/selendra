# API for [selendra-node](https://github.com/selendra/selendra) chain.

This crate provides a Rust application interface for submitting transactions to `selendra-node` chain.
Most of the [pallets](https://docs.substrate.io/reference/frame-pallets/) are common to any
[Substrate](https://github.com/paritytech/substrate) chain, but there are some unique to `aleph-node`,
e.g. [`pallets::elections::ElectionsApi`](./src/pallets/elections.rs).

## Build

Just use `cargo build` or `cargo build --release`, depends on your usecase.


## Metadata

`selendra-client` uses [`subxt`](https://github.com/paritytech/subxt) to communicate with a Substrate-based chain which
`selendra-node` is. In order to provide a strong type safety.


# LICENSE

Code is implement from [Aleph Zero](https://github.com/Cardinal-Cryptography/aleph-node/tree/main/aleph-client) under [Apache 2.0 License](LICENSE).