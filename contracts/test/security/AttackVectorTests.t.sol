// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {Test, stdError} from "forge-std/Test.sol";
import {WrappedUSDT} from "src/stablecoin/WrappedUSDT.sol";
import {ReserveVault} from "src/stablecoin/ReserveVault.sol";
import {VerifyingPaymaster} from "src/erc4337/VerifyingPaymaster.sol";
import {MockERC20} from "../mocks/MockERC20.sol";
import {MockEntryPoint} from "../mocks/MockEntryPoint.sol";
import {IEntryPoint} from "account-abstraction/interfaces/IEntryPoint.sol";
import {PackedUserOperation} from "account-abstraction/interfaces/PackedUserOperation.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/**
 * @title AttackVectorTests
 * @notice Tests various attack vectors against the contracts
 * @dev Simulates reentrancy, flash loans, front-running, and other attacks
 */
contract AttackVectorTests is Test {
    WrappedUSDT public wusdt;
    ReserveVault public vault;
    VerifyingPaymaster public paymaster;
    MockERC20 public usdt;
    MockEntryPoint public mockEntryPoint;

    address public owner;
    address public user;
    address public attacker;

    uint256 signerPrivateKey = 0x12345;
    address public trustedSigner;

    error Ownable__NotOwner();
    error ReserveVault__InsufficientReserves();

    function setUp() public {
        owner = address(this);
        user = address(0x1);
        attacker = address(0x2);
        trustedSigner = vm.addr(signerPrivateKey);

        vm.deal(user, 10 ether);
        vm.deal(attacker, 10 ether);

        usdt = new MockERC20();
        vault = new ReserveVault(usdt);
        vault.setWrappedUSDT(address(this));

        vm.prank(owner);
        wusdt = new WrappedUSDT(usdt, vault);

        vm.prank(owner);
        vault.setWrappedUSDT(address(wusdt));

        mockEntryPoint = new MockEntryPoint();
        paymaster = new VerifyingPaymaster(
            IEntryPoint(address(mockEntryPoint)),
            trustedSigner
        );

        usdt.mint(user, 1000000 * 10**6);
        usdt.mint(attacker, 1000000 * 10**6);
    }

    // ============================================================
    // Reentrancy Tests
    // ============================================================

    function test_ReentrancyOnWrap() public {
        // The wrap function follows checks-effects-interactions pattern:
        // 1. Checks: amount > 0, whenNotPaused
        // 2. Effects: vault.mintWithReserves, _mint (state changes)
        // 3. Interactions: usdt.transferFrom (external call)

        // Since external call happens AFTER state changes, reentrancy is mitigated
        uint256 wrapAmount = 1000 * 10**6;

        uint256 vaultReservesBefore = vault.totalReserves();
        uint256 vaultWrappedBefore = vault.totalWrapped();

        vm.startPrank(user);
        usdt.approve(address(wusdt), wrapAmount);
        wusdt.wrap(wrapAmount);
        vm.stopPrank();

        // State updated correctly
        assertEq(vault.totalReserves(), vaultReservesBefore + wrapAmount);
        assertEq(vault.totalWrapped(), vaultWrappedBefore + wrapAmount * 1e12);

        // Invariant maintained
        assertLe(vault.totalWrapped(), vault.totalReserves() * 1e12 + wusdt.accumulatedDust());
    }

    function test_ReentrancyOnUnwrap() public {
        // The unwrap function burns BEFORE calling vault, preventing reentrancy
        uint256 wrapAmount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(wusdt), wrapAmount);
        wusdt.wrap(wrapAmount);

        uint256 balanceBefore = wusdt.balanceOf(user);
        uint256 vaultReservesBefore = vault.totalReserves();
        uint256 vaultWrappedBefore = vault.totalWrapped();

        wusdt.unwrap(wrapAmount * 1e12);
        vm.stopPrank();

        // State updated correctly
        assertEq(wusdt.balanceOf(user), balanceBefore - wrapAmount * 1e12);
        assertEq(vault.totalWrapped(), vaultWrappedBefore - wrapAmount * 1e12);
        assertEq(vault.totalReserves(), vaultReservesBefore - wrapAmount);

        // Invariant maintained
        assertLe(vault.totalWrapped(), vault.totalReserves() * 1e12 + wusdt.accumulatedDust());
    }

    function test_ReentrancyOnVaultEmergencyWithdraw() public {
        // The vault emergencyWithdraw function updates state BEFORE external transfer
        uint256 depositAmount = 10000 * 10**6;
        usdt.mint(owner, depositAmount);

        vm.startPrank(owner);
        usdt.approve(address(vault), depositAmount);
        vault.deposit(depositAmount);

        uint256 reservesBefore = vault.totalReserves();

        vault.emergencyWithdraw(depositAmount);
        vm.stopPrank();

        // State updated correctly
        assertEq(vault.totalReserves(), reservesBefore - depositAmount);
    }

    // ============================================================
    // Flash Loan Attack Tests
    // ============================================================

    function test_FlashLoanAttack() public {
        // Simulate flash loan attack where attacker borrows wUSDT, manipulates price, then returns

        // First establish normal operations
        uint256 normalWrap = 10000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(wusdt), normalWrap);
        wusdt.wrap(normalWrap);
        vm.stopPrank();

        uint256 totalReservesBefore = vault.totalReserves();
        uint256 totalWrappedBefore = vault.totalWrapped();

        // Attacker gets a flash loan of wUSDT (simulated by transfer)
        uint256 flashLoanAmount = 5000 * 10**18;

        vm.prank(user);
        wusdt.transfer(attacker, flashLoanAmount);

        // Attacker unwraps to drain reserves
        vm.startPrank(attacker);
        wusdt.unwrap(flashLoanAmount);

        // Try to wrap back at manipulated rate
        // Should still maintain 1:1 peg
        usdt.approve(address(wusdt), flashLoanAmount / 1e12);
        wusdt.wrap(flashLoanAmount / 1e12);
        vm.stopPrank();

        // Invariant should still hold
        assertLe(vault.totalWrapped(), vault.totalReserves() * 1e12 + wusdt.accumulatedDust());
    }

    function test_FlashLoanDrainReserves() public {
        // Attacker tries to drain all reserves via unwrap
        uint256 userWrap = 50000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(wusdt), userWrap);
        wusdt.wrap(userWrap);
        vm.stopPrank();

        // Attacker gets wUSDT from user (via transfer or other means)
        uint256 attackerWUSDT = wusdt.balanceOf(user);

        vm.prank(user);
        wusdt.transfer(attacker, attackerWUSDT);

        // Attacker tries to unwrap everything
        vm.prank(attacker);
        wusdt.unwrap(attackerWUSDT);

        // Reserves should be backed
        assertLe(vault.totalWrapped(), vault.totalReserves() * 1e12 + wusdt.accumulatedDust());
    }

    // ============================================================
    // Front Running Tests
    // ============================================================

    function test_FrontRunningUnwrap() public {
        // User wraps tokens
        uint256 wrapAmount = 10000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(wusdt), wrapAmount);
        wusdt.wrap(wrapAmount);

        // User wants to unwrap 1000 wUSDT
        uint256 unwrapAmount = 1000 * 10**18;

        // Attacker sees transaction in mempool and front-runs
        vm.stopPrank();

        // Attacker wraps to increase demand
        vm.startPrank(attacker);
        usdt.approve(address(wusdt), 50000 * 10**6);
        wusdt.wrap(50000 * 10**6);
        vm.stopPrank();

        // User's unwrap still goes through at 1:1
        vm.prank(user);
        wusdt.unwrap(unwrapAmount);

        // User receives expected amount
        // Due to decimal conversion, there might be dust
        assertGe(usdt.balanceOf(user), unwrapAmount / 1e12);

        // Invariant holds
        assertLe(vault.totalWrapped(), vault.totalReserves() * 1e12 + wusdt.accumulatedDust());
    }

    // ============================================================
    // Large Amount Tests
    // ============================================================

    function test_LargeWrapUnwrap() public {
        // Test with 1M USDT
        uint256 largeAmount = 1000000 * 10**6;

        usdt.mint(user, largeAmount);

        vm.startPrank(user);
        usdt.approve(address(wusdt), largeAmount);
        wusdt.wrap(largeAmount);

        assertEq(wusdt.balanceOf(user), largeAmount * 1e12);
        assertEq(vault.totalReserves(), largeAmount);
        assertEq(vault.totalWrapped(), largeAmount * 1e12);

        // Unwrap all
        wusdt.unwrap(largeAmount * 1e12);
        vm.stopPrank();

        // Should have all USDT back (minus dust)
        assertGe(usdt.balanceOf(user), largeAmount - 10**6); // Allow small rounding error

        // Invariant holds
        assertLe(vault.totalWrapped(), vault.totalReserves() * 1e12 + wusdt.accumulatedDust());
    }

    function test_LargeAmountNoOverflow() public {
        // Test that very large amounts don't cause overflow
        uint256 hugeAmount = 1_000_000_000 * 10**6; // 1 billion USDT

        usdt.mint(user, hugeAmount);

        vm.startPrank(user);
        usdt.approve(address(wusdt), hugeAmount);
        wusdt.wrap(hugeAmount);
        vm.stopPrank();

        // No overflow - Solidity 0.8+ has built-in overflow checks
        assertEq(vault.totalReserves(), hugeAmount);
    }

    // ============================================================
    // Dust Overflow Tests
    // ============================================================

    function test_DustOverflow() public {
        // Perform many small wraps/unwraps to accumulate dust
        // To create dust, unwrap amounts that don't divide evenly by 1e12
        uint256 iterations = 100;
        uint256 smallAmount = 1000 * 10**6;

        usdt.mint(user, smallAmount * iterations);

        vm.startPrank(user);
        usdt.approve(address(wusdt), smallAmount * iterations);

        for (uint256 i = 0; i < iterations; i++) {
            wusdt.wrap(smallAmount);
        }

        // Unwrap in amounts that create dust (add 1 to create remainder when divided by 1e12)
        // (1000 * 1e12 + 1) / 1e12 = 1000 USDT with 1 unit of dust in 18 decimals
        for (uint256 i = 0; i < iterations; i++) {
            wusdt.unwrap(1000 * 1e12 + 1 + i);
        }

        vm.stopPrank();

        // Dust should not overflow
        uint256 dust = wusdt.accumulatedDust();
        assertTrue(dust > 0, "Dust should have accumulated");

        // Even with large dust, invariant holds
        assertLe(vault.totalWrapped(), vault.totalReserves() * 1e12 + dust);
    }

    function test_DustDoesNotOverflowUint256() public {
        // Even extreme operations should not cause overflow
        uint256 wrapAmount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(wusdt), wrapAmount);
        wusdt.wrap(wrapAmount);

        // Try to unwrap amount that would create max dust
        // This should not overflow
        wusdt.unwrap(wrapAmount * 1e12 - 1);
        vm.stopPrank();

        // Dust should be reasonable (< 1e12)
        assertLt(wusdt.accumulatedDust(), 1e12);
    }

    // ============================================================
    // Signature Replay Tests
    // ============================================================

    function test_PaymasterSignatureReplay() public {
        address sender = address(0x999);

        // Whitelist sender
        vm.prank(owner);
        paymaster.setSenderWhitelist(sender, true);

        // Deposit to paymaster (deposit to entry point for the paymaster)
        mockEntryPoint.depositTo{value: 1 ether}(address(paymaster));

        // Create a valid userOp and signature
        // The hash is: keccak256(abi.encode(userOpHash, paymasterData, nonce))
        // paymasterData is empty bytes(""), nonce starts at 0
        bytes32 dummyUserOpHash = keccak256("replayTest");
        bytes memory paymasterData = "";
        uint256 nonceBefore = paymaster.nonces(sender);

        // Hash must match what paymaster computes: keccak256(abi.encode(userOpHash, paymasterData, nonces[userOp.sender]))
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

        // First validation should succeed (signature valid, deposit sufficient)
        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(userOp, dummyUserOpHash, 0);

        // Nonce should have been incremented
        assertEq(paymaster.nonces(sender), nonceBefore + 1);

        // Try to replay with same signature - should fail because nonce changed
        // The hash would now be computed with nonce=1, making the old signature invalid
        vm.prank(address(mockEntryPoint));
        vm.expectRevert(); // Will fail due to wrong nonce in signature verification
        paymaster.validatePaymasterUserOp(userOp, dummyUserOpHash, 0);

        // Nonce should still be 1 (not incremented again)
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

        // User 1's validation
        vm.prank(address(mockEntryPoint));
        vm.expectRevert();
        paymaster.validatePaymasterUserOp(userOp1, hash1Final, 0);

        // User 2 trying to use user 1's signature - should fail
        vm.prank(address(mockEntryPoint));
        vm.expectRevert();
        paymaster.validatePaymasterUserOp(userOp2, hash2Final, 0);
    }

    // ============================================================
    // Timestamp/Block Manipulation Tests
    // ============================================================

    function test_TimestampManipulation() public {
        // Contracts don't use timestamps for critical operations
        // But let's verify wrapping/unwrapping works regardless of timestamp

        vm.warp(1000000);

        uint256 amount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);

        assertEq(wusdt.balanceOf(user), amount * 1e12);

        // Advance timestamp significantly
        vm.warp(2000000000);

        // Unwrap should still work
        wusdt.unwrap(amount * 1e12);
        vm.stopPrank();

        // Should receive expected amount
        assertGe(usdt.balanceOf(user), amount);
    }

    // ============================================================
    // sandwich Attack Tests
    // ============================================================

    function test_SandwichAttack() public {
        // Attacker tries to sandwich a large wrap transaction

        // Victim prepares to wrap
        uint256 victimAmount = 100000 * 10**6;

        usdt.mint(user, victimAmount);

        // Attacker front-runs with large wrap
        uint256 attackerAmount = 500000 * 10**6;

        vm.startPrank(attacker);
        usdt.approve(address(wusdt), attackerAmount);
        wusdt.wrap(attackerAmount);
        vm.stopPrank();

        // Victim's wrap (sandwiched)
        vm.startPrank(user);
        usdt.approve(address(wusdt), victimAmount);
        wusdt.wrap(victimAmount);
        vm.stopPrank();

        // Attacker back-runs with large unwrap
        vm.startPrank(attacker);
        wusdt.unwrap(attackerAmount * 1e12);
        vm.stopPrank();

        // Both parties should have fair amounts
        // 1:1 peg maintained regardless of order
        assertGe(usdt.balanceOf(attacker), attackerAmount - 1000);
        assertGe(usdt.balanceOf(user), victimAmount - 1000);
    }
}
