// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {Test, console} from "forge-std/Test.sol";
import {VerifyingPaymaster} from "src/erc4337/VerifyingPaymaster.sol";
import {MockEntryPoint} from "../mocks/MockEntryPoint.sol";
import {IEntryPoint} from "account-abstraction/interfaces/IEntryPoint.sol";
import {PackedUserOperation} from "account-abstraction/interfaces/PackedUserOperation.sol";

contract VerifyingPaymasterTest is Test {
    VerifyingPaymaster public paymaster;
    MockEntryPoint public mockEntryPoint;

    address public owner;
    address public trustedSigner;
    address public user;
    address public whitelistedSender;

    uint256 signerPrivateKey;

    error Ownable__NotOwner();

    receive() external payable {}

    function setUp() public {
        owner = address(this);
        signerPrivateKey = 0x12345;
        trustedSigner = vm.addr(signerPrivateKey);
        user = address(0x1);
        whitelistedSender = address(0x2);

        mockEntryPoint = new MockEntryPoint();
        paymaster = new VerifyingPaymaster(
            IEntryPoint(address(mockEntryPoint)),
            trustedSigner
        );
    }

    function test_Deposit() public {
        uint256 amount = 1 ether;

        vm.deal(user, amount);
        vm.prank(user);
        paymaster.deposit{value: amount}();

        assertEq(paymaster.getDeposit(), amount);
    }

    function test_Withdraw() public {
        uint256 amount = 1 ether;

        vm.deal(user, amount);
        vm.prank(user);
        paymaster.deposit{value: amount}();

        uint256 ownerBalanceBefore = owner.balance;
        vm.prank(owner);
        paymaster.withdraw(payable(owner), amount);

        assertEq(paymaster.getDeposit(), 0);
        assertEq(owner.balance, ownerBalanceBefore + amount);
    }

    function test_SetTrustedSigner() public {
        address newSigner = address(0x999);

        vm.prank(owner);
        paymaster.setTrustedSigner(newSigner);

        assertEq(paymaster.trustedSigner(), newSigner);
    }

    function test_SetSenderWhitelist() public {
        vm.prank(owner);
        paymaster.setSenderWhitelist(whitelistedSender, true);

        assertTrue(paymaster.whitelistedSenders(whitelistedSender));
    }

    function test_RemoveFromWhitelist() public {
        vm.prank(owner);
        paymaster.setSenderWhitelist(whitelistedSender, true);

        vm.prank(owner);
        paymaster.setSenderWhitelist(whitelistedSender, false);

        assertFalse(paymaster.whitelistedSenders(whitelistedSender));
    }

    function test_VerifySigner() public {
        bytes32 hash = keccak256("test message");
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
        bytes memory signature = abi.encodePacked(r, s, v);

        assertTrue(paymaster.verifySigner(hash, signature));
    }

    function test_VerifySigner_Fails() public {
        bytes32 hash = keccak256("test message");
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(0x99999, hash);
        bytes memory signature = abi.encodePacked(r, s, v);

        assertFalse(paymaster.verifySigner(hash, signature));
    }

    function test_OnlyOwnerCanSetSigner() public {
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        paymaster.setTrustedSigner(address(0x999));
    }

    function test_OnlyOwnerCanWhitelist() public {
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        paymaster.setSenderWhitelist(user, true);
    }

    function test_DepositToEntryPoint() public {
        uint256 amount = 1 ether;

        vm.deal(user, amount);
        vm.prank(user);
        paymaster.deposit{value: amount}();

        assertEq(mockEntryPoint.balanceOf(address(paymaster)), amount);
    }

    function testFuzz_VerifySignature(bytes32 message) public {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, message);
        bytes memory signature = abi.encodePacked(r, s, v);

        assertTrue(paymaster.verifySigner(message, signature));
    }
}
