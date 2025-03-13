# Build Your First dApp on Selendra

This guide will walk you through building a simple decentralized application (dApp) on Selendra. You'll create a token voting application that combines smart contracts with a web interface.

## Prerequisites

Before you begin, make sure you have:
- Completed the [Developer Environment Setup](./dev-environment-setup.md)
- Basic knowledge of JavaScript/TypeScript
- Basic understanding of React
- Familiarity with Solidity (for EVM) or Rust (for WASM)

## Project Overview

We'll build a simple voting dApp with these features:
- Create voting topics
- Cast votes using tokens
- View voting results

## Step 1: Set Up Project Structure

Start by creating a new project using the Selendra starter template:

```bash
# Clone the starter template
git clone https://github.com/selendra/selendra-evm-starter.git token-voting-dapp
cd token-voting-dapp

# Install dependencies
yarn install
```

The template includes:
- Hardhat for smart contract development
- React for the frontend
- Ethers.js for blockchain interaction
- Tailwind CSS for styling

## Step 2: Create the Smart Contract

Create a new file `contracts/TokenVoting.sol`:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract TokenVoting is Ownable {
    struct Proposal {
        string title;
        string description;
        uint256 votesFor;
        uint256 votesAgainst;
        bool active;
        uint256 endTime;
    }
    
    IERC20 public token;
    Proposal[] public proposals;
    
    // Track votes to prevent double voting
    mapping(address => mapping(uint256 => bool)) public hasVoted;
    
    event ProposalCreated(uint256 proposalId, string title, uint256 endTime);
    event VoteCast(uint256 proposalId, address voter, bool support, uint256 amount);
    
    constructor(address _token) {
        token = IERC20(_token);
    }
    
    function createProposal(string memory _title, string memory _description, uint256 _durationDays) external onlyOwner {
        proposals.push(Proposal({
            title: _title,
            description: _description,
            votesFor: 0,
            votesAgainst: 0,
            active: true,
            endTime: block.timestamp + (_durationDays * 1 days)
        }));
        
        emit ProposalCreated(proposals.length - 1, _title, block.timestamp + (_durationDays * 1 days));
    }
    
    function vote(uint256 _proposalId, bool _support, uint256 _amount) external {
        require(_proposalId < proposals.length, "Invalid proposal ID");
        require(proposals[_proposalId].active, "Proposal not active");
        require(block.timestamp < proposals[_proposalId].endTime, "Voting period ended");
        require(!hasVoted[msg.sender][_proposalId], "Already voted");
        require(token.balanceOf(msg.sender) >= _amount, "Insufficient token balance");
        
        // Transfer tokens to contract for voting
        token.transferFrom(msg.sender, address(this), _amount);
        
        // Record vote
        if (_support) {
            proposals[_proposalId].votesFor += _amount;
        } else {
            proposals[_proposalId].votesAgainst += _amount;
        }
        
        hasVoted[msg.sender][_proposalId] = true;
        
        emit VoteCast(_proposalId, msg.sender, _support, _amount);
    }
    
    function closeProposal(uint256 _proposalId) external onlyOwner {
        require(_proposalId < proposals.length, "Invalid proposal ID");
        require(proposals[_proposalId].active, "Proposal already closed");
        
        proposals[_proposalId].active = false;
    }
    
    function getProposal(uint256 _proposalId) external view 
        returns (
            string memory title,
            string memory description,
            uint256 votesFor,
            uint256 votesAgainst,
            bool active,
            uint256 endTime
        ) 
    {
        require(_proposalId < proposals.length, "Invalid proposal ID");
        Proposal memory p = proposals[_proposalId];
        return (p.title, p.description, p.votesFor, p.votesAgainst, p.active, p.endTime);
    }
    
    function getProposalCount() external view returns (uint256) {
        return proposals.length;
    }
}
```

## Step 3: Create a Simple ERC20 Token

Create a file `contracts/VotingToken.sol`:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract VotingToken is ERC20 {
    constructor(uint256 initialSupply) ERC20("Voting Token", "VOTE") {
        _mint(msg.sender, initialSupply * 10**decimals());
    }
    
    function mint(address to, uint256 amount) public {
        _mint(to, amount);
    }
}
```

## Step 4: Deploy the Contracts

Create a deployment script in `scripts/deploy.js`:

