// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {Test, stdError} from "forge-std/Test.sol";
import {WrappedUSDT} from "src/stablecoin/WrappedUSDT.sol";
import {ReserveVault} from "src/stablecoin/ReserveVault.sol";
import {IReserveVault} from "src/stablecoin/IReserveVault.sol";
import {MockERC20} from "../mocks/MockERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/**
 * @title EdgeCaseTests
 * @notice Tests edge cases and boundary conditions
 * @dev Ensures contracts handle zero values, overflows, and edge cases correctly
 */
contract EdgeCaseTests is Test {
    WrappedUSDT public wusdt;
    ReserveVault public vault;
    MockERC20 public usdt;

    address public owner;
    address public user;
    address public zeroAddress = address(0);

    error WrappedUSDT__ZeroAmount();
    error ReserveVault__ZeroAmount();
    error ReserveVault__ZeroAddress();
    error ReserveVault__InsufficientReserves();
    error Ownable__ZeroAddress();

    function setUp() public {
        owner = address(this);
        user = address(0x1);

        vm.deal(user, 10 ether);

        usdt = new MockERC20();
        vault = new ReserveVault(usdt);
        vault.setWrappedUSDT(address(this));

        vm.prank(owner);
        wusdt = new WrappedUSDT(usdt, vault);

        vm.prank(owner);
        vault.setWrappedUSDT(address(wusdt));

        usdt.mint(user, 1000000 * 10**6);
    }

    // ============================================================
    // Zero Amount Tests
    // ============================================================

    function test_WrapZeroAmount() public {
        vm.startPrank(user);
        usdt.approve(address(wusdt), 0);

        vm.expectRevert(WrappedUSDT__ZeroAmount.selector);
        wusdt.wrap(0);
        vm.stopPrank();
    }

    function test_UnwrapZeroAmount() public {
        // First wrap some tokens
        uint256 amount = 1000 * 10**6;
        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);

        // Try to unwrap zero
        vm.expectRevert(WrappedUSDT__ZeroAmount.selector);
        wusdt.unwrap(0);
        vm.stopPrank();
    }

    function test_VaultDepositZero() public {
        vm.startPrank(user);
        usdt.approve(address(vault), 0);

        vm.expectRevert(ReserveVault__ZeroAmount.selector);
        vault.deposit(0);
        vm.stopPrank();
    }

    function test_VaultEmergencyWithdrawZero() public {
        // First deposit something
        uint256 amount = 1000 * 10**6;
        vm.startPrank(owner);
        usdt.approve(address(vault), amount);
        vault.deposit(amount);

        // Try to emergency withdraw zero
        vm.expectRevert(ReserveVault__ZeroAmount.selector);
        vault.emergencyWithdraw(0);
        vm.stopPrank();
    }

    function test_VaultMintZero() public {
        vm.expectRevert(ReserveVault__ZeroAmount.selector);
        vm.prank(address(wusdt));
        vault.mint(user, 0);
    }

    function test_VaultBurnZero() public {
        // First mint some tokens
        vm.prank(address(wusdt));
        vault.mint(user, 1000 * 10**18);

        vm.expectRevert(ReserveVault__ZeroAmount.selector);
        vm.prank(address(wusdt));
        vault.burn(user, 0);
    }

    // ============================================================
    // Insufficient Balance Tests
    // ============================================================

    function test_UnwrapMoreThanBalance() public {
        // First wrap some tokens
        uint256 amount = 1000 * 10**6;
        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);

        // Try to unwrap more than balance
        // OpenZeppelin's ERC20 burns with ERC20InsufficientBalance error
        vm.expectRevert();
        wusdt.unwrap(2000 * 10**18);
        vm.stopPrank();
    }

    function test_EmergencyWithdrawMoreThanReserves() public {
        // Deposit some tokens
        uint256 amount = 1000 * 10**6;
        vm.startPrank(owner);
        usdt.approve(address(vault), amount);
        vault.deposit(amount);

        // Try to emergency withdraw more than available
        vm.expectRevert(ReserveVault__InsufficientReserves.selector);
        vault.emergencyWithdraw(amount + 1);
        vm.stopPrank();
    }

    function test_TransferMoreThanBalance() public {
        // First wrap some tokens
        uint256 amount = 1000 * 10**6;
        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);

        // Try to transfer more than balance
        // OpenZeppelin's ERC20 uses ERC20InsufficientBalance error
        vm.expectRevert();
        wusdt.transfer(address(0x2), 2000 * 10**18);
        vm.stopPrank();
    }

    // ============================================================
    // Insufficient Allowance Tests
    // ============================================================

    function test_WrapWithInsufficientAllowance() public {
        uint256 amount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(wusdt), amount - 1);

        vm.expectRevert();
        wusdt.wrap(amount);
        vm.stopPrank();
    }

    function test_VaultDepositWithInsufficientAllowance() public {
        uint256 amount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(vault), amount - 1);

        vm.expectRevert();
        vault.deposit(amount);
        vm.stopPrank();
    }

    // ============================================================
    // Pause State Tests
    // ============================================================

    function test_WrapWhenPaused() public {
        uint256 amount = 1000 * 10**6;

        vm.prank(owner);
        wusdt.pauseTransfers();

        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);

        vm.expectRevert();
        wusdt.wrap(amount);
        vm.stopPrank();
    }

    function test_UnwrapWhenPaused() public {
        uint256 amount = 1000 * 10**6;
        uint256 expectedWUSDT = 1000 * 10**18;

        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);
        vm.stopPrank();

        vm.prank(owner);
        wusdt.pauseTransfers();

        vm.prank(user);
        vm.expectRevert();
        wusdt.unwrap(expectedWUSDT);
    }

    function test_TransferWhenPaused() public {
        uint256 amount = 1000 * 10**6;
        uint256 expectedWUSDT = 1000 * 10**18;

        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);
        vm.stopPrank();

        vm.prank(owner);
        wusdt.pauseTransfers();

        vm.prank(user);
        vm.expectRevert();
        wusdt.transfer(address(0x2), expectedWUSDT);
    }

    function test_VaultDepositWhenPaused() public {
        uint256 amount = 1000 * 10**6;

        vm.prank(owner);
        vault.pauseDeposits();

        vm.startPrank(user);
        usdt.approve(address(vault), amount);

        vm.expectRevert();
        vault.deposit(amount);
        vm.stopPrank();
    }

    function test_VaultEmergencyWithdrawWhenPaused() public {
        uint256 amount = 1000 * 10**6;

        vm.startPrank(owner);
        usdt.approve(address(vault), amount);
        vault.deposit(amount);

        vault.pauseDeposits();

        // Emergency withdraw should also fail when paused
        vm.expectRevert();
        vault.emergencyWithdraw(amount);
        vm.stopPrank();
    }

    // ============================================================
    // Zero Address Tests
    // ============================================================

    function test_MintToZeroAddress() public {
        vm.expectRevert(ReserveVault__ZeroAddress.selector);
        vm.prank(address(wusdt));
        vault.mint(zeroAddress, 1000 * 10**18);
    }

    function test_MintWithReservesToZeroAddress() public {
        vm.expectRevert(ReserveVault__ZeroAddress.selector);
        vm.prank(address(wusdt));
        vault.mintWithReserves(zeroAddress, 1000 * 10**18, 1000 * 10**6);
    }

    function test_BurnAndWithdrawToZeroAddress() public {
        // Set up reserves first
        uint256 depositAmount = 2000 * 10**6;
        usdt.mint(owner, depositAmount);

        vm.startPrank(owner);
        usdt.approve(address(vault), depositAmount);
        vault.deposit(depositAmount);
        vm.stopPrank();

        // Mint some tokens first so we have something to burn
        vm.prank(address(wusdt));
        vault.mint(user, 1000 * 10**18);

        // Note: burnAndWithdraw doesn't explicitly check for zero address
        // But safeTransfer to zero address will fail with ERC20InvalidReceiver
        // This test documents the current behavior
        vm.prank(address(wusdt));
        vm.expectRevert(); // SafeERC20.safeTransfer reverts when sending to zero address
        vault.burnAndWithdraw(zeroAddress, 500 * 10**18, 500 * 10**6);
    }

    function test_SetWrappedUSDTToZeroAddress() public {
        vm.expectRevert(Ownable__ZeroAddress.selector);
        vm.prank(owner);
        vault.setWrappedUSDT(zeroAddress);
    }

    function test_ConstructorWithZeroAddressUSDT() public {
        vm.expectRevert(Ownable__ZeroAddress.selector);
        new ReserveVault(IERC20(zeroAddress));
    }

    function test_WrappedUSDTConstructorWithZeroAddress() public {
        vm.expectRevert(Ownable__ZeroAddress.selector);
        new WrappedUSDT(IERC20(zeroAddress), vault);
    }

    function test_WrappedUSDTConstructorWithZeroVault() public {
        vm.expectRevert(Ownable__ZeroAddress.selector);
        new WrappedUSDT(usdt, IReserveVault(zeroAddress));
    }

    // ============================================================
    // Idempotent Pause/Unpause Tests
    // ============================================================

    function test_DoublePause() public {
        vm.prank(owner);
        wusdt.pauseTransfers();
        assertTrue(wusdt.paused());

        // Pause again - should be allowed (idempotent at contract level)
        // though it's already paused
        vm.prank(owner);
        wusdt.pauseTransfers();
        assertTrue(wusdt.paused());
    }

    function test_DoubleUnpause() public {
        vm.prank(owner);
        wusdt.pauseTransfers();
        assertTrue(wusdt.paused());

        vm.prank(owner);
        wusdt.unpauseTransfers();
        assertFalse(wusdt.paused());

        // Unpause again when not paused - should be allowed
        vm.prank(owner);
        wusdt.unpauseTransfers();
        assertFalse(wusdt.paused());
    }

    function test_DoublePauseVault() public {
        vm.prank(owner);
        vault.pauseDeposits();
        assertTrue(vault.paused());

        vm.prank(owner);
        vault.pauseDeposits();
        assertTrue(vault.paused());
    }

    function test_DoubleUnpauseVault() public {
        vm.prank(owner);
        vault.pauseDeposits();
        assertTrue(vault.paused());

        vm.prank(owner);
        vault.unpauseDeposits();
        assertFalse(vault.paused());

        vm.prank(owner);
        vault.unpauseDeposits();
        assertFalse(vault.paused());
    }

    // ============================================================
    // Boundary Value Tests
    // ============================================================

    function test_WrapOneUnit() public {
        uint256 amount = 1; // 1 wei of USDT (6 decimals, so 0.000001 USDT)

        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);
        vm.stopPrank();

        // 1 USDT (6 decimals) * 1e12 = 1e12 wUSDT (18 decimals)
        assertEq(wusdt.balanceOf(user), 1e12);
    }

    function test_WrapMaxUint256() public {
        // This should fail due to insufficient balance, not overflow
        vm.startPrank(user);
        usdt.approve(address(wusdt), type(uint256).max);

        vm.expectRevert();
        wusdt.wrap(type(uint256).max);
        vm.stopPrank();
    }

    function test_UnwrapOneUnit() public {
        uint256 wrapAmount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(wusdt), wrapAmount);
        wusdt.wrap(wrapAmount);

        // Try to unwrap 1 wUSDT - this should revert because
        // 1 wUSDT = 0 USDT (truncated), and burnAndWithdraw requires amountReserves > 0
        vm.expectRevert(ReserveVault__ZeroAmount.selector);
        wusdt.unwrap(1);
        vm.stopPrank();

        // To create dust, need to unwrap amounts that result in non-zero USDT
        // but have a remainder. Let's unwrap 1000 * 1e12 + 1 wUSDT = 1000 USDT + 1 dust
        vm.prank(user);
        wusdt.unwrap(1000 * 1e12 + 1);

        // Now dust should be accumulated
        assertEq(wusdt.accumulatedDust(), 1);
    }

    function test_DecimalEdgeCase() public {
        // Test amounts that are right at the decimal conversion boundary
        uint256 amount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);

        // Unwrap exactly what we wrapped
        wusdt.unwrap(amount * 1e12);
        vm.stopPrank();

        // Should have zero balance
        assertEq(wusdt.balanceOf(user), 0);
    }

    // ============================================================
    // Rescue Token Edge Cases
    // ============================================================

    function test_RescueUSDTShouldFail() public {
        // Owner cannot rescue USDT from the vault
        uint256 amount = 1000 * 10**6;

        vm.startPrank(owner);
        usdt.approve(address(vault), amount);
        vault.deposit(amount);

        // Try to rescue USDT - should fail
        vm.expectRevert(ReserveVault__ZeroAddress.selector);
        vault.rescueTokens(address(usdt), amount);
        vm.stopPrank();
    }

    function test_RescueZeroAmount() public {
        MockERC20 otherToken = new MockERC20();
        otherToken.mint(address(vault), 1000 * 10**6);

        // Rescuing 0 should now revert with ReserveVault__ZeroAmount (new validation)
        vm.prank(owner);
        vm.expectRevert(ReserveVault__ZeroAmount.selector);
        vault.rescueTokens(address(otherToken), 0);
    }

    // ============================================================
    // Dust Claiming Edge Cases
    // ============================================================

    function test_ClaimDustWhenNoDust() public {
        uint256 balanceBefore = usdt.balanceOf(owner);

        // Claim when there's no dust
        vm.prank(owner);
        wusdt.claimDust();

        // Balance should not change
        assertEq(usdt.balanceOf(owner), balanceBefore);
    }

    function test_ClaimDustMultipleTimes() public {
        uint256 wrapAmount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(wusdt), wrapAmount);
        wusdt.wrap(wrapAmount);
        vm.stopPrank();

        // Mint some extra wUSDT to owner via vault so claimDust can burn it
        uint256 extraForDust = 10 * 1e18;
        vm.prank(address(wusdt));
        vault.mint(owner, extraForDust);

        // Deposit extra USDT to vault to cover the wUSDT backing requirement
        uint256 extraReserves = 10 * 10**6;
        usdt.mint(owner, extraReserves);
        vm.startPrank(owner);
        usdt.approve(address(vault), extraReserves);
        vault.deposit(extraReserves);
        vm.stopPrank();

        // Create dust by unwrapping amounts that don't divide evenly by 1e12
        // Need at least 1e12 dust to be claimable
        vm.startPrank(user);
        wusdt.unwrap(1000 * 1e12 + 5e11);  // Creates 5e11 dust
        wusdt.unwrap(1000 * 1e12 + 5e11);  // Creates another 5e11 dust, total 1e12
        vm.stopPrank();

        uint256 dustBeforeClaim = wusdt.accumulatedDust();
        assertGe(dustBeforeClaim, 1e12, "Dust should be at least 1e12 to be claimable");

        // First claim
        vm.prank(owner);
        wusdt.claimDust();

        // Dust should be reset
        assertEq(wusdt.accumulatedDust(), 0);

        // Second claim - should be no-op
        vm.prank(owner);
        wusdt.claimDust();

        assertEq(wusdt.accumulatedDust(), 0);
    }
}
