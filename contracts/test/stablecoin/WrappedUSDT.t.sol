// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {Test, console} from "forge-std/Test.sol";
import {WrappedUSDT} from "src/stablecoin/WrappedUSDT.sol";
import {ReserveVault} from "src/stablecoin/ReserveVault.sol";
import {MockERC20} from "../mocks/MockERC20.sol";

contract WrappedUSDTTest is Test {
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

    function test_Wrap() public {
        uint256 amount = 1000 * 10**6;
        uint256 expectedWUSDT = 1000 * 10**18;

        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);
        vm.stopPrank();

        assertEq(wusdt.balanceOf(user), expectedWUSDT);
        assertEq(vault.totalReserves(), amount);
        assertEq(vault.totalWrapped(), expectedWUSDT);
    }

    function test_Unwrap() public {
        uint256 amount = 1000 * 10**6;
        uint256 expectedWUSDT = 1000 * 10**18;

        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);
        wusdt.unwrap(expectedWUSDT);
        vm.stopPrank();

        assertEq(wusdt.balanceOf(user), 0);
        assertEq(usdt.balanceOf(user), 1000000 * 10**6);
    }

    function test_Transfer() public {
        uint256 amount = 1000 * 10**6;
        uint256 expectedWUSDT = 1000 * 10**18;

        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);
        wusdt.transfer(alice, expectedWUSDT);
        vm.stopPrank();

        assertEq(wusdt.balanceOf(user), 0);
        assertEq(wusdt.balanceOf(alice), expectedWUSDT);
    }

    function test_Pause() public {
        uint256 amount = 1000 * 10**6;

        vm.prank(owner);
        wusdt.pauseTransfers();

        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        vm.expectRevert();
        wusdt.wrap(amount);
        vm.stopPrank();
    }

    function test_Unpause() public {
        uint256 amount = 1000 * 10**6;

        vm.prank(owner);
        wusdt.pauseTransfers();

        vm.prank(owner);
        wusdt.unpauseTransfers();

        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);
        vm.stopPrank();

        assertEq(wusdt.balanceOf(user), 1000 * 10**18);
    }

    function test_Decimals() public view {
        assertEq(wusdt.decimals(), 18);
    }

    function test_NameAndSymbol() public view {
        assertEq(wusdt.name(), "Wrapped USDT");
        assertEq(wusdt.symbol(), "wUSDT");
    }

    function testFuzz_Wrap(uint256 amount) public {
        vm.assume(amount > 0 && amount <= 1000000 * 10**6);
        
        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);
        vm.stopPrank();

        uint256 expectedWUSDT = amount * 1e12;
        assertEq(wusdt.balanceOf(user), expectedWUSDT);
    }

    function test_OnlyOwnerCanPause() public {
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        wusdt.pauseTransfers();
    }
}
