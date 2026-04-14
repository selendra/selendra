// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {IEntryPoint, PackedUserOperation} from "account-abstraction/interfaces/IEntryPoint.sol";

// Minimal mock for testing - does NOT implement full IEntryPoint
contract MockEntryPoint {
    mapping(address => uint256) public balances;

    function depositTo(address account) external payable {
        balances[account] += msg.value;
    }

    function withdrawTo(address payable withdrawAddress, uint256 withdrawAmount) external {
        if (balances[msg.sender] < withdrawAmount) {
            revert("insufficient balance");
        }
        balances[msg.sender] -= withdrawAmount;
        withdrawAddress.transfer(withdrawAmount);
    }

    function balanceOf(address account) external view returns (uint256) {
        return balances[account];
    }

    uint256 public balance;

    function handleOps(
        PackedUserOperation[] calldata ops,
        address payable beneficiary
    ) external {}

    function handleAggregatedOps(
        PackedUserOperation[] calldata ops,
        address payable beneficiary
    ) external {}

    function getUserOpHash(
        PackedUserOperation calldata userOp
    ) external view returns (bytes32) {
        return keccak256(abi.encode(userOp));
    }

    receive() external payable {}
}
