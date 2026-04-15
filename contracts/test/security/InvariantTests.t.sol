// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {Test, console} from "forge-std/Test.sol";
import {WrappedUSDT} from "src/stablecoin/WrappedUSDT.sol";
import {ReserveVault} from "src/stablecoin/ReserveVault.sol";
import {MockERC20} from "../mocks/MockERC20.sol";

/**
 * @title InvariantTests
 * @notice Tests reserve invariant: totalWrapped * 1e12 <= totalReserves * 1e12 + accumulatedDust tolerance
 * @dev Core invariant: wUSDT supply must always be backed by USDT reserves (accounting for decimal conversion)
 */
contract InvariantTests is Test {
    WrappedUSDT public wusdt;
    ReserveVault public vault;
    MockERC20 public usdt;

    address public owner;
    address public user;
    address public alice;

    error Ownable__NotOwner();

    function setUp() public {
        owner = address(this);
        user = address(0x1);
        alice = address(0x2);

        vm.deal(user, 10 ether);
        vm.deal(alice, 10 ether);

        usdt = new MockERC20();
        vault = new ReserveVault(usdt);
        vault.setWrappedUSDT(address(this));

        vm.prank(owner);
        wusdt = new WrappedUSDT(usdt, vault);

        vm.prank(owner);
        vault.setWrappedUSDT(address(wusdt));

        usdt.mint(user, 1000000 * 10**6);
        usdt.mint(alice, 1000000 * 10**6);
    }

    /// @notice Check reserve invariant: totalWrapped <= totalReserves * 1e12 + accumulatedDust
    function checkInvariant() internal view {
        uint256 totalWrapped = vault.totalWrapped();
        uint256 totalReserves = vault.totalReserves();
        uint256 accumulatedDust = wusdt.accumulatedDust();

        // Convert reserves to 18 decimals for comparison
        uint256 reservesIn18 = totalReserves * 1e12;

        // Invariant: wrapped supply must be <= reserves + dust (dust is in 18 decimals)
        assertLe(totalWrapped, reservesIn18 + accumulatedDust, "Invariant broken: wrapped > reserves + dust");
    }

    function test_ReserveInvariantAfterWrap() public {
        uint256 amount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);
        vm.stopPrank();

        checkInvariant();
    }

    function test_ReserveInvariantAfterUnwrap() public {
        uint256 amount = 1000 * 10**6;
        uint256 expectedWUSDT = 1000 * 10**18;

        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);
        wusdt.unwrap(expectedWUSDT);
        vm.stopPrank();

        checkInvariant();
    }

    function test_ReserveInvariantAfterMultipleOps() public {
        uint256 amount1 = 1000 * 10**6;
        uint256 amount2 = 500 * 10**6;
        uint256 unwrapAmount = 300 * 10**18;

        vm.startPrank(user);
        usdt.approve(address(wusdt), amount1 + amount2);

        // First wrap
        wusdt.wrap(amount1);
        checkInvariant();

        // Second wrap
        wusdt.wrap(amount2);
        checkInvariant();

        // Partial unwrap
        wusdt.unwrap(unwrapAmount);
        vm.stopPrank();

        checkInvariant();
    }

    function test_CannotWithdrawBelowInvariant() public {
        // First wrap some tokens to establish reserves
        uint256 wrapAmount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(wusdt), wrapAmount);
        wusdt.wrap(wrapAmount);
        vm.stopPrank();

        // Owner deposits extra reserves
        uint256 extraReserves = 100 * 10**6;
        usdt.mint(owner, extraReserves);

        vm.startPrank(owner);
        usdt.approve(address(vault), extraReserves);
        vault.deposit(extraReserves);

        // Try to emergency withdraw more than allowed - should revert
        // Current state: totalWrapped = 1000 * 1e18, totalReserves = 1100 * 1e6
        // Max withdrawal: (1100 - 1000) * 1e6 = 100 * 1e6
        // Trying to withdraw 101 should fail
        vm.expectRevert();
        vault.emergencyWithdraw(101 * 10**6);
        vm.stopPrank();

        checkInvariant();
    }

    function testFuzz_ReserveInvariant(uint256 wrapAmount) public {
        // Bound amount to reasonable values
        vm.assume(wrapAmount > 0 && wrapAmount <= 1000000 * 10**6);

        usdt.mint(user, wrapAmount);

        vm.startPrank(user);
        usdt.approve(address(wusdt), wrapAmount);
        wusdt.wrap(wrapAmount);
        vm.stopPrank();

        checkInvariant();
    }

    function test_DustAccumulation() public {
        // Wrapping amounts that don't divide evenly creates dust
        // To create dust, unwrap amounts that have a remainder when divided by 1e12

        uint256 wrapAmount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(wusdt), wrapAmount);
        wusdt.wrap(wrapAmount);

        // Unwrap amounts that create dust (add 1 to create remainder)
        // (1000 * 1e12 + 1) / 1e12 = 1000 USDT with 1 dust unit
        wusdt.unwrap(1000 * 1e12 + 1);
        assertGt(wusdt.accumulatedDust(), 0, "Dust should accumulate");

        // More unwraps with dust
        wusdt.unwrap(1000 * 1e12 + 2);
        wusdt.unwrap(1000 * 1e12 + 3);

        uint256 dustAfter = wusdt.accumulatedDust();
        assertGt(dustAfter, 1, "Dust should grow");
        vm.stopPrank();

        checkInvariant();
    }

    function test_ClaimDust() public {
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

        // Create some dust by unwrapping amounts that don't divide evenly by 1e12
        // Need at least 1e12 dust to be claimable as 1 USDT
        vm.startPrank(user);
        wusdt.unwrap(1000 * 1e12 + 5e11);  // Creates 5e11 dust
        wusdt.unwrap(1000 * 1e12 + 5e11);  // Creates another 5e11 dust, total 1e12
        vm.stopPrank();

        uint256 dustBefore = wusdt.accumulatedDust();
        assertGe(dustBefore, 1e12, "Dust should be at least 1e12 to be claimable");

        uint256 ownerBalanceBefore = usdt.balanceOf(owner);

        // Owner claims dust
        vm.prank(owner);
        wusdt.claimDust();

        // Dust should be reset
        assertEq(wusdt.accumulatedDust(), 0);

        // Owner should have received dust (converted to 6 decimals)
        uint256 expectedDustIn6 = dustBefore / 1e12;
        assertEq(usdt.balanceOf(owner), ownerBalanceBefore + expectedDustIn6);
    }

    function test_ClaimDust_OnlyOwner() public {
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        wusdt.claimDust();
    }

    function test_Pausable_CannotPausePublicly() public {
        // This is the critical security fix - pause/unpause should NOT be public
        // They should only be callable by owner via pauseTransfers/unpauseTransfers

        // Try to call internal _pause function - not accessible externally
        // Pausable contract uses internal _pause/_unpause, only exposed via whenNotPaused modifier

        // Verify that pause() and unpause() are NOT public functions
        // This is a compile-time check - the Pausable contract only has internal _pause/_unpause

        // The only way to pause is through owner functions:
        vm.prank(owner);
        wusdt.pauseTransfers();
        assertTrue(wusdt.paused());

        vm.prank(owner);
        wusdt.unpauseTransfers();
        assertFalse(wusdt.paused());

        // Non-owner cannot pause
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        wusdt.pauseTransfers();
    }

    function test_InvariantHoldsAfterDustClaim() public {
        uint256 wrapAmount = 10000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(wusdt), wrapAmount);
        wusdt.wrap(wrapAmount);

        // Create significant dust
        for (uint256 i = 0; i < 100; i++) {
            wusdt.unwrap((1000 + i) * 1e12);
        }
        vm.stopPrank();

        // Claim dust
        vm.prank(owner);
        wusdt.claimDust();

        // Invariant should still hold
        checkInvariant();
    }

    function test_MultiUserInvariant() public {
        uint256 amount1 = 5000 * 10**6;
        uint256 amount2 = 3000 * 10**6;

        // User 1 wraps
        vm.startPrank(user);
        usdt.approve(address(wusdt), amount1);
        wusdt.wrap(amount1);
        vm.stopPrank();

        checkInvariant();

        // User 2 wraps
        vm.startPrank(alice);
        usdt.approve(address(wusdt), amount2);
        wusdt.wrap(amount2);
        vm.stopPrank();

        checkInvariant();

        // User 1 partially unwraps
        vm.prank(user);
        wusdt.unwrap(1000 * 10**18);

        checkInvariant();

        // User 2 partially unwraps
        vm.prank(alice);
        wusdt.unwrap(500 * 10**18);

        checkInvariant();
    }
}
