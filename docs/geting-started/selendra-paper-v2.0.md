---
title: SEL 2.0
---

# Selendra 2.0

This document is a living document. So it will grow and change as we develop Selendra Open Network. 

Selendra is a multi-chain multi-asset interoperable nominated Proof-of-Stake system for developing and running Substrate-based and EVM compatible blockchain application. 

# Features
- On-chain tokens exchange
- Multi-assets 
- Multi-chain token utilities
- Built-in Smart Contract (EVM & Wasm)
- Bridge to EVM chains and Substrate-based networks
- 1000+ TPS on one chain
- Deflationary via fees burns

# To-do
Once the Selendra's network usage increase, millions of accounts created, and reach above 10 millions daily transactions, we will; 

- Selendra Native Parachains, create shards to increase TPS to 50,000-100,000
- Parathread to Polkadot Network
- Appchain with Near Protocol
- IBC to Cosmos Network 

# Network

| Key             | Value                                      |
| --------------- | ------------------------------------------ |
| Name, Precision | SEL, 12                                    |
| SS58 Format     | 204                                        |
| EVM chain id    | 204                                        |
| SEL ERC20/SRC20 | 0x0000000000000000000000000.....           |
| Block production| BABE                                       |
| Finality        | GRANDPA                                    |
| Block Time      | 6s                                         |
| Block Size      | 5mb                                        |

```
1 SEL = 1.000,000,000,000 Silt
1 Silt = 0.000,000,000,001 SEL 
```
# Endpoint

## Mainnet

| **Network**     | **URL**                                    |
|-----------------|--------------------------------------------|
| HTTP RPC        | https://mainnet.selendra.org               |
| Websocket       | wss://mainnet.selendra.org                 |
| EVM chain ID    | 204                                        | 

## Testnet

| **Network**     | **URL**                                    |
|-----------------|--------------------------------------------|
| HTTP RPC        | https://testnet.selendra.org               |
| Websocket       | wss://testnet.selendra.org                 |
| EVM chain ID    | 200                                        |


# Node specification

The most common way for a beginner to run a validator is on a cloud server running Linux. You may choose whatever VPS provider that your prefer, and whatever operating system you are comfortable with. 

The transactions weights in Selendra were benchmarked on standard hardware. It is recommended that validators run at least the standard hardware in order to ensure they are able to process all blocks in time. The following are not minimum requirements but if you decide to run with less than this beware that you might have performance issue.

## Standard Hardware

For the full details of the standard hardware please see here.

- CPU - Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz
- Storage - An NVMe solid state drive of 1 TB (As it should be reasonably sized to deal with blockchain growth).
- Memory - 64GB ECC.

The specs posted above are by no means the minimum specs that you could use when running a validator, however you should be aware that if you are using less you may need to toggle some extra optimizations in order to be equal to other validators that are running the standard.



