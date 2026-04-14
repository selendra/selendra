// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {Script, console} from "forge-std/Script.sol";
import {VerifyingPaymaster} from "src/erc4337/VerifyingPaymaster.sol";
import {ReserveVault} from "src/stablecoin/ReserveVault.sol";
import {WrappedUSDT} from "src/stablecoin/WrappedUSDT.sol";

contract Verify is Script {
    function run() external {
        // Deployed addresses - update after deployment
        address paymasterAddr = 0x5FbDB2315678afecb367f032d93F642f64180aa3; // CHANGE THIS
        address vaultAddr = 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512; // CHANGE THIS
        address wusdtAddr = 0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0; // CHANGE THIS

        VerifyingPaymaster paymaster = VerifyingPaymaster(payable(paymasterAddr));
        ReserveVault vault = ReserveVault(vaultAddr);
        WrappedUSDT wusdt = WrappedUSDT(wusdtAddr);

        console.log("\n=== Contract Verification ===");
        console.log("VerifyingPaymaster:");
        console.log("  Owner:", paymaster.owner());
        console.log("  TrustedSigner:", paymaster.trustedSigner());
        console.log("  EntryPoint:", address(paymaster.entryPoint()));
        
        console.log("\nReserveVault:");
        console.log("  Owner:", vault.owner());
        console.log("  WrappedUSDT:", vault.wrappedUSDT());
        console.log("  USDT:", address(vault.usdt()));
        console.log("  TotalReserves:", vault.totalReserves());
        console.log("  TotalWrapped:", vault.totalWrapped());
        
        console.log("\nWrappedUSDT:");
        console.log("  Owner:", wusdt.owner());
        console.log("  Vault:", address(wusdt.vault()));
        console.log("  USDT:", address(wusdt.usdt()));
        console.log("  Name:", wusdt.name());
        console.log("  Symbol:", wusdt.symbol());
        console.log("  Decimals:", wusdt.decimals());
    }
}
