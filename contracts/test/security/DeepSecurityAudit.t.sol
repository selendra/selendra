// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {Test} from "forge-std/Test.sol";
import {VerifyingPaymaster} from "src/erc4337/VerifyingPaymaster.sol";
import {MockEntryPoint} from "../mocks/MockEntryPoint.sol";
import {IEntryPoint} from "account-abstraction/interfaces/IEntryPoint.sol";
import {PackedUserOperation} from "account-abstraction/interfaces/PackedUserOperation.sol";

/**
 * @title DeepSecurityAudit
 * @notice Security tests for VerifyingPaymaster
 * @dev Covers signature replay, access control, nonce isolation, edge cases
 */
contract DeepSecurityAudit is Test {
    VerifyingPaymaster public paymaster;
    MockEntryPoint public mockEntryPoint;

    address public owner;
    address public user;
    address public alice;
    address public attacker;

    uint256 signerPrivateKey = 0x12345;
    address public trustedSigner;

    receive() external payable {}

    function setUp() public {
        owner = address(this);
        user = address(0x1);
        alice = address(0x2);
        attacker = address(0x999);
        trustedSigner = vm.addr(signerPrivateKey);

        vm.deal(user, 10 ether);
        vm.deal(alice, 10 ether);
        vm.deal(attacker, 10 ether);

        mockEntryPoint = new MockEntryPoint();
        paymaster = new VerifyingPaymaster(
            IEntryPoint(address(mockEntryPoint)),
            trustedSigner
        );
    }

    // ============================================================
    // CRITICAL: Signature Replay Prevention
    // ============================================================

    function test_Security_NoncesPreventReplay() public {
        address testUser = address(0x1234);

        vm.prank(owner);
        paymaster.setSenderWhitelist(testUser, true);

        mockEntryPoint.depositTo{value: 1 ether}(address(paymaster));

        assertEq(paymaster.nonces(testUser), 0);

        PackedUserOperation memory userOp;
        userOp.sender = testUser;
        userOp.nonce = 0;

        bytes32 dummyUserOpHash = keccak256("testUserOp");
        bytes32 hash = keccak256(abi.encode(dummyUserOpHash, bytes(""), paymaster.nonces(testUser)));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
        bytes memory signature = abi.encodePacked(r, s, v);

        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), signature));

        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(userOp, dummyUserOpHash, 0);

        assertEq(paymaster.nonces(testUser), 1);

        // Same signature fails — nonce changed
        vm.prank(address(mockEntryPoint));
        vm.expectRevert();
        paymaster.validatePaymasterUserOp(userOp, dummyUserOpHash, 0);
    }

    // ============================================================
    // Signature Malleability
    // ============================================================

    function test_Security_SignatureMalleabilityCheck() public {
        bytes32 hash = keccak256("test");

        // s value in upper half should be rejected by ECDSA
        uint8 v = 27;
        bytes32 r = bytes32(uint256(1));
        bytes32 s = bytes32(uint256(0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0) + 1);

        bytes memory badSignature = abi.encodePacked(r, s, v);

        assertFalse(paymaster.verifySigner(hash, badSignature),
            "Signature with s in upper half should be rejected");
    }

    // ============================================================
    // Access Control
    // ============================================================

    function test_Security_AllOwnerFunctionsProtected() public {
        vm.expectRevert();
        vm.prank(attacker);
        paymaster.setTrustedSigner(attacker);

        vm.expectRevert();
        vm.prank(attacker);
        paymaster.setSenderWhitelist(attacker, true);

        vm.expectRevert();
        vm.prank(attacker);
        paymaster.withdraw(payable(attacker), 1 ether);
    }

    function test_Security_OnlyEntryPointCanValidate() public {
        address sender = address(0x1234);
        vm.prank(owner);
        paymaster.setSenderWhitelist(sender, true);

        bytes32 hash = keccak256(abi.encode(bytes32(0), bytes(""), uint256(0)));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
        bytes memory sig = abi.encodePacked(r, s, v);

        PackedUserOperation memory userOp;
        userOp.sender = sender;
        userOp.nonce = 0;
        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig));

        // Attacker calls directly — should revert
        vm.prank(attacker);
        vm.expectRevert();
        paymaster.validatePaymasterUserOp(userOp, bytes32(0), 0);

        // Nonce unchanged
        assertEq(paymaster.nonces(sender), 0);
    }

    // ============================================================
    // Context Return
    // ============================================================

    function test_Security_ValidateReturnsContext() public {
        address whitelistedSender = address(0xABC);

        vm.prank(owner);
        paymaster.setSenderWhitelist(whitelistedSender, true);

        mockEntryPoint.depositTo{value: 1 ether}(address(paymaster));

        PackedUserOperation memory userOp;
        userOp.sender = whitelistedSender;
        userOp.nonce = 0;

        bytes32 dummyUserOpHash = keccak256("whitelistTest");
        uint256 nonceBefore = paymaster.nonces(whitelistedSender);
        bytes32 hash = keccak256(abi.encode(dummyUserOpHash, bytes(""), nonceBefore));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
        bytes memory signature = abi.encodePacked(r, s, v);

        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), signature));

        vm.prank(address(mockEntryPoint));
        (bytes memory context) = paymaster.validatePaymasterUserOp(userOp, dummyUserOpHash, 0);

        assertGt(context.length, 0, "Context should not be empty");
    }

    // ============================================================
    // Multi-User Isolation
    // ============================================================

    function test_Security_MultiUserNonceIsolation() public {
        address user1 = address(0x111);
        address user2 = address(0x222);

        vm.prank(owner);
        paymaster.setSenderWhitelist(user1, true);
        vm.prank(owner);
        paymaster.setSenderWhitelist(user2, true);

        mockEntryPoint.depositTo{value: 5 ether}(address(paymaster));

        // User 1 does 5 operations
        for (uint256 i = 0; i < 5; i++) {
            bytes32 hash = keccak256(abi.encode(bytes32(uint256(i + 100)), bytes(""), paymaster.nonces(user1)));
            (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
            bytes memory sig = abi.encodePacked(r, s, v);

            PackedUserOperation memory op;
            op.sender = user1;
            op.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig));

            vm.prank(address(mockEntryPoint));
            paymaster.validatePaymasterUserOp(op, bytes32(uint256(i + 100)), 0);
        }

        // User 2 does 3 operations
        for (uint256 i = 0; i < 3; i++) {
            bytes32 hash = keccak256(abi.encode(bytes32(uint256(i + 200)), bytes(""), paymaster.nonces(user2)));
            (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
            bytes memory sig = abi.encodePacked(r, s, v);

            PackedUserOperation memory op;
            op.sender = user2;
            op.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig));

            vm.prank(address(mockEntryPoint));
            paymaster.validatePaymasterUserOp(op, bytes32(uint256(i + 200)), 0);
        }

        // Verify independent nonces
        assertEq(paymaster.nonces(user1), 5);
        assertEq(paymaster.nonces(user2), 3);
    }

    // ============================================================
    // Whitelist Bypass
    // ============================================================

    function test_Security_NonWhitelistedSenderRejected() public {
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

        assertEq(paymaster.nonces(sender), 0);
    }

    function test_Security_WhitelistToggleBlocksAfterRemoval() public {
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

        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(userOp, bytes32(0), 0);

        assertEq(paymaster.nonces(sender), 1);

        // Remove from whitelist
        vm.prank(owner);
        paymaster.setSenderWhitelist(sender, false);

        bytes32 hash2 = keccak256(abi.encode(bytes32(0), bytes(""), uint256(1)));
        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(signerPrivateKey, hash2);
        bytes memory sig2 = abi.encodePacked(r2, s2, v2);
        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig2));

        vm.prank(address(mockEntryPoint));
        vm.expectRevert();
        paymaster.validatePaymasterUserOp(userOp, bytes32(0), 0);
    }

    // ============================================================
    // Owner Operations
    // ============================================================

    function test_Security_OwnerWithdrawWorks() public {
        mockEntryPoint.depositTo{value: 10 ether}(address(paymaster));

        uint256 balanceBefore = owner.balance;

        vm.prank(owner);
        paymaster.withdraw(payable(owner), 5 ether);

        assertEq(owner.balance, balanceBefore + 5 ether);
    }

    function test_Security_NonOwnerCannotWithdraw() public {
        vm.deal(address(paymaster), 10 ether);

        vm.prank(attacker);
        vm.expectRevert();
        paymaster.withdraw(payable(attacker), 5 ether);
    }

    // ============================================================
    // Stress Test
    // ============================================================

    function test_Security_RapidOperations() public {
        address sender = address(0x999);
        vm.prank(owner);
        paymaster.setSenderWhitelist(sender, true);

        mockEntryPoint.depositTo{value: 100 ether}(address(paymaster));

        for (uint256 i = 0; i < 100; i++) {
            bytes32 hash = keccak256(abi.encode(bytes32(i), bytes(""), paymaster.nonces(sender)));
            (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
            bytes memory sig = abi.encodePacked(r, s, v);

            PackedUserOperation memory op;
            op.sender = sender;
            op.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig));

            vm.prank(address(mockEntryPoint));
            paymaster.validatePaymasterUserOp(op, bytes32(i), 0);
        }

        assertEq(paymaster.nonces(sender), 100);
    }
}
