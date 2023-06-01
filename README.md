[![LOGO][selendra-logo]][selendra-homepage]

[![Unit tests][unit-tests-badge]][unit-tests]
[![E2E Tests][e2e-tests-badge]][e2e-tests]

This repository contains the Rust implementation of [Selendra Zero][selendra-homepage] blockchain node based on the [Substrate][substrate-homepage] framework.

Selendra Zero is an open-source layer 1 blockchain focused on privacy, scalability and energy efficiency. It is based on a unique, peer-reviewed consensus algorithm, SelendraBFT (as described in our [paper][selendra-bft-paper] and implemented [here][selendra-bft-link]).

Selendra node is based on a Substrate node where the default finality gadget (GRANDPA) has been replaced with SelendraBFT. Block authoring is realized with Substrate's Aura. The chain is run in periodic sesssions (900 blocks each) utilizing the Session pallet. The authorities in each session serve for both Aura and SelendraBFT authorities, and on top of that are responsible for running the Aggregator protocol producing multisignatures of finalized blocks.

### Building

Please consult the [BUILD][build] guide.

### Running

#### Connect to Selendra Zero Testnet

You can connect to global Selendra Zero Testnet network by running `selendra-node --chain=testnet`.

#### Local Network

To experiment with Selendra Node you can locally run a small blockchain network using the `run_nodes.sh` script from the root of this repository. Please consult the script or the output of `run_nodes.sh -help` for additional parameters (like the number of nodes etc.). The script starts multiple instances of Selendra Node on your local machine, so please adjust the number of nodes carefully with performance of your system in mind. By default 4 nodes are started.

You can interact with your locally running nodes using RPC (use port 9933 for node0, 9934 for node1 and so on). A more convenient alternative is to attach to it with a polkadot.js wallet app. We recommend using our fork of that app which can be found [here][selendra-polkadot-link].

### Contributing

If you would like to contribute, please fork the repository, introduce your changes and submit a pull request. All pull requests are warmly welcome.

### License

The code in this repository is licensed as follows:

- all crates under `bin` directory are licensed under the terms of the GNU GPL version 3
- all rest of the crates are licensed under the terms of Apache License 2.0.

[selendra-homepage]: https://selendrazero.org
[selendra-logo]: https://assets.selendrazero.org/branding/logo/digital/A0-horizontal-light-background.jpg
[selendra-bft-link]: https://github.com/Cardinal-Cryptography/SelendraBFT
[selendra-bft-paper]: https://arxiv.org/abs/1908.05156
[selendra-polkadot-link]: https://github.com/Cardinal-Cryptography/apps
[substrate-homepage]: https://substrate.io
[unit-tests]: https://github.com/Cardinal-Cryptography/selendra-node/actions/workflows/unit_tests.yml
[unit-tests-badge]: https://github.com/Cardinal-Cryptography/selendra-node/actions/workflows/unit_tests.yml/badge.svg
[e2e-tests]: https://github.com/Cardinal-Cryptography/selendra-node/actions/workflows/e2e-tests-main-devnet.yml
[e2e-tests-badge]: https://github.com/Cardinal-Cryptography/selendra-node/actions/workflows/e2e-tests-main-devnet.yml/badge.svg
[build]: ./BUILD.md
