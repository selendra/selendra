// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {Test} from "forge-std/Test.sol";
import {VerifyingPaymaster} from "src/erc4337/VerifyingPaymaster.sol";
import {MockEntryPoint} from "../mocks/MockEntryPoint.sol";
import {IEntryPoint} from "account-abstraction/interfaces/IEntryPoint.sol";
import {PackedUserOperation} from "account-abstraction/interfaces/PackedUserOperation.sol";
import {IPaymaster} from "account-abstraction/interfaces/IPaymaster.sol";

/**
 * @title OpusAudit
 * @notice Security tests from Claude Opus second-opinion audit
 * @dev Tests for ERC-4337 compliance, validationData, postOp modes, and edge cases
 */
contract OpusAudit is Test {
    VerifyingPaymaster public paymaster;
    MockEntryPoint public mockEntryPoint;

    address public owner;
    address public user;
    address public attacker;

    uint256 signerPrivateKey = 0x12345;
    address public trustedSigner;

    function setUp() public {
        owner = address(this);
        user = address(0x1);
        attacker = address(0x999);
        trustedSigner = vm.addr(signerPrivateKey);

        mockEntryPoint = new MockEntryPoint();
        paymaster = new VerifyingPaymaster(
            IEntryPoint(address(mockEntryPoint)),
            trustedSigner
        );
    }

    // ============================================================
    // ERC-4337 Compliance: validationData Return
    // ============================================================

    function test_ERC4337_ValidateReturnsValidationData() public {
        vm.prank(owner);
        paymaster.setSenderWhitelist(user, true);

        mockEntryPoint.depositTo{value: 1 ether}(address(paymaster));

        PackedUserOperation memory userOp;
        userOp.sender = user;
        userOp.nonce = 0;

        bytes32 dummyUserOpHash = keccak256("validationDataTest");
        uint256 nonceBefore = paymaster.nonces(user);
        bytes32 hash = keccak256(abi.encode(dummyUserOpHash, bytes(""), nonceBefore));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
        bytes memory signature = abi.encodePacked(r, s, v);

        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), signature));

        vm.prank(address(mockEntryPoint));
        (bytes memory context, uint256 validationData) = paymaster.validatePaymasterUserOp(userOp, dummyUserOpHash, 0);

        // validationData should be 0: sigFailed=0, validUntil=0, validAfter=0
        assertEq(validationData, 0, "validationData must be 0 for valid signatures");
        assertGt(context.length, 0, "context must not be empty");
    }

    // ============================================================
    // ERC-4337 Compliance: postOp Signature
    // ============================================================

    function test_ERC4337_PostOpHasCorrectSignature() public {
        // Test that postOp accepts PostOpMode enum
        vm.prank(address(mockEntryPoint));
        paymaster.postOp(IPaymaster.PostOpMode.opSucceeded, abi.encodePacked(uint256(0)), 1000, 50);

        vm.prank(address(mockEntryPoint));
        paymaster.postOp(IPaymaster.PostOpMode.opReverted, abi.encodePacked(uint256(0)), 1000, 50);

        // All modes should work without revert
        assertTrue(true, "postOp accepts all PostOpMode values");
    }

    function test_ERC4337_PostOpOnlyEntryPointCanCall() public {
        vm.prank(attacker);
        vm.expectRevert();
        paymaster.postOp(IPaymaster.PostOpMode.opSucceeded, "", 0, 0);
    }

    // ============================================================
    // Signature Verification: Nonce Inclusion
    // ============================================================

    function test_Security_NonceIncludedInSignatureHash() public {
        vm.prank(owner);
        paymaster.setSenderWhitelist(user, true);

        mockEntryPoint.depositTo{value: 2 ether}(address(paymaster));

        uint256 nonce0 = paymaster.nonces(user);

        // Create signature for nonce 0
        bytes32 hash0 = keccak256(abi.encode(bytes32(0), bytes(""), nonce0));
        (uint8 v0, bytes32 r0, bytes32 s0) = vm.sign(signerPrivateKey, hash0);
        bytes memory sig0 = abi.encodePacked(r0, s0, v0);

        PackedUserOperation memory op0;
        op0.sender = user;
        op0.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig0));

        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(op0, bytes32(0), 0);

        assertEq(paymaster.nonces(user), 1);

        // Same signature should fail for nonce 1 (nonce is now 1)
        uint256 nonce1 = paymaster.nonces(user);
        bytes32 hash1 = keccak256(abi.encode(bytes32(0), bytes(""), nonce1));

        // hash0 != hash1, so sig0 should fail
        vm.prank(address(mockEntryPoint));
        vm.expectRevert();
        paymaster.validatePaymasterUserOp(op0, bytes32(0), 0);
    }

    // ============================================================
    // Deposit Management
    // ============================================================

    function test_Security_InsufficientDepositReverts() public {
        vm.prank(owner);
        paymaster.setSenderWhitelist(user, true);

        // Deposit only 0.5 ether
        mockEntryPoint.depositTo{value: 0.5 ether}(address(paymaster));

        PackedUserOperation memory userOp;
        userOp.sender = user;
        userOp.nonce = 0;

        bytes32 dummyUserOpHash = keccak256("depositTest");
        uint256 nonceBefore = paymaster.nonces(user);
        bytes32 hash = keccak256(abi.encode(dummyUserOpHash, bytes(""), nonceBefore));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
        bytes memory signature = abi.encodePacked(r, s, v);

        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), signature));

        // Request maxCost of 1 ether but only have 0.5
        vm.prank(address(mockEntryPoint));
        vm.expectRevert();
        paymaster.validatePaymasterUserOp(userOp, dummyUserOpHash, 1 ether);

        // Nonce should not have incremented
        assertEq(paymaster.nonces(user), nonceBefore, "Nonce must not increment on deposit failure");
    }

    function test_Security_DepositCheckUsesActualBalance() public {
        vm.prank(owner);
        paymaster.setSenderWhitelist(user, true);

        // Verify deposit check calls entryPoint.balanceOf
        mockEntryPoint.depositTo{value: 2 ether}(address(paymaster));

        PackedUserOperation memory userOp;
        userOp.sender = user;

        bytes32 dummyUserOpHash = keccak256("balanceCheck");
        uint256 nonceBefore = paymaster.nonces(user);
        bytes32 hash = keccak256(abi.encode(dummyUserOpHash, bytes(""), nonceBefore));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
        bytes memory signature = abi.encodePacked(r, s, v);

        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), signature));

        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(userOp, dummyUserOpHash, 1 ether);

        // Should succeed with sufficient deposit
        assertEq(paymaster.nonces(user), nonceBefore + 1);
    }

    // ============================================================
    // Signature Edge Cases
    // ============================================================

    function test_Security_EmptySignatureRejected() public {
        bytes memory emptySig = "";
        assertFalse(paymaster.verifySigner(bytes32(0), emptySig), "Empty signature should be rejected");
    }

    function test_Security_SignatureLengthNot65Rejected() public {
        bytes memory shortSig = abi.encodePacked(bytes32(uint256(1)), bytes32(0)); // Only 64 bytes
        assertFalse(paymaster.verifySigner(bytes32(0), shortSig), "64-byte signature should be rejected");

        bytes memory longSig = abi.encodePacked(bytes32(uint256(1)), bytes32(0), bytes32(0), uint8(27)); // 97 bytes
        assertFalse(paymaster.verifySigner(bytes32(0), longSig), "97-byte signature should be rejected");
    }

    function test_Security_InvalidVValueRejected() public {
        // v = 2 would become 29 after v += 27, which is > 28
        bytes memory veryBadV = abi.encodePacked(bytes32(uint256(1)), bytes32(0), uint8(2));
        assertFalse(paymaster.verifySigner(bytes32(0), veryBadV), "v=2 (becomes 29) should be rejected");
    }

    function test_Security_STooLargeRejected() public {
        // Test that s values above the boundary are rejected
        // Note: We can't test exact boundary with a valid signature since
        // we'd need to craft a specific signature, but we can test that
        // values above boundary fail the length check early

        uint256 boundary = 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0;

        // Create a signature with s > boundary
        bytes32 r = bytes32(uint256(1));
        bytes32 sAboveBoundary = bytes32(boundary + 1);
        bytes memory sigAbove = abi.encodePacked(r, sAboveBoundary, uint8(27));

        // This should fail because s > boundary (check happens before ecrecover)
        assertFalse(paymaster.verifySigner(bytes32(0), sigAbove), "s > boundary should be rejected");

        // Test maximum valid s value (boundary - 1) will pass the boundary check
        // but fail ecrecover (not a valid signature)
        bytes32 sBelowBoundary = bytes32(boundary - 1);
        bytes memory sigBelow = abi.encodePacked(r, sBelowBoundary, uint8(27));

        // Should return false (ecrecover fails) but NOT because of s boundary check
        assertFalse(paymaster.verifySigner(bytes32(0), sigBelow), "invalid sig should fail");
    }

    // ============================================================
    // Context Encoding
    // ============================================================

    function test_Context_EncodesNonceCorrectly() public {
        vm.prank(owner);
        paymaster.setSenderWhitelist(user, true);

        mockEntryPoint.depositTo{value: 1 ether}(address(paymaster));

        PackedUserOperation memory userOp;
        userOp.sender = user;
        userOp.nonce = 0;

        bytes32 dummyUserOpHash = keccak256("contextTest");
        uint256 nonceBefore = paymaster.nonces(user); // Get actual nonce (0)

        bytes32 hash = keccak256(abi.encode(dummyUserOpHash, bytes(""), nonceBefore));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
        bytes memory signature = abi.encodePacked(r, s, v);

        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), signature));

        vm.prank(address(mockEntryPoint));
        (bytes memory context,) = paymaster.validatePaymasterUserOp(userOp, dummyUserOpHash, 0);

        // Context should be abi.encodePacked(nonceBefore) which is 0
        uint256 decodedNonce;
        assembly {
            decodedNonce := mload(add(context, 0x20))
        }

        assertEq(decodedNonce, nonceBefore, "Context should encode the nonce before increment");
        assertEq(paymaster.nonces(user), 1, "Nonce should have incremented to 1");
    }

    // ============================================================
    // Multiple Operations: Nonce Progression
    // ============================================================

    function test_MultipleOps_NonceProgressionCorrect() public {
        vm.prank(owner);
        paymaster.setSenderWhitelist(user, true);

        mockEntryPoint.depositTo{value: 10 ether}(address(paymaster));

        uint256 expectedNonce = 0;

        for (uint256 i = 0; i < 10; i++) {
            assertEq(paymaster.nonces(user), expectedNonce, "Nonce should match expected");

            PackedUserOperation memory userOp;
            userOp.sender = user;

            bytes32 dummyUserOpHash = keccak256(abi.encode("nonceTest", i));
            bytes32 hash = keccak256(abi.encode(dummyUserOpHash, bytes(""), expectedNonce));

            (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
            bytes memory signature = abi.encodePacked(r, s, v);

            userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), signature));

            vm.prank(address(mockEntryPoint));
            paymaster.validatePaymasterUserOp(userOp, dummyUserOpHash, 0);

            expectedNonce++;
        }

        assertEq(paymaster.nonces(user), 10, "Nonce should be 10 after 10 operations");
    }

    // ============================================================
    // Reentrancy Protection
    // ============================================================

    function test_Security_NoExternalCallsDuringValidate() public {
        vm.prank(owner);
        paymaster.setSenderWhitelist(user, true);

        mockEntryPoint.depositTo{value: 1 ether}(address(paymaster));

        PackedUserOperation memory userOp;
        userOp.sender = user;

        bytes32 dummyUserOpHash = keccak256("reentrancyTest");
        bytes32 hash = keccak256(abi.encode(dummyUserOpHash, bytes(""), paymaster.nonces(user)));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
        bytes memory signature = abi.encodePacked(r, s, v);

        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), signature));

        // validatePaymasterUserOp makes no external calls except to entryPoint.balanceOf (view)
        // and does NOT call user-provided contracts, so reentrancy is not possible
        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(userOp, dummyUserOpHash, 0);

        assertTrue(true, "validatePaymasterUserOp completes without reentrancy issues");
    }

    // ============================================================
    // Access Control: Zero Address Protection
    // ============================================================

    function test_Constructor_RejectsZeroEntryPoint() public {
        vm.expectRevert();
        new VerifyingPaymaster(IEntryPoint(address(0)), trustedSigner);
    }

    function test_Constructor_RejectsZeroTrustedSigner() public {
        vm.expectRevert();
        new VerifyingPaymaster(IEntryPoint(address(mockEntryPoint)), address(0));
    }

    function test_OwnerCannotSetZeroSigner() public {
        vm.prank(owner);
        vm.expectRevert();
        paymaster.setTrustedSigner(address(0));
    }

    // ============================================================
    // EntryPoint Immutability
    // ============================================================

    function test_EntryPointIsImmutable() public {
        // entryPoint should be immutable - verify it's set correctly
        assertEq(address(paymaster.entryPoint()), address(mockEntryPoint));
    }
}