```javascript
const hre = require("hardhat");

async function main() {
  // Deploy the token
  const VotingToken = await hre.ethers.getContractFactory("VotingToken");
  const initialSupply = 1000000; // 1 million tokens
  const token = await VotingToken.deploy(initialSupply);
  await token.deployed();
  console.log("VotingToken deployed to:", token.address);

  // Deploy the voting contract
  const TokenVoting = await hre.ethers.getContractFactory("TokenVoting");
  const voting = await TokenVoting.deploy(token.address);
  await voting.deployed();
  console.log("TokenVoting deployed to:", voting.address);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
```

Deploy to the local Selendra node:

```bash
npx hardhat run scripts/deploy.js --network selendraLocal
```

## Step 5: Build the Frontend

### Create React Components

First, create the main components. In `src/components/`:

#### VotingForm.js
```jsx
import { useState } from 'react';
import { ethers } from 'ethers';

function VotingForm({ votingContract, tokenContract, onVoteSubmitted }) {
  const [proposalId, setProposalId] = useState('');
  const [amount, setAmount] = useState('');
  const [support, setSupport] = useState(true);
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (!votingContract || !tokenContract) return;

    try {
      setLoading(true);
      
      // First approve tokens for the voting contract
      const amountWei = ethers.utils.parseEther(amount);
      const approveTx = await tokenContract.approve(votingContract.address, amountWei);
      await approveTx.wait();
      
      // Then cast the vote
      const voteTx = await votingContract.vote(proposalId, support, amountWei);
      await voteTx.wait();
      
      // Reset form and notify parent
      setProposalId('');
      setAmount('');
      if (onVoteSubmitted) onVoteSubmitted();
    } catch (error) {
      console.error("Error voting:", error);
      alert(`Error: ${error.message || error}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="bg-white p-6 rounded-lg shadow-md">
      <h2 className="text-xl font-semibold mb-4">Cast Your Vote</h2>
      
      <form onSubmit={handleSubmit}>
        <div className="mb-4">
          <label className="block text-sm font-medium mb-1">Proposal ID</label>
          <input
            type="number"
            value={proposalId}
            onChange={(e) => setProposalId(e.target.value)}
            className="w-full p-2 border rounded"
            required
          />
        </div>
        
        <div className="mb-4">
          <label className="block text-sm font-medium mb-1">Amount of Tokens</label>
          <input
            type="text"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            className="w-full p-2 border rounded"
            placeholder="0.0"
            required
          />
        </div>
        
        <div className="mb-4">
          <label className="block text-sm font-medium mb-1">Vote</label>
          <div className="flex gap-4">
            <label className="flex items-center">
              <input
                type="radio"
                checked={support}
                onChange={() => setSupport(true)}
                className="mr-2"
              />
              Support
            </label>
            <label className="flex items-center">
              <input
                type="radio"
                checked={!support}
                onChange={() => setSupport(false)}
                className="mr-2"
              />
              Against
            </label>
          </div>
        </div>
        
        <button
          type="submit"
          disabled={loading}
          className="w-full bg-blue-600 text-white py-2 rounded hover:bg-blue-700 disabled:bg-gray-400"
        >
          {loading ? 'Processing...' : 'Submit Vote'}
        </button>
      </form>
    </div>
  );
}

export default VotingForm;
```

#### ProposalList.js
```jsx
import { useState, useEffect } from 'react';
import { ethers } from 'ethers';

