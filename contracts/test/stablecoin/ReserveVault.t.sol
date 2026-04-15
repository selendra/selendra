// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {Test, console} from "forge-std/Test.sol";
import {ReserveVault} from "src/stablecoin/ReserveVault.sol";
import {IReserveVault} from "src/stablecoin/IReserveVault.sol";
import {MockERC20} from "../mocks/MockERC20.sol";

contract ReserveVaultTest is Test {
    ReserveVault public vault;
    MockERC20 public usdt;

    address public owner;
    address public user;
    address public wrappedContract;

    error Ownable__NotOwner();
    error Ownable__ZeroAddress();

    function setUp() public {
        owner = address(this);
        user = address(0x1);
        wrappedContract = address(0x2);

        vm.deal(user, 10 ether);

        usdt = new MockERC20();
        vault = new ReserveVault(usdt);
        vault.setWrappedUSDT(wrappedContract);

        usdt.mint(user, 1000000 * 10**6);
    }

    function test_Deposit() public {
        uint256 amount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(vault), amount);
        vault.deposit(amount);
        vm.stopPrank();

        assertEq(vault.totalReserves(), amount);
        assertEq(usdt.balanceOf(address(vault)), amount);
    }

    function test_EmergencyWithdraw() public {
        uint256 amount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(vault), amount);
        vault.deposit(amount);
        vm.stopPrank();

        // Only owner can call emergencyWithdraw
        uint256 ownerBalanceBefore = usdt.balanceOf(owner);
        vm.prank(owner);
        vault.emergencyWithdraw(amount);

        assertEq(vault.totalReserves(), 0);
        assertEq(usdt.balanceOf(owner), ownerBalanceBefore + amount);
    }

    function test_EmergencyWithdraw_OnlyOwner() public {
        uint256 amount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(vault), amount);
        vault.deposit(amount);

        // Non-owner cannot call emergencyWithdraw
        vm.expectRevert();
        vault.emergencyWithdraw(amount);
        vm.stopPrank();
    }

    function test_EmergencyWithdraw_RespectsInvariant() public {
        // Deposit extra reserves
        uint256 depositAmount = 2000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(vault), depositAmount);
        vault.deposit(depositAmount);
        vm.stopPrank();

        // Mint some wUSDT (simulating wrap)
        vm.prank(wrappedContract);
        vault.mint(user, 1000 * 10**18);

        // Now totalReserves = 2000 * 1e6, totalWrapped = 1000 * 1e18
        // Can only withdraw up to 1000 * 1e6 (must keep 1000 * 1e6 to back the 1000 * 1e18 wUSDT)
        vm.prank(owner);
        vault.emergencyWithdraw(1000 * 10**6);

        assertEq(vault.totalReserves(), 1000 * 10**6);

        // Try to withdraw more - should fail invariant check
        vm.expectRevert();
        vm.prank(owner);
        vault.emergencyWithdraw(1 * 10**6);
    }

    function test_Mint() public {
        uint256 amount = 1000 * 10**18;

        vm.prank(wrappedContract);
        vault.mint(user, amount);

        assertEq(vault.totalWrapped(), amount);
    }

    function test_Burn() public {
        uint256 amount = 1000 * 10**18;

        vm.prank(wrappedContract);
        vault.mint(user, amount);
        vm.prank(wrappedContract);
        vault.burn(user, amount);

        assertEq(vault.totalWrapped(), 0);
    }

    function test_Pause() public {
        vm.prank(owner);
        vault.pauseDeposits();

        uint256 amount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(vault), amount);
        vm.expectRevert();
        vault.deposit(amount);
        vm.stopPrank();
    }

    function test_OnlyWrappedCanMint() public {
        vm.expectRevert();
        vm.prank(user);
        vault.mint(user, 1000 * 10**18);
    }

    function test_OnlyOwnerCanPause() public {
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        vault.pauseDeposits();
    }

    function testFuzz_DepositWithdraw(uint256 amount) public {
        vm.assume(amount > 0 && amount <= 1000000 * 10**6);

        vm.startPrank(user);
        usdt.approve(address(vault), amount);
        vault.deposit(amount);
        assertEq(vault.totalReserves(), amount);
        vm.stopPrank();

        // Owner can emergency withdraw
        vm.prank(owner);
        vault.emergencyWithdraw(amount);
        assertEq(vault.totalReserves(), 0);
    }

    function test_ReserveBacked() public {
        uint256 depositAmount = 10000 * 10**6;
        uint256 mintAmount = 10000 * 10**18;

        vm.startPrank(user);
        usdt.approve(address(vault), depositAmount);
        vault.deposit(depositAmount);
        vm.stopPrank();

        vm.prank(wrappedContract);
        vault.mint(user, mintAmount);

        assertEq(vault.totalReserves(), depositAmount);
        assertEq(vault.totalWrapped(), mintAmount);
    }
}
