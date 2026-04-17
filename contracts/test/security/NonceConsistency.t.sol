// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {Test} from "forge-std/Test.sol";
import {VerifyingPaymaster} from "src/erc4337/VerifyingPaymaster.sol";
import {MockEntryPoint} from "../mocks/MockEntryPoint.sol";
import {IEntryPoint} from "account-abstraction/interfaces/IEntryPoint.sol";
import {PackedUserOperation} from "account-abstraction/interfaces/PackedUserOperation.sol";

/**
 * @title NonceConsistency
 * @notice Tests that nonce is not incremented on failed validation (deposit check, etc.)
 */
contract NonceConsistency is Test {
    VerifyingPaymaster public paymaster;
    MockEntryPoint public mockEntryPoint;

    address public owner;
    address public sender;
    uint256 signerPrivateKey = 0x12345;
    address public trustedSigner;

    function setUp() public {
        owner = address(this);
        sender = address(0xABC);
        trustedSigner = vm.addr(signerPrivateKey);

        mockEntryPoint = new MockEntryPoint();
        paymaster = new VerifyingPaymaster(
            IEntryPoint(address(mockEntryPoint)),
            trustedSigner
        );

        vm.prank(owner);
        paymaster.setSenderWhitelist(sender, true);
    }

    /// @notice Nonce should NOT increment when deposit is insufficient
    function test_NonceNotIncrementedOnInsufficientDeposit() public {
        // No deposit made — balance is 0

        bytes32 dummyUserOpHash = keccak256("depositFailTest");
        uint256 nonceBefore = paymaster.nonces(sender);
        bytes32 hash = keccak256(abi.encode(dummyUserOpHash, bytes(""), nonceBefore));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
        bytes memory signature = abi.encodePacked(r, s, v);

        PackedUserOperation memory userOp;
        userOp.sender = sender;
        userOp.nonce = 0;
        userOp.paymasterAndData = abi.encodePacked(
            address(paymaster),
            abi.encode(bytes(""), signature)
        );

        // Should revert (insufficient deposit)
        vm.prank(address(mockEntryPoint));
        vm.expectRevert();
        paymaster.validatePaymasterUserOp(userOp, dummyUserOpHash, 1 ether);

        // Nonce MUST remain unchanged
        assertEq(paymaster.nonces(sender), nonceBefore, "Nonce incremented on deposit failure");

        // Same signature should still work after funding
        mockEntryPoint.depositTo{value: 2 ether}(address(paymaster));

        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(userOp, dummyUserOpHash, 1 ether);

        assertEq(paymaster.nonces(sender), nonceBefore + 1);
    }
}