function ProposalList({ votingContract }) {
  const [proposals, setProposals] = useState([]);
  const [loading, setLoading] = useState(true);
  
  const fetchProposals = async () => {
    if (!votingContract) return;

    try {
      setLoading(true);
      const count = await votingContract.getProposalCount();
      
      const proposalData = [];
      for (let i = 0; i < count; i++) {
        const proposal = await votingContract.getProposal(i);
        proposalData.push({
          id: i,
          title: proposal.title,
          description: proposal.description,
          votesFor: ethers.utils.formatEther(proposal.votesFor),
          votesAgainst: ethers.utils.formatEther(proposal.votesAgainst),
          active: proposal.active,
          endTime: new Date(proposal.endTime.toNumber() * 1000).toLocaleString()
        });
      }
      
      setProposals(proposalData);
    } catch (error) {
      console.error("Error fetching proposals:", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchProposals();
  }, [votingContract]);

  if (loading) {
    return <div className="text-center py-4">Loading proposals...</div>;
  }

  if (proposals.length === 0) {
    return <div className="text-center py-4">No proposals found</div>;
  }

  return (
    <div className="bg-white rounded-lg shadow-md overflow-hidden">
      <h2 className="text-xl font-semibold p-4 border-b">Active Proposals</h2>
      
      <div className="divide-y">
        {proposals.map(proposal => (
          <div key={proposal.id} className="p-4">
            <div className="flex justify-between">
              <h3 className="text-lg font-medium">{proposal.title}</h3>
              <span className={`px-2 py-1 rounded text-sm ${proposal.active ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'}`}>
                {proposal.active ? 'Active' : 'Closed'}
              </span>
            </div>
            
            <p className="text-gray-600 mt-1">{proposal.description}</p>
            
            <div className="mt-3 grid grid-cols-2 gap-4">
              <div>
                <div className="text-sm text-gray-500">Votes For</div>
                <div className="font-semibold">{proposal.votesFor} VOTE</div>
              </div>
              <div>
                <div className="text-sm text-gray-500">Votes Against</div>
                <div className="font-semibold">{proposal.votesAgainst} VOTE</div>
              </div>
            </div>
            
            <div className="mt-2 text-sm text-gray-500">
              Ends: {proposal.endTime}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export default ProposalList;
```

#### CreateProposal.js (Admin Only)
```jsx
import { useState } from 'react';

function CreateProposal({ votingContract, isAdmin, onProposalCreated }) {
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [duration, setDuration] = useState(7); // Default 7 days
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (!votingContract || !isAdmin) return;

    try {
      setLoading(true);
      const tx = await votingContract.createProposal(title, description, duration);
      await tx.wait();
      
      setTitle('');
      setDescription('');
      setDuration(7);
      
      if (onProposalCreated) onProposalCreated();
    } catch (error) {
      console.error("Error creating proposal:", error);
      alert(`Error: ${error.message || error}`);
    } finally {
      setLoading(false);
    }
  };

  if (!isAdmin) {
    return null; // Only show to admin
  }

  return (
    <div className="bg-white p-6 rounded-lg shadow-md">
      <h2 className="text-xl font-semibold mb-4">Create New Proposal</h2>
      
      <form onSubmit={handleSubmit}>
        <div className="mb-4">
          <label className="block text-sm font-medium mb-1">Title</label>
          <input
            type="text"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="w-full p-2 border rounded"
            required
          />
        </div>
        
        <div className="mb-4">
          <label className="block text-sm font-medium mb-1">Description</label>
          <textarea
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            className="w-full p-2 border rounded h-24"
            required
          />
        </div>
        
        <div className="mb-4">
          <label className="block text-sm font-medium mb-1">Duration (days)</label>
          <input
            type="number"
            value={duration}
            onChange={(e) => setDuration(parseInt(e.target.value))}
            className="w-full p-2 border rounded"
            min="1"
            required
          />
        </div>
        
        <button
          type="submit"
          disabled={loading}
          className="w-full bg-green-600 text-white py-2 rounded hover:bg-green-700 disabled:bg-gray-400"
        >
          {loading ? 'Creating...' : 'Create Proposal'}
        </button>
      </form>
    </div>
  );
}

export default CreateProposal;
```

### Update App.js
Edit `src/App.js` to integrate these components:

```jsx
import { useState, useEffect } from 'react';
import { ethers } from 'ethers';
import VotingForm from './components/VotingForm';
import ProposalList from './components/ProposalList';
import CreateProposal from './components/CreateProposal';
import VotingTokenABI from './contracts/VotingToken.json';
import TokenVotingABI from './contracts/TokenVoting.json';

// Replace with your deployed contract addresses
const TOKEN_ADDRESS = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
const VOTING_ADDRESS = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512";

function App() {
  const [provider, setProvider] = useState(null);
  const [signer, setSigner] = useState(null);
  const [account, setAccount] = useState('');
  const [tokenContract, setTokenContract] = useState(null);
  const [votingContract, setVotingContract] = useState(null);
  const [tokenBalance, setTokenBalance] = useState('0');
  const [isAdmin, setIsAdmin] = useState(false);

  async function connectWallet() {
    try {
      if (window.ethereum) {
        const provider = new ethers.providers.Web3Provider(window.ethereum);
        await window.ethereum.request({ method: 'eth_requestAccounts' });
        const signer = provider.getSigner();
        const address = await signer.getAddress();

        setProvider(provider);
        setSigner(signer);
        setAccount(address);

        const token = new ethers.Contract(TOKEN_ADDRESS, VotingTokenABI.abi, signer);
        const voting = new ethers.Contract(VOTING_ADDRESS, TokenVotingABI.abi, signer);
        
        setTokenContract(token);
        setVotingContract(voting);
        
        // Check if connected account is the admin
        const owner = await voting.owner();
        setIsAdmin(owner.toLowerCase() === address.toLowerCase());
        
        // Get token balance
        const balance = await token.balanceOf(address);
        setTokenBalance(ethers.utils.formatEther(balance));
        
        // Setup event listeners for balance updates
        token.on('Transfer', (from, to, amount, event) => {
          if (from.toLowerCase() === address.toLowerCase() || to.toLowerCase() === address.toLowerCase()) {
            updateBalance(address, token);
          }
        });
      } else {
        alert('Please install MetaMask to use this dApp!');
      }
    } catch (error) {
      console.error('Error connecting wallet:', error);
    }
  }
  
  async function updateBalance(address, tokenContract) {
    const balance = await tokenContract.balanceOf(address);
    setTokenBalance(ethers.utils.formatEther(balance));
  }
  
  async function requestTokens() {
    if (!tokenContract || !isAdmin) return;
    
    try {
      const tx = await tokenContract.mint(account, ethers.utils.parseEther('100'));
      await tx.wait();
      updateBalance(account, tokenContract);
    } catch (error) {
      console.error('Error minting tokens:', error);
    }
  }
  
  useEffect(() => {
    connectWallet();
  }, []);

  return (
    <div className="min-h-screen bg-gray-100 py-8">
      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
        <header className="mb-8">
          <h1 className="text-3xl font-bold text-center mb-4">Selendra Token Voting</h1>
          {!account ? (
            <button 
              onClick={connectWallet}
              className="mx-auto block bg-blue-600 text-white py-2 px-4 rounded hover:bg-blue-700"
            >
              Connect Wallet
            </button>
          ) : (
            <div className="flex justify-between items-center bg-white p-4 rounded-lg shadow-sm">
              <div>
                <div className="text-sm text-gray-500">Connected Account</div>
                <div className="font-mono text-sm">{account}</div>
              </div>
              <div>
                <div className="text-sm text-gray-500">Your Balance</div>
                <div className="font-semibold">{tokenBalance} VOTE</div>
              </div>
              {isAdmin && (
                <button 
                  onClick={requestTokens}
                  className="bg-green-600 text-white py-1 px-3 rounded hover:bg-green-700 text-sm"
                >
                  Get Test Tokens
                </button>
              )}
            </div>
          )}
        </header>
        
        {account && (
          <div className="grid md:grid-cols-2 gap-8">
            <div>
              <ProposalList 
                votingContract={votingContract} 
              />
              
              <div className="mt-8">
                <CreateProposal 
                  votingContract={votingContract}
                  isAdmin={isAdmin}
                  onProposalCreated={() => {}}
                />
              </div>
            </div>
            
            <div>
              <VotingForm 
                votingContract={votingContract}
                tokenContract={tokenContract}
                onVoteSubmitted={() => {}}
              />
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
```

## Step 6: Run Your dApp

1. Make sure your local Selendra node is running:
```bash
docker run -p 9944:9944 -p 9933:9933 selendrachain/selendra:latest --dev --ws-external
```

2. Start the React development server:
```bash
yarn start
```

3. Open your browser and navigate to `http://localhost:3000`

## Testing Your dApp

1. Connect your MetaMask to the local Selendra network (set up in the [Developer Environment Setup](./dev-environment-setup.md))
2. If you're the contract deployer (admin), you can create proposals and mint test tokens
3. Create a proposal with a title, description, and duration
4. Vote on proposals by specifying the proposal ID, amount of tokens, and your stance

## Next Steps

Now that you've built a basic dApp, you can extend it with:

1. **Enhanced UI/UX**: Improve the user interface and experience
2. **Multiple Vote Types**: Add different voting mechanisms (quadratic, delegation)
3. **Result Visualization**: Add charts and graphs for voting results
4. **Timelock Execution**: Implement timelock for executing passed proposals
5. **Unit Tests**: Add comprehensive tests for your contracts

## Troubleshooting

- **Contract Deployment Issues**: Ensure your Selendra node is running and properly configured
- **Transaction Errors**: Check the console for specific error messages
- **MetaMask Connection**: Make sure MetaMask is properly connected to your local Selendra node
- **Gas Fees**: Ensure your account has enough SEL for transaction fees

## Learn More

- [EVM Smart Contracts Guide](./evm-contracts.md) for more details on Solidity development
- [Contract Testing Guide](./contract-testing.md) to learn how to test your contracts
- [DApp Front-End Development](./dapp-frontend.md) for UI/UX best practices 