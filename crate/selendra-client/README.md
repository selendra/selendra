This crate provides a Rust application interface for submitting transactions to `selendra-node` chain.
Most of the [pallets](https://docs.substrate.io/reference/frame-pallets/) are common to any
[Substrate](https://github.com/paritytech/substrate) chain, but there are some unique to `selendra-node`,
e.g. [`pallets::elections::ElectionsApi`](./src/pallets/elections.rs).

## Build

Just use `cargo build` or `cargo build --release`, depends on your usecase.

## Contributions

All contributions are welcome, e.g. adding new API for pallets in `selendra-node`. 

## Metadata

`selendra-client` uses [`subxt`](https://github.com/paritytech/subxt) to communicate with a Substrate-based chain which
`selendra-node` is. In order to provide a strong type safety, it uses a manually generated file [`selendra.rs`](src/selendra.rs)
which refers to top of the `main` branch in `selendra-node` repository. See more info [here](docker/README.md).
