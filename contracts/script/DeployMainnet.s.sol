// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {Script, console} from "forge-std/Script.sol";
import {IEntryPoint} from "account-abstraction/interfaces/IEntryPoint.sol";
import {VerifyingPaymaster} from "src/erc4337/VerifyingPaymaster.sol";

contract DeployMainnet is Script {
    // EntryPoint v0.7 address (standard, do not redeploy)
    address constant ENTRYPOINT = 0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789;
    
    // Configuration - UPDATE THESE BEFORE DEPLOYMENT
    address constant TRUSTED_SIGNER = 0x0000000000000000000000000000000000000001; // CHANGE THIS

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        // Deploy VerifyingPaymaster
        console.log("Deploying VerifyingPaymaster...");
        VerifyingPaymaster paymaster = new VerifyingPaymaster(
            IEntryPoint(ENTRYPOINT),
            TRUSTED_SIGNER
        );
        console.log("VerifyingPaymaster:", address(paymaster));

        vm.stopBroadcast();

        console.log("\n=== Deployment Summary ===");
        console.log("VerifyingPaymaster:", address(paymaster));
        console.log("TrustedSigner:", TRUSTED_SIGNER);
        console.log("Owner:", msg.sender);
    }
}
