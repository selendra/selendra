// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {Script, console} from "forge-std/Script.sol";
import {VerifyingPaymaster} from "src/erc4337/VerifyingPaymaster.sol";

contract Verify is Script {
    function run() external {
        // Set these to deployed addresses before running
        address payable paymasterAddr = payable(vm.envOr("PAYMASTER_ADDRESS", address(0)));
        
        if (paymasterAddr == address(0)) {
            console.log("Set PAYMASTER_ADDRESS env var");
            return;
        }

        VerifyingPaymaster paymaster = VerifyingPaymaster(paymasterAddr);

        console.log("\nVerifyingPaymaster:");
        console.log("  Owner:", paymaster.owner());
        console.log("  TrustedSigner:", paymaster.trustedSigner());
        console.log("  EntryPoint:", address(paymaster.entryPoint()));
    }
}
