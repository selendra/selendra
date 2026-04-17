// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {Test} from "forge-std/Test.sol";
import {VerifyingPaymaster} from "src/erc4337/VerifyingPaymaster.sol";
import {MockEntryPoint} from "../mocks/MockEntryPoint.sol";
import {IEntryPoint} from "account-abstraction/interfaces/IEntryPoint.sol";
import {PackedUserOperation} from "account-abstraction/interfaces/PackedUserOperation.sol";
import {IPaymaster} from "account-abstraction/interfaces/IPaymaster.sol";

/**
 * @title AccessControlTests
 * @notice Tests access control for VerifyingPaymaster
 * @dev Ensures only authorized addresses can call sensitive functions
 */
contract AccessControlTests is Test {
    VerifyingPaymaster public paymaster;
    MockEntryPoint public mockEntryPoint;

    address public owner;
    address public user;
    address public alice;

    uint256 signerPrivateKey = 0x12345;
    address public trustedSigner;

    function setUp() public {
        owner = address(this);
        user = address(0x1);
        alice = address(0x2);
        trustedSigner = vm.addr(signerPrivateKey);

        mockEntryPoint = new MockEntryPoint();
        paymaster = new VerifyingPaymaster(
            IEntryPoint(address(mockEntryPoint)),
            trustedSigner
        );
    }

    // ============================================================
    // VerifyingPaymaster OnlyOwner Functions
    // ============================================================

    function test_Ownable_VerifyingPaymaster_SetTrustedSigner() public {
        vm.expectRevert();
        vm.prank(user);
        paymaster.setTrustedSigner(alice);
    }

    function test_Ownable_VerifyingPaymaster_SetSenderWhitelist() public {
        vm.expectRevert();
        vm.prank(user);
        paymaster.setSenderWhitelist(alice, true);
    }

    function test_Ownable_VerifyingPaymaster_Withdraw() public {
        vm.deal(address(paymaster), 1 ether);

        vm.expectRevert();
        vm.prank(user);
        paymaster.withdraw(payable(alice), 0.5 ether);
    }

    // ============================================================
    // VerifyingPaymaster EntryPoint-only Functions
    // ============================================================

    function test_Paymaster_OnlyEntryPoint_ValidatePaymasterUserOp() public {
        PackedUserOperation memory userOp;
        userOp.sender = alice;
        userOp.nonce = 0;
        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), bytes("")));

        bytes32 userOpHash = keccak256(abi.encode(userOp));

        vm.expectRevert();
        vm.prank(user);
        paymaster.validatePaymasterUserOp(userOp, userOpHash, 0);
    }

    function test_Paymaster_OnlyEntryPoint_PostOp() public {
        vm.expectRevert();
        vm.prank(user);
        paymaster.postOp(IPaymaster.PostOpMode.opSucceeded, "", 0, 0);
    }

    function test_Paymaster_EntryPointCanCallValidate() public {
        vm.prank(owner);
        paymaster.setSenderWhitelist(alice, true);

        bytes32 dummyHash = keccak256(abi.encode("test"));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, dummyHash);
        bytes memory signature = abi.encodePacked(r, s, v);

        PackedUserOperation memory userOp;
        userOp.sender = alice;
        userOp.nonce = 0;
        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), signature));

        bytes32 userOpHash = keccak256(abi.encode(userOp));

        vm.prank(address(mockEntryPoint));
        vm.expectRevert();
        paymaster.validatePaymasterUserOp(userOp, userOpHash, 0);
    }

    // ============================================================
    // Paymaster Per-User Nonces
    // ============================================================

    function test_Paymaster_PerUserNonces() public {
        address user1 = address(0x100);
        address user2 = address(0x200);

        assertEq(paymaster.nonces(user1), 0);
        assertEq(paymaster.nonces(user2), 0);

        vm.prank(owner);
        paymaster.setSenderWhitelist(user1, true);
        vm.prank(owner);
        paymaster.setSenderWhitelist(user2, true);

        vm.deal(address(this), 2 ether);
        IEntryPoint(address(mockEntryPoint)).depositTo{value: 2 ether}(address(paymaster));

        // Increment user1 nonce
        PackedUserOperation memory userOp1;
        userOp1.sender = user1;
        userOp1.nonce = 0;

        bytes32 dummyUserOpHash1 = keccak256("user1");
        uint256 nonceBefore1 = paymaster.nonces(user1);
        bytes32 hash1 = keccak256(abi.encode(dummyUserOpHash1, bytes(""), nonceBefore1));

        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(signerPrivateKey, hash1);
        bytes memory signature1 = abi.encodePacked(r1, s1, v1);
        userOp1.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), signature1));

        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(userOp1, dummyUserOpHash1, 0);

        assertEq(paymaster.nonces(user1), 1);
        assertEq(paymaster.nonces(user2), 0);

        // Increment user2 nonce
        PackedUserOperation memory userOp2;
        userOp2.sender = user2;
        userOp2.nonce = 0;

        bytes32 dummyUserOpHash2 = keccak256("user2");
        uint256 nonceBefore2 = paymaster.nonces(user2);
        bytes32 hash2 = keccak256(abi.encode(dummyUserOpHash2, bytes(""), nonceBefore2));

        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(signerPrivateKey, hash2);
        bytes memory signature2 = abi.encodePacked(r2, s2, v2);
        userOp2.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), signature2));

        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(userOp2, dummyUserOpHash2, 0);

        assertEq(paymaster.nonces(user1), 1);
        assertEq(paymaster.nonces(user2), 1);
    }

    function test_Paymaster_ConcurrentUsers() public {
        address user1 = address(0x100);
        address user2 = address(0x200);

        vm.prank(owner);
        paymaster.setSenderWhitelist(user1, true);
        vm.prank(owner);
        paymaster.setSenderWhitelist(user2, true);

        vm.deal(address(this), 2 ether);
        IEntryPoint(address(mockEntryPoint)).depositTo{value: 2 ether}(address(paymaster));

        assertEq(paymaster.nonces(user1), 0);
        assertEq(paymaster.nonces(user2), 0);

        // Increment user1
        bytes32 u1Hash = keccak256(abi.encode(keccak256("u1"), bytes(""), uint256(0)));
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(signerPrivateKey, u1Hash);
        bytes memory sig1 = abi.encodePacked(r1, s1, v1);
        bytes memory pad1 = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig1));
        PackedUserOperation memory op1;
        op1.sender = user1;
        op1.paymasterAndData = pad1;
        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(op1, keccak256("u1"), 0);

        // Increment user2
        bytes32 u2Hash = keccak256(abi.encode(keccak256("u2"), bytes(""), uint256(0)));
        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(signerPrivateKey, u2Hash);
        bytes memory sig2 = abi.encodePacked(r2, s2, v2);
        bytes memory pad2 = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig2));
        PackedUserOperation memory op2;
        op2.sender = user2;
        op2.paymasterAndData = pad2;
        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(op2, keccak256("u2"), 0);

        assertEq(paymaster.nonces(user1), 1);
        assertEq(paymaster.nonces(user2), 1);
    }

    // ============================================================
    // Comprehensive Access Control Matrix
    // ============================================================

    function test_Ownable_OnlyOwnerFunctions() public {
        vm.expectRevert();
        vm.prank(user);
        paymaster.setTrustedSigner(alice);

        vm.expectRevert();
        vm.prank(user);
        paymaster.setSenderWhitelist(alice, true);

        vm.expectRevert();
        vm.prank(user);
        paymaster.withdraw(payable(alice), 0.5 ether);
    }
}
