---
title: Smart Contracts
section: Developers
order: 5
---

# Smart Contracts

Your Solidity. Your tools. Zero changes.

## EVM Contracts (Solidity)

Full compatibility. Deploy your existing contracts.

### Deploy with Hardhat

```javascript
// hardhat.config.js
module.exports = {
  networks: {
    selendra: {
      url: 'https://rpc.selendra.org',
      chainId: 1961,
      accounts: [process.env.PRIVATE_KEY]
    }
  },
  solidity: '0.8.20'
}
```

```bash
npx hardhat compile
npx hardhat run scripts/deploy.js --network selendra
```

### Deploy with Remix

1. Open Remix: https://remix.ethereum.org
2. Write or import your contract
3. Connect MetaMask to Selendra
4. Compile and deploy

Works exactly like Ethereum.

### Deploy with Foundry

```bash
forge create --rpc-url https://rpc.selendra.org \
  --private-key $PRIVATE_KEY \
  src/MyContract.sol:MyContract
```

## ink! Contracts (Wasm)

Native Substrate contracts. Lower gas. More features.

### Install

```bash
cargo install cargo-contract --force
```

### Create

```bash
cargo contract new flipper
cd flipper
```

### Build

```bash
cargo contract build
```

### Deploy

```bash
cargo contract instantiate \
  --constructor new \
  --args true \
  --suri //Alice \
  --url wss://rpc.selendra.org
```

## EVM Precompiles

Call native runtime from Solidity.

### Standard Precompiles

- 0x01: ECRecover
- 0x02: Sha256
- 0x03: Ripemd160
- 0x04: Identity
- 0x05: Modexp

### Custom Precompiles

**Sha3FIPS256 (0x0400)**
```solidity
address constant SHA3_FIPS = 0x0000000000000000000000000000000000000400;

function hash(bytes memory data) public view returns (bytes32) {
    (bool success, bytes memory result) = SHA3_FIPS.staticcall(data);
    require(success);
    return abi.decode(result, (bytes32));
}
```

**ECRecoverPublicKey (0x0401)**
```solidity
address constant ECRECOVER_PUB = 0x0000000000000000000000000000000000000401;

function recoverPublicKey(
    bytes32 hash,
    bytes memory signature
) public view returns (bytes memory) {
    (bool success, bytes memory pubkey) = ECRECOVER_PUB.staticcall(
        abi.encodePacked(hash, signature)
    );
    require(success);
    return pubkey;
}
```

## Coming Soon (v4.0)

More precompiles to call native features from Solidity.

**Staking Precompile (0x0403)**
```solidity
interface IStaking {
    function stake(address validator, uint256 amount) external;
    function unstake(uint256 amount) external;
    function claimRewards() external;
}

IStaking staking = IStaking(0x0000000000000000000000000000000000000403);
staking.stake(validator, 1000 ether);
```

**Oracle Precompile (0x0402)**
```solidity
interface IOracle {
    function getPrice(uint32 assetId) external view returns (uint256);
    function getTimestamp(uint32 assetId) external view returns (uint256);
}

IOracle oracle = IOracle(0x0000000000000000000000000000000000000402);
uint256 price = oracle.getPrice(1); // SEL/USD
```

**Governance Precompile (0x0404)**
```solidity
interface IGovernance {
    function propose(bytes32 proposalHash, uint256 value) external;
    function vote(uint256 proposalId, bool aye) external;
}
```

## Unified Accounts

One account. EVM and native.

```solidity
contract CrossRuntime {
    // EVM address = Native address
    // No bridge needed

    function transferToNative(address to, uint256 amount) public {
        // This address works for both EVM and native
        payable(to).transfer(amount);
    }
}
```

## Contract Limits

Current runtime allows contracts to call:
- Staking operations
- Nomination pools
- Balance transfers (coming soon)

Can NOT call:
- Sudo (admin functions)
- Treasury
- Governance (yet)

## Gas Optimization

Lower than Ethereum. Much lower.

**Storage**
- 0.00004 SEL per byte
- vs 20,000 gas on Ethereum

**Transactions**
- Transfer: ~21,000 gas
- Token swap: ~150,000 gas
- NFT mint: ~80,000 gas

**Block Limit**
- 15M gas per block
- Target: 50M in v4.0

## Best Practices

**Test first**
```bash
# Use testnet
selendra deploy --network testnet
```

**Verify contracts**
```bash
selendra verify <address> --network mainnet
```

**Use established libraries**
```bash
npm install @openzeppelin/contracts
```

**Audit before mainnet**
Large contracts: Get audited.

## Examples

**ERC20 Token**
```bash
git clone https://github.com/selendra/erc20-template
npm install
npm run deploy
```

**NFT Collection**
```bash
git clone https://github.com/selendra/nft-template
npm install
npm run deploy
```

**DeFi Protocol**
```bash
git clone https://github.com/selendra/defi-template
npm install
npm run deploy
```

## Resources

- OpenZeppelin: https://openzeppelin.com/contracts
- Hardhat: https://hardhat.org
- Remix: https://remix.ethereum.org
- ink! Docs: https://use.ink

## Get Help

Stuck? Ask in Discord.

https://discord.gg/selendra
