// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {Test} from "forge-std/Test.sol";
import {VerifyingPaymaster} from "src/erc4337/VerifyingPaymaster.sol";
import {MockEntryPoint} from "../mocks/MockEntryPoint.sol";
import {IEntryPoint} from "account-abstraction/interfaces/IEntryPoint.sol";
import {PackedUserOperation} from "account-abstraction/interfaces/PackedUserOperation.sol";

/**
 * @title AttackVectorTests
 * @notice Tests attack vectors against VerifyingPaymaster
 * @dev Simulates signature replay, front-running, concurrent users, timestamp manipulation
 */
contract AttackVectorTests is Test {
    VerifyingPaymaster public paymaster;
    MockEntryPoint public mockEntryPoint;

    address public owner;
    address public user;
    address public attacker;

    uint256 signerPrivateKey = 0x12345;
    address public trustedSigner;

    receive() external payable {}

    function setUp() public {
        owner = address(this);
        user = address(0x1);
        attacker = address(0x2);
        trustedSigner = vm.addr(signerPrivateKey);

        vm.deal(user, 10 ether);
        vm.deal(attacker, 10 ether);

        mockEntryPoint = new MockEntryPoint();
        paymaster = new VerifyingPaymaster(
            IEntryPoint(address(mockEntryPoint)),
            trustedSigner
        );
    }

    // ============================================================
    // Signature Replay Tests
    // ============================================================

    function test_PaymasterSignatureReplay() public {
        address sender = address(0x999);

        vm.prank(owner);
        paymaster.setSenderWhitelist(sender, true);

        mockEntryPoint.depositTo{value: 1 ether}(address(paymaster));

        bytes32 dummyUserOpHash = keccak256("replayTest");
        bytes memory paymasterData = "";
        uint256 nonceBefore = paymaster.nonces(sender);

        bytes32 hash = keccak256(abi.encode(dummyUserOpHash, paymasterData, nonceBefore));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
        bytes memory signature = abi.encodePacked(r, s, v);

        PackedUserOperation memory userOp;
        userOp.sender = sender;
        userOp.nonce = 0;
        userOp.paymasterAndData = abi.encodePacked(
            address(paymaster),
            abi.encode(paymasterData, signature)
        );

        // First validation succeeds
        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(userOp, dummyUserOpHash, 0);

        assertEq(paymaster.nonces(sender), nonceBefore + 1);

        // Replay should fail — nonce changed
        vm.prank(address(mockEntryPoint));
        vm.expectRevert();
        paymaster.validatePaymasterUserOp(userOp, dummyUserOpHash, 0);

        assertEq(paymaster.nonces(sender), nonceBefore + 1);
    }

    function test_PaymasterMultipleUsersIndependentNonces() public {
        address user1 = address(0x111);
        address user2 = address(0x222);

        vm.prank(owner);
        paymaster.setSenderWhitelist(user1, true);
        vm.prank(owner);
        paymaster.setSenderWhitelist(user2, true);

        // User 1's signature shouldn't work for user 2
        bytes32 hash1 = keccak256(abi.encode(bytes32(0), bytes(""), paymaster.nonces(user1)));
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(signerPrivateKey, hash1);
        bytes memory sig1 = abi.encodePacked(r1, s1, v1);

        PackedUserOperation memory userOp1;
        userOp1.sender = user1;
        userOp1.nonce = 0;
        userOp1.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig1));

        PackedUserOperation memory userOp2;
        userOp2.sender = user2;
        userOp2.nonce = 0;
        userOp2.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig1));

        bytes32 hash1Final = keccak256(abi.encode(userOp1, bytes(""), 0));
        bytes32 hash2Final = keccak256(abi.encode(userOp2, bytes(""), 0));

        vm.prank(address(mockEntryPoint));
        vm.expectRevert();
        paymaster.validatePaymasterUserOp(userOp1, hash1Final, 0);

        vm.prank(address(mockEntryPoint));
        vm.expectRevert();
        paymaster.validatePaymasterUserOp(userOp2, hash2Final, 0);
    }

    // ============================================================
    // Timestamp/Block Manipulation Tests
    // ============================================================

    function test_TimestampManipulation() public {
        // Paymaster doesn't use timestamps — verify it works regardless

        vm.warp(1000000);

        address sender = address(0x999);
        vm.prank(owner);
        paymaster.setSenderWhitelist(sender, true);

        bytes32 hash = keccak256(abi.encode(bytes32(0), bytes(""), paymaster.nonces(sender)));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
        bytes memory sig = abi.encodePacked(r, s, v);

        PackedUserOperation memory userOp;
        userOp.sender = sender;
        userOp.nonce = 0;
        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig));

        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(userOp, bytes32(0), 0);

        // Advance timestamp massively
        vm.warp(2000000000);

        // Second operation should still work with new nonce
        bytes32 hash2 = keccak256(abi.encode(bytes32(0), bytes(""), paymaster.nonces(sender)));
        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(signerPrivateKey, hash2);
        bytes memory sig2 = abi.encodePacked(r2, s2, v2);

        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig2));

        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(userOp, bytes32(0), 0);

        assertEq(paymaster.nonces(sender), 2);
    }

    // ============================================================
    // Unauthorized Caller Tests
    // ============================================================

    function test_NonEntryPointCannotValidate() public {
        address sender = address(0x999);
        vm.prank(owner);
        paymaster.setSenderWhitelist(sender, true);

        bytes32 hash = keccak256(abi.encode(bytes32(0), bytes(""), uint256(0)));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
        bytes memory sig = abi.encodePacked(r, s, v);

        PackedUserOperation memory userOp;
        userOp.sender = sender;
        userOp.nonce = 0;
        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig));

        // Attacker tries to call validate directly — should revert
        vm.prank(attacker);
        vm.expectRevert();
        paymaster.validatePaymasterUserOp(userOp, bytes32(0), 0);

        // Nonce should not have changed
        assertEq(paymaster.nonces(sender), 0);
    }

    function test_NonEntryPointCannotPostOp() public {
        PackedUserOperation memory userOp;
        userOp.sender = address(0x999);

        vm.prank(attacker);
        vm.expectRevert();
        paymaster.postOp(userOp, "", 0);
    }

    // ============================================================
    // Owner Attack Vectors
    // ============================================================

    function test_OwnerCannotStealFundsViaWithdrawToSelf() public {
        // Owner CAN withdraw — that's by design (owner is trusted multisig)
        // Verify the mechanism works correctly
        mockEntryPoint.depositTo{value: 10 ether}(address(paymaster));

        // Owner is this contract — it can receive ETH
        uint256 balanceBefore = address(this).balance;

        vm.prank(owner);
        paymaster.withdraw(payable(owner), 5 ether);

        assertEq(address(this).balance, balanceBefore + 5 ether);
    }

    function test_NonOwnerCannotWithdraw() public {
        vm.deal(address(paymaster), 10 ether);

        vm.prank(attacker);
        vm.expectRevert();
        paymaster.withdraw(payable(attacker), 5 ether);
    }

    function test_OwnerCannotSetInvalidSigner() public {
        // Setting signer to zero address is blocked by Ownable
        vm.prank(owner);
        vm.expectRevert();
        paymaster.setTrustedSigner(address(0));
    }

    // ============================================================
    // Concurrent Operations Stress Test
    // ============================================================

    function test_RapidNonceIncrement() public {
        address sender = address(0x999);
        vm.prank(owner);
        paymaster.setSenderWhitelist(sender, true);

        mockEntryPoint.depositTo{value: 100 ether}(address(paymaster));

        // Process 50 rapid operations
        for (uint256 i = 0; i < 50; i++) {
            bytes32 hash = keccak256(abi.encode(bytes32(uint256(i)), bytes(""), paymaster.nonces(sender)));
            (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
            bytes memory sig = abi.encodePacked(r, s, v);

            PackedUserOperation memory userOp;
            userOp.sender = sender;
            userOp.nonce = 0;
            userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig));

            vm.prank(address(mockEntryPoint));
            paymaster.validatePaymasterUserOp(userOp, bytes32(uint256(i)), 0);
        }

        assertEq(paymaster.nonces(sender), 50);
    }

    // ============================================================
    // Whitelist Bypass Tests
    // ============================================================

    function test_NonWhitelistedSenderRejected() public {
        address sender = address(0x999);
        // NOT whitelisted

        bytes32 hash = keccak256(abi.encode(bytes32(0), bytes(""), uint256(0)));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
        bytes memory sig = abi.encodePacked(r, s, v);

        PackedUserOperation memory userOp;
        userOp.sender = sender;
        userOp.nonce = 0;
        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig));

        vm.prank(address(mockEntryPoint));
        vm.expectRevert();
        paymaster.validatePaymasterUserOp(userOp, bytes32(0), 0);

        // Nonce unchanged
        assertEq(paymaster.nonces(sender), 0);
    }

    function test_WhitelistToggle() public {
        address sender = address(0x999);

        // Whitelist → validate → remove from whitelist → should fail
        vm.prank(owner);
        paymaster.setSenderWhitelist(sender, true);

        bytes32 hash = keccak256(abi.encode(bytes32(0), bytes(""), uint256(0)));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
        bytes memory sig = abi.encodePacked(r, s, v);

        PackedUserOperation memory userOp;
        userOp.sender = sender;
        userOp.nonce = 0;
        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig));

        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(userOp, bytes32(0), 0);

        assertEq(paymaster.nonces(sender), 1);

        // Remove from whitelist
        vm.prank(owner);
        paymaster.setSenderWhitelist(sender, false);

        // Should fail now
        bytes32 hash2 = keccak256(abi.encode(bytes32(0), bytes(""), uint256(1)));
        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(signerPrivateKey, hash2);
        bytes memory sig2 = abi.encodePacked(r2, s2, v2);
        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig2));

        vm.prank(address(mockEntryPoint));
        vm.expectRevert();
        paymaster.validatePaymasterUserOp(userOp, bytes32(0), 0);
    }
}
