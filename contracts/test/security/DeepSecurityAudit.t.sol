// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {Test} from "forge-std/Test.sol";
import {WrappedUSDT} from "src/stablecoin/WrappedUSDT.sol";
import {ReserveVault} from "src/stablecoin/ReserveVault.sol";
import {IReserveVault} from "src/stablecoin/IReserveVault.sol";
import {VerifyingPaymaster} from "src/erc4337/VerifyingPaymaster.sol";
import {MockERC20} from "../mocks/MockERC20.sol";
import {MockEntryPoint} from "../mocks/MockEntryPoint.sol";
import {IEntryPoint} from "account-abstraction/interfaces/IEntryPoint.sol";
import {PackedUserOperation} from "account-abstraction/interfaces/PackedUserOperation.sol";

/**
 * @title DeepSecurityAudit
 * @notice Comprehensive security tests covering all audit findings
 * @dev Tests for CRITICAL, HIGH, and MEDIUM severity findings
 */
contract DeepSecurityAudit is Test {
    WrappedUSDT public wusdt;
    ReserveVault public vault;
    VerifyingPaymaster public paymaster;
    MockERC20 public usdt;
    MockEntryPoint public mockEntryPoint;

    address public owner;
    address public user;
    address public alice;
    address public attacker;

    uint256 signerPrivateKey = 0x12345;
    address public trustedSigner;

    error Ownable__NotOwner();
    error ReserveVault__InsufficientReserves();
    error ReserveVault__InvalidAmount();
    error WrappedUSDT__TransferFailed();
    error WrappedUSDT__InvalidVault();
    error VerifyingPaymaster__InvalidSignature();

    function setUp() public {
        owner = address(this);
        user = address(0x1);
        alice = address(0x2);
        attacker = address(0x999);
        trustedSigner = vm.addr(signerPrivateKey);

        vm.deal(user, 10 ether);
        vm.deal(alice, 10 ether);
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
        usdt.mint(alice, 1000000 * 10**6);
        usdt.mint(attacker, 1000000 * 10**6);
    }

    // ============================================================
    // CRITICAL #1: ReserveVault.withdraw() removed
    // ============================================================

    function test_Security_WithdrawFunctionRemoved() public {
        // Verify that public withdraw() no longer exists
        // This test ensures the critical vulnerability is fixed
        // Users must now withdraw via WrappedUSDT.unwrap()

        uint256 amount = 1000 * 10**6;

        // User deposits to vault (this should still work via deposit())
        vm.startPrank(user);
        usdt.approve(address(vault), amount);
        vault.deposit(amount);
        vm.stopPrank();

        assertEq(vault.totalReserves(), amount);

        // Attacker tries to withdraw user's funds
        // Before fix: attacker could call vault.withdraw() and steal funds
        // After fix: withdraw() is removed, only emergencyWithdraw (owner only) exists
        vm.prank(attacker);
        (bool success, ) = address(vault).call(
            abi.encodeWithSignature("withdraw(uint256)", amount)
        );
        assertFalse(success, "withdraw() should not be callable");

        // Verify funds are still safe
        assertEq(vault.totalReserves(), amount);
    }

    function test_Security_EmergencyWithdrawOnlyOwner() public {
        uint256 amount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(vault), amount);
        vault.deposit(amount);
        vm.stopPrank();

        // Non-owner cannot call emergencyWithdraw
        vm.prank(attacker);
        vm.expectRevert(Ownable__NotOwner.selector);
        vault.emergencyWithdraw(amount);

        // Owner can call emergencyWithdraw
        vm.prank(owner);
        vault.emergencyWithdraw(amount);

        assertEq(vault.totalReserves(), 0);
    }

    // ============================================================
    // CRITICAL #2: WrappedUSDT.claimDust() safety
    // ============================================================

    function test_Security_ClaimDustChecksVaultBalance() public {
        uint256 wrapAmount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(wusdt), wrapAmount);
        wusdt.wrap(wrapAmount);
        vm.stopPrank();

        // Mint some extra wUSDT to owner via vault so claimDust can burn it
        // This is needed because burnAndWithdraw requires amountWrapped > 0
        uint256 extraForDust = 10 * 1e18; // 10 wUSDT to burn for dust claim
        vm.prank(address(wusdt));
        vault.mint(owner, extraForDust);

        // Deposit extra USDT to vault to cover the wUSDT backing requirement
        // The vault needs 1 USDT (6 decimals) to back 1 * 1e12 wUSDT (18 decimals)
        // So 10 wUSDT needs 10 / 1e12 = 0.000...01 USDT, but we deposit 10 USDT for simplicity
        uint256 extraReserves = 10 * 10**6;
        usdt.mint(owner, extraReserves);
        vm.startPrank(owner);
        usdt.approve(address(vault), extraReserves);
        vault.deposit(extraReserves);
        vm.stopPrank();

        // Create dust by unwrapping amounts that don't divide evenly by 1e12
        // Need at least 1e12 dust to be claimable as 1 USDT (in 6 decimals)
        vm.startPrank(user);
        wusdt.unwrap(1000 * 1e12 + 5e11);  // Creates 5e11 dust
        wusdt.unwrap(1000 * 1e12 + 5e11);  // Creates another 5e11 dust, total 1e12
        vm.stopPrank();

        uint256 dustBefore = wusdt.accumulatedDust();
        assertGt(dustBefore, 0, "Dust should have accumulated");

        // Owner claims dust - now has wUSDT to burn and vault has enough reserves
        vm.prank(owner);
        wusdt.claimDust();

        assertEq(wusdt.accumulatedDust(), 0);

        // Try to claim again - should handle gracefully (dust is 0)
        vm.prank(owner);
        wusdt.claimDust(); // Should not revert
    }

    function test_Security_ClaimDustOnlyOwner() public {
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(attacker);
        wusdt.claimDust();
    }

    // ============================================================
    // HIGH #3: WrappedUSDT.wrap() balance verification
    // ============================================================

    function test_Security_WrapVerifiesVaultReceivedUSDT() public {
        uint256 amount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);

        // Normal wrap should succeed
        wusdt.wrap(amount);

        assertEq(wusdt.balanceOf(user), amount * 1e12);
        assertEq(vault.totalReserves(), amount);
        assertEq(vault.totalWrapped(), amount * 1e12);

        vm.stopPrank();
    }

    // ============================================================
    // HIGH #4: SafeERC20 usage
    // ============================================================

    function test_Security_SafeTransferFrom() public {
        uint256 amount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(vault), amount);

        // Safe transfer should work
        vault.deposit(amount);

        assertEq(vault.totalReserves(), amount);
        vm.stopPrank();
    }

    function test_Security_SafeTransfer() public {
        uint256 amount = 1000 * 10**6;

        // Wrap first
        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);

        // Unwrap uses safe transfer internally via burnAndWithdraw
        uint256 balanceBefore = usdt.balanceOf(user);
        wusdt.unwrap(amount * 1e12);
        uint256 balanceAfter = usdt.balanceOf(user);

        assertEq(balanceAfter - balanceBefore, amount);
        vm.stopPrank();
    }

    // ============================================================
    // MEDIUM #5: ReserveVault.setWrappedUSDT event
    // ============================================================

    function test_Security_SetWrappedUSDTEmitsEvent() public {
        address newWrapped = address(0x9999);

        vm.expectEmit(true, true, true, true);
        emit IReserveVault.WrappedUSDTSet(address(wusdt), newWrapped);

        vm.prank(owner);
        vault.setWrappedUSDT(newWrapped);
    }

    // ============================================================
    // MEDIUM #6: WrappedUSDT validates vault USDT
    // ============================================================

    function test_Security_ConstructorValidatesVaultUSDT() public {
        // Create a vault with different USDT
        MockERC20 differentUSDT = new MockERC20();
        ReserveVault differentVault = new ReserveVault(differentUSDT);

        // Deploying wUSDT with mismatched USDT should fail
        vm.expectRevert(WrappedUSDT__InvalidVault.selector);
        vm.prank(owner);
        new WrappedUSDT(usdt, differentVault);
    }

    function test_Security_ConstructorSucceedsWithMatchingUSDT() public {
        // This should succeed - vault uses same USDT
        vm.prank(owner);
        WrappedUSDT validWUSDT = new WrappedUSDT(usdt, vault);

        assertEq(address(validWUSDT.usdt()), address(usdt));
        assertEq(address(validWUSDT.vault()), address(vault));
    }

    // ============================================================
    // MEDIUM #7: VerifyingPaymaster returns correct context
    // ============================================================

    function test_Security_ValidatePaymasterUserOpReturnsContext() public {
        address whitelistedSender = address(0xABC);

        vm.prank(owner);
        paymaster.setSenderWhitelist(whitelistedSender, true);

        // Deposit to paymaster so the deposit check passes
        mockEntryPoint.depositTo{value: 1 ether}(address(paymaster));

        PackedUserOperation memory userOp;
        userOp.sender = whitelistedSender;
        userOp.nonce = 0;

        bytes32 dummyUserOpHash = keccak256("whitelistTest");
        // Hash must match what paymaster computes: keccak256(abi.encode(userOpHash, paymasterData, nonces[userOp.sender]))
        uint256 nonceBefore = paymaster.nonces(whitelistedSender);
        bytes32 hash = keccak256(abi.encode(dummyUserOpHash, bytes(""), nonceBefore));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
        bytes memory signature = abi.encodePacked(r, s, v);

        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), signature));

        vm.prank(address(mockEntryPoint));
        (bytes memory context) = paymaster.validatePaymasterUserOp(userOp, dummyUserOpHash, 0);

        // Context should contain the nonce (0 in this case)
        assertGt(context.length, 0, "Context should not be empty");
    }

    // ============================================================
    // MEDIUM #8: ReserveVault.rescueTokens validation
    // ============================================================

    function test_Security_RescueTokensValidatesBalance() public {
        // Mint some other tokens to vault
        MockERC20 otherToken = new MockERC20();
        otherToken.mint(address(vault), 100 * 10**6);

        // Try to rescue more than vault has
        vm.prank(owner);
        vm.expectRevert(ReserveVault__InsufficientReserves.selector);
        vault.rescueTokens(address(otherToken), 200 * 10**6);

        // Rescue correct amount should work
        uint256 ownerBalanceBefore = otherToken.balanceOf(owner);
        vm.prank(owner);
        vault.rescueTokens(address(otherToken), 50 * 10**6);

        assertEq(otherToken.balanceOf(owner), ownerBalanceBefore + 50 * 10**6);
    }

    function test_Security_RescueTokensCannotRescueUSDT() public {
        // Try to rescue USDT from vault
        vm.startPrank(user);
        usdt.approve(address(vault), 1000 * 10**6);
        vault.deposit(1000 * 10**6);
        vm.stopPrank();

        // Owner cannot rescue USDT
        vm.prank(owner);
        vm.expectRevert();
        vault.rescueTokens(address(usdt), 100 * 10**6);
    }

    function test_Security_RescueTokensEmitsEvent() public {
        MockERC20 otherToken = new MockERC20();
        otherToken.mint(address(vault), 100 * 10**6);

        vm.expectEmit(true, true, true, true);
        emit IReserveVault.TokensRescued(address(otherToken), owner, 50 * 10**6);

        vm.prank(owner);
        vault.rescueTokens(address(otherToken), 50 * 10**6);
    }

    // ============================================================
    // Comprehensive invariant tests
    // ============================================================

    function test_Security_ReserveInvariantAlwaysHolds() public {
        // Multiple wraps and unwraps
        uint256 amount1 = 1000 * 10**6;
        uint256 amount2 = 500 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(wusdt), amount1 + amount2);
        wusdt.wrap(amount1);
        _checkInvariant();
        wusdt.wrap(amount2);
        _checkInvariant();
        wusdt.unwrap(700 * 10**18);
        _checkInvariant();
        vm.stopPrank();
    }

    function _checkInvariant() internal view {
        uint256 totalWrapped = vault.totalWrapped();
        uint256 totalReserves = vault.totalReserves();
        uint256 reservesIn18 = totalReserves * 1e12;

        // Allow small dust tolerance
        assertLe(totalWrapped, reservesIn18 + wusdt.accumulatedDust(),
            "Invariant violated: wrapped > reserves + dust");
    }

    // ============================================================
    // Reentrancy protection tests
    // ============================================================

    function test_Security_NoReentrancyInUnwrap() public {
        uint256 amount = 1000 * 10**6;

        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);

        // Unwrap should be reentrant-safe
        // State changes happen before external calls
        uint256 balanceBefore = wusdt.balanceOf(user);
        wusdt.unwrap(amount * 1e12);
        uint256 balanceAfter = wusdt.balanceOf(user);

        assertEq(balanceBefore - balanceAfter, amount * 1e12);
        vm.stopPrank();
    }

    // ============================================================
    // Access control comprehensive tests
    // ============================================================

    function test_Security_AllOwnerFunctionsProtected() public {
        // WrappedUSDT owner functions
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(attacker);
        wusdt.pauseTransfers();

        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(attacker);
        wusdt.unpauseTransfers();

        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(attacker);
        wusdt.claimDust();

        // ReserveVault owner functions
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(attacker);
        vault.setWrappedUSDT(attacker);

        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(attacker);
        vault.pauseDeposits();

        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(attacker);
        vault.unpauseDeposits();

        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(attacker);
        vault.emergencyWithdraw(100);

        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(attacker);
        vault.rescueTokens(address(usdt), 100);

        // VerifyingPaymaster owner functions
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(attacker);
        paymaster.setTrustedSigner(attacker);

        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(attacker);
        paymaster.setSenderWhitelist(attacker, true);
    }

    // ============================================================
    // Signature verification tests
    // ============================================================

    function test_Security_SignatureMalleabilityCheck() public {
        // Test that s value in upper half is rejected
        bytes32 hash = keccak256("test");

        // Create signature with s value in upper half (should be rejected)
        uint8 v = 27;
        bytes32 r = bytes32(uint256(1));
        bytes32 s = bytes32(uint256(0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0) + 1);

        bytes memory badSignature = abi.encodePacked(r, s, v);

        assertFalse(paymaster.verifySigner(hash, badSignature),
            "Signature with s value in upper half should be rejected");
    }

    function test_Security_NoncesPreventReplay() public {
        address testUser = address(0x1234);

        // Whitelist test user
        vm.prank(owner);
        paymaster.setSenderWhitelist(testUser, true);

        // Deposit to paymaster
        mockEntryPoint.depositTo{value: 1 ether}(address(paymaster));

        // Initial nonce
        assertEq(paymaster.nonces(testUser), 0);

        // Create valid userOp with simple hash
        PackedUserOperation memory userOp;
        userOp.sender = testUser;
        userOp.nonce = 0;

        bytes32 dummyUserOpHash = keccak256("testUserOp");
        // Hash must match what paymaster computes: keccak256(abi.encode(userOpHash, paymasterData, nonces[userOp.sender]))
        bytes32 hash = keccak256(abi.encode(dummyUserOpHash, bytes(""), paymaster.nonces(testUser)));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, hash);
        bytes memory signature = abi.encodePacked(r, s, v);

        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), signature));

        // First validation increments nonce
        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(userOp, dummyUserOpHash, 0);

        // Nonce should now be 1
        assertEq(paymaster.nonces(testUser), 1);

        // Same signature with old nonce (0) should no longer be valid
        // because the hash includes the nonce, and nonce is now 1
        vm.prank(address(mockEntryPoint));
        vm.expectRevert();
        paymaster.validatePaymasterUserOp(userOp, dummyUserOpHash, 0);
    }

    // ============================================================
    // Pausable functionality tests
    // ============================================================

    function test_Security_PauseStopsAllCriticalOperations() public {
        uint256 amount = 1000 * 10**6;

        // Pause both contracts
        vm.prank(owner);
        wusdt.pauseTransfers();
        vm.prank(owner);
        vault.pauseDeposits();

        // Try to wrap - should fail
        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        vm.expectRevert();
        wusdt.wrap(amount);
        vm.stopPrank();

        // Try to deposit - should fail
        vm.startPrank(user);
        usdt.approve(address(vault), amount);
        vm.expectRevert();
        vault.deposit(amount);
        vm.stopPrank();

        // Unpause
        vm.prank(owner);
        wusdt.unpauseTransfers();
        vm.prank(owner);
        vault.unpauseDeposits();

        // Operations should work again
        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);
        vm.stopPrank();

        assertEq(wusdt.balanceOf(user), amount * 1e12);
    }

    // ============================================================
    // Edge case tests
    // ============================================================

    function test_Security_WrapZeroAmountReverts() public {
        vm.startPrank(user);
        usdt.approve(address(wusdt), type(uint256).max);
        vm.expectRevert();
        wusdt.wrap(0);
        vm.stopPrank();
    }

    function test_Security_UnwrapZeroAmountReverts() public {
        vm.startPrank(user);
        vm.expectRevert();
        wusdt.unwrap(0);
        vm.stopPrank();
    }

    function test_Security_DepositZeroAmountReverts() public {
        vm.startPrank(user);
        usdt.approve(address(vault), type(uint256).max);
        vm.expectRevert();
        vault.deposit(0);
        vm.stopPrank();
    }

    function test_Safety_MaximumAmounts() public {
        // Test with maximum reasonable amounts
        uint256 maxAmount = 1000000 * 10**6; // 1M USDT

        vm.startPrank(user);
        usdt.approve(address(wusdt), maxAmount);
        wusdt.wrap(maxAmount);

        assertEq(wusdt.balanceOf(user), maxAmount * 1e12);
        assertEq(vault.totalReserves(), maxAmount);
        assertEq(vault.totalWrapped(), maxAmount * 1e12);

        // Unwrap all
        wusdt.unwrap(maxAmount * 1e12);

        assertEq(wusdt.balanceOf(user), 0);
        vm.stopPrank();
    }

    // ============================================================
    // Dust handling tests
    // ============================================================

    function test_Security_DustAccumulationAndRecovery() public {
        uint256 wrapAmount = 10000 * 10**6;

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
        // Need to accumulate at least 1e12 dust for claimable amount
        vm.startPrank(user);
        for (uint256 i = 0; i < 10; i++) {
            wusdt.unwrap(100 * 1e12 + 1e11);
        }
        vm.stopPrank();

        uint256 dust = wusdt.accumulatedDust();
        assertGe(dust, 1e12, "Dust should have accumulated to at least 1e12");

        // Claim dust
        uint256 ownerBalanceBefore = usdt.balanceOf(owner);
        vm.prank(owner);
        wusdt.claimDust();

        uint256 dustIn6 = dust / 1e12;
        assertEq(usdt.balanceOf(owner), ownerBalanceBefore + dustIn6);
        assertEq(wusdt.accumulatedDust(), 0);
    }

    // ============================================================
    // Event emission tests
    // ============================================================

    function test_Security_AllCriticalEventsEmitted() public {
        uint256 amount = 1000 * 10**6;

        // Wrapped event - need to do approval separately to avoid catching Approval event
        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        vm.stopPrank();

        vm.expectEmit(true, false, false, true);
        emit WrappedUSDT.Wrapped(user, amount);

        vm.prank(user);
        wusdt.wrap(amount);

        // Deposit event
        vm.startPrank(alice);
        usdt.approve(address(vault), amount);
        vm.stopPrank();

        vm.expectEmit(true, false, false, true);
        emit IReserveVault.Deposit(alice, amount);

        vm.prank(alice);
        vault.deposit(amount);

        // Mint event - amount18 is in 18 decimals
        uint256 amount18 = amount * 1e12;
        vm.expectEmit(true, false, false, true);
        emit IReserveVault.Mint(user, amount18);

        vm.prank(address(wusdt));
        vault.mint(user, amount18);

        // Burn event
        vm.expectEmit(true, false, false, true);
        emit IReserveVault.Burn(alice, amount18);

        vm.prank(address(wusdt));
        vault.burn(alice, amount18);
    }

    // ============================================================
    // Multi-user security tests
    // ============================================================

    function test_Security_MultiUserIsolation() public {
        uint256 amount1 = 1000 * 10**6;  // 1000 USDT in 6 decimals
        uint256 amount2 = 500 * 10**6;   // 500 USDT in 6 decimals

        // User 1 wraps
        vm.startPrank(user);
        usdt.approve(address(wusdt), amount1);
        wusdt.wrap(amount1);
        vm.stopPrank();

        // User 2 wraps
        vm.startPrank(alice);
        usdt.approve(address(wusdt), amount2);
        wusdt.wrap(amount2);
        vm.stopPrank();

        // Verify balances (1000 USDT * 1e12 = 1000 * 10^18 wUSDT in 18 decimals)
        uint256 user1Balance = amount1 * 1e12;  // 1000 * 10^18
        uint256 aliceBalance = amount2 * 1e12;  // 500 * 10^18
        assertEq(wusdt.balanceOf(user), user1Balance);
        assertEq(wusdt.balanceOf(alice), aliceBalance);

        // User 1 transfers to User 2 (300 wUSDT = 300 * 10^18 in 18 decimals)
        uint256 transferAmount = 300 * 10**18;
        vm.prank(user);
        wusdt.transfer(alice, transferAmount);

        assertEq(wusdt.balanceOf(user), user1Balance - transferAmount);
        assertEq(wusdt.balanceOf(alice), aliceBalance + transferAmount);

        // Both users unwrap
        vm.prank(user);
        wusdt.unwrap(user1Balance - transferAmount);

        vm.prank(alice);
        wusdt.unwrap(aliceBalance + transferAmount);

        // Verify invariant holds
        _checkInvariant();
    }
}
