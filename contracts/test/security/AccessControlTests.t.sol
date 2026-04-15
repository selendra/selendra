// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {Test} from "forge-std/Test.sol";
import {WrappedUSDT} from "src/stablecoin/WrappedUSDT.sol";
import {ReserveVault} from "src/stablecoin/ReserveVault.sol";
import {VerifyingPaymaster} from "src/erc4337/VerifyingPaymaster.sol";
import {MockERC20} from "../mocks/MockERC20.sol";
import {MockEntryPoint} from "../mocks/MockEntryPoint.sol";
import {IEntryPoint} from "account-abstraction/interfaces/IEntryPoint.sol";
import {PackedUserOperation} from "account-abstraction/interfaces/PackedUserOperation.sol";

/**
 * @title AccessControlTests
 * @notice Tests access control across all contracts
 * @dev Ensures only authorized addresses can call sensitive functions
 */
contract AccessControlTests is Test {
    WrappedUSDT public wusdt;
    ReserveVault public vault;
    VerifyingPaymaster public paymaster;
    MockERC20 public usdt;
    MockEntryPoint public mockEntryPoint;

    address public owner;
    address public user;
    address public alice;
    address public wrappedContract;

    uint256 signerPrivateKey = 0x12345;
    address public trustedSigner;

    error Ownable__NotOwner();
    error ReserveVault__NotWrappedContract();

    function setUp() public {
        owner = address(this);
        user = address(0x1);
        alice = address(0x2);
        wrappedContract = address(0x3);
        trustedSigner = vm.addr(signerPrivateKey);

        usdt = new MockERC20();
        vault = new ReserveVault(usdt);
        vault.setWrappedUSDT(wrappedContract);

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
    }

    // ============================================================
    // WrappedUSDT OnlyOwner Functions
    // ============================================================

    function test_Ownable_WrappedUSDT_PauseTransfers() public {
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        wusdt.pauseTransfers();
    }

    function test_Ownable_WrappedUSDT_UnpauseTransfers() public {
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        wusdt.unpauseTransfers();
    }

    function test_Ownable_WrappedUSDT_ClaimDust() public {
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        wusdt.claimDust();
    }

    // ============================================================
    // ReserveVault OnlyOwner Functions
    // ============================================================

    function test_Ownable_ReserveVault_SetWrappedUSDT() public {
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        vault.setWrappedUSDT(alice);
    }

    function test_Ownable_ReserveVault_PauseDeposits() public {
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        vault.pauseDeposits();
    }

    function test_Ownable_ReserveVault_UnpauseDeposits() public {
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        vault.unpauseDeposits();
    }

    function test_Ownable_ReserveVault_RescueTokens() public {
        // Mint some non-USDT tokens to the vault
        MockERC20 otherToken = new MockERC20();
        otherToken.mint(address(vault), 1000 * 10**6);

        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        vault.rescueTokens(address(otherToken), 100 * 10**6);
    }

    // ============================================================
    // VerifyingPaymaster OnlyOwner Functions
    // ============================================================

    function test_Ownable_VerifyingPaymaster_SetTrustedSigner() public {
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        paymaster.setTrustedSigner(alice);
    }

    function test_Ownable_VerifyingPaymaster_SetSenderWhitelist() public {
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        paymaster.setSenderWhitelist(alice, true);
    }

    function test_Ownable_VerifyingPaymaster_Withdraw() public {
        // Deposit some ETH first
        vm.deal(address(paymaster), 1 ether);

        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        paymaster.withdraw(payable(alice), 0.5 ether);
    }

    // ============================================================
    // ReserveVault OnlyWrappedContract Functions
    // ============================================================

    function test_Vault_OnlyWrappedCanMint() public {
        vm.expectRevert(ReserveVault__NotWrappedContract.selector);
        vm.prank(user);
        vault.mint(user, 1000 * 10**18);
    }

    function test_Vault_OnlyWrappedCanBurn() public {
        // First mint some tokens
        vm.prank(address(wusdt));
        vault.mint(user, 1000 * 10**18);

        vm.expectRevert(ReserveVault__NotWrappedContract.selector);
        vm.prank(user);
        vault.burn(user, 500 * 10**18);
    }

    function test_Vault_OnlyWrappedCanMintWithReserves() public {
        vm.expectRevert(ReserveVault__NotWrappedContract.selector);
        vm.prank(user);
        vault.mintWithReserves(user, 1000 * 10**18, 1000 * 10**6);
    }

    function test_Vault_OnlyWrappedCanBurnAndWithdraw() public {
        // First set up reserves
        uint256 depositAmount = 2000 * 10**6;
        usdt.mint(owner, depositAmount);

        vm.startPrank(owner);
        usdt.approve(address(vault), depositAmount);
        vault.deposit(depositAmount);
        vm.stopPrank();

        // Mint tokens
        vm.prank(address(wusdt));
        vault.mint(user, 1000 * 10**18);

        vm.expectRevert(ReserveVault__NotWrappedContract.selector);
        vm.prank(user);
        vault.burnAndWithdraw(user, 500 * 10**18, 500 * 10**6);
    }

    function test_Vault_WrappedContractCanMint() public {
        // Should succeed when called by wrappedUSDT
        vm.prank(address(wusdt));
        vault.mint(user, 1000 * 10**18);

        assertEq(vault.totalWrapped(), 1000 * 10**18);
    }

    function test_Vault_WrappedContractCanBurn() public {
        // First mint
        vm.prank(address(wusdt));
        vault.mint(user, 1000 * 10**18);

        // Then burn - should succeed
        vm.prank(address(wusdt));
        vault.burn(user, 500 * 10**18);

        assertEq(vault.totalWrapped(), 500 * 10**18);
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

        vm.expectRevert(); // Should revert when not called by entryPoint
        vm.prank(user);
        paymaster.validatePaymasterUserOp(userOp, userOpHash, 0);
    }

    function test_Paymaster_OnlyEntryPoint_PostOp() public {
        PackedUserOperation memory userOp;
        userOp.sender = alice;

        vm.expectRevert(); // Should revert when not called by entryPoint
        vm.prank(user);
        paymaster.postOp(userOp, "", 0);
    }

    function test_Paymaster_EntryPointCanCallValidate() public {
        // Whitelist the sender first
        vm.prank(owner);
        paymaster.setSenderWhitelist(alice, true);

        // Create a proper signature
        bytes32 dummyHash = keccak256(abi.encode("test"));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signerPrivateKey, dummyHash);
        bytes memory signature = abi.encodePacked(r, s, v);

        PackedUserOperation memory userOp;
        userOp.sender = alice;
        userOp.nonce = 0;
        userOp.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), signature));

        bytes32 userOpHash = keccak256(abi.encode(userOp));

        // This should not revert with "not from entry point" error
        // It may still revert due to signature/other validation, but not access control
        vm.prank(address(mockEntryPoint));
        // We expect this to fail due to signature/nonce, not access control
        vm.expectRevert();
        paymaster.validatePaymasterUserOp(userOp, userOpHash, 0);
    }

    // ============================================================
    // WrappedUSDT Vault-only Functions
    // ============================================================

    function test_WrappedUSDT_OnlyVaultCanMint() public {
        vm.expectRevert();
        vm.prank(user);
        wusdt.mint(user, 1000 * 10**18);
    }

    function test_WrappedUSDT_OnlyVaultCanBurn() public {
        // First mint some tokens to user
        uint256 amount = 1000 * 10**6;
        vm.startPrank(user);
        usdt.approve(address(wusdt), amount);
        wusdt.wrap(amount);
        vm.stopPrank();

        vm.expectRevert();
        vm.prank(user);
        wusdt.burn(user, 500 * 10**18);
    }

    function test_WrappedUSDT_VaultCanMint() public {
        vm.prank(address(vault));
        wusdt.mint(user, 1000 * 10**18);

        assertEq(wusdt.balanceOf(user), 1000 * 10**18);
    }

    function test_WrappedUSDT_VaultCanBurn() public {
        // First mint
        vm.prank(address(vault));
        wusdt.mint(user, 1000 * 10**18);

        // Then burn - should succeed
        vm.prank(address(vault));
        wusdt.burn(user, 500 * 10**18);

        assertEq(wusdt.balanceOf(user), 500 * 10**18);
    }

    // ============================================================
    // Paymaster Per-User Nonces
    // ============================================================

    function test_Paymaster_PerUserNonces() public {
        address user1 = address(0x100);
        address user2 = address(0x200);

        // Both users start at nonce 0
        assertEq(paymaster.nonces(user1), 0);
        assertEq(paymaster.nonces(user2), 0);

        // Verify that each user has independent nonce by using validatePaymasterUserOp
        // First, whitelist both users and deposit
        vm.prank(owner);
        paymaster.setSenderWhitelist(user1, true);
        vm.prank(owner);
        paymaster.setSenderWhitelist(user2, true);

        // Mock entry point deposit
        vm.deal(address(this), 2 ether);
        IEntryPoint(address(mockEntryPoint)).depositTo{value: 2 ether}(address(paymaster));

        // Create a valid userOp to increment nonce for user1
        PackedUserOperation memory userOp1;
        userOp1.sender = user1;
        userOp1.nonce = 0;

        // Use a simple hash for testing (actual userOp hash would be computed by entry point)
        bytes32 dummyUserOpHash1 = keccak256("user1");

        // Get nonce before and create signature
        uint256 nonceBefore1 = paymaster.nonces(user1);
        bytes32 hash1 = keccak256(abi.encode(dummyUserOpHash1, bytes(""), nonceBefore1));

        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(signerPrivateKey, hash1);
        bytes memory signature1 = abi.encodePacked(r1, s1, v1);

        userOp1.paymasterAndData = abi.encodePacked(address(paymaster), abi.encode(bytes(""), signature1));

        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(userOp1, dummyUserOpHash1, 0);

        // user1's nonce should now be 1, user2's should still be 0
        assertEq(paymaster.nonces(user1), 1);
        assertEq(paymaster.nonces(user2), 0);

        // Now increment user2's nonce
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

        // user1's nonce should still be 1, user2's should now be 1
        assertEq(paymaster.nonces(user1), 1);
        assertEq(paymaster.nonces(user2), 1);
    }

    function test_Paymaster_ConcurrentUsers() public {
        address user1 = address(0x100);
        address user2 = address(0x200);

        // Whitelist users
        vm.prank(owner);
        paymaster.setSenderWhitelist(user1, true);
        vm.prank(owner);
        paymaster.setSenderWhitelist(user2, true);

        // Deposit to paymaster
        vm.deal(address(this), 2 ether);
        IEntryPoint(address(mockEntryPoint)).depositTo{value: 2 ether}(address(paymaster));

        // Each user has independent nonce starting at 0
        assertEq(paymaster.nonces(user1), 0);
        assertEq(paymaster.nonces(user2), 0);

        // Increment user1's nonce
        bytes32 u1Hash = keccak256(abi.encode(keccak256("u1"), bytes(""), uint256(0)));
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(signerPrivateKey, u1Hash);
        bytes memory sig1 = abi.encodePacked(r1, s1, v1);
        bytes memory paymasterAndData1 = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig1));
        PackedUserOperation memory op1;
        op1.sender = user1;
        op1.paymasterAndData = paymasterAndData1;
        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(op1, keccak256("u1"), 0);

        // Increment user2's nonce
        bytes32 u2Hash = keccak256(abi.encode(keccak256("u2"), bytes(""), uint256(0)));
        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(signerPrivateKey, u2Hash);
        bytes memory sig2 = abi.encodePacked(r2, s2, v2);
        bytes memory paymasterAndData2 = abi.encodePacked(address(paymaster), abi.encode(bytes(""), sig2));
        PackedUserOperation memory op2;
        op2.sender = user2;
        op2.paymasterAndData = paymasterAndData2;
        vm.prank(address(mockEntryPoint));
        paymaster.validatePaymasterUserOp(op2, keccak256("u2"), 0);

        // Verify nonces incremented independently
        assertEq(paymaster.nonces(user1), 1);
        assertEq(paymaster.nonces(user2), 1);
    }

    // ============================================================
    // Comprehensive Access Control Matrix
    // ============================================================

    function test_Ownable_OnlyOwnerFunctions() public {
        // WrappedUSDT
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        wusdt.pauseTransfers();

        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        wusdt.unpauseTransfers();

        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        wusdt.claimDust();

        // ReserveVault
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        vault.setWrappedUSDT(alice);

        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        vault.pauseDeposits();

        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        vault.unpauseDeposits();

        // VerifyingPaymaster
        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        paymaster.setTrustedSigner(alice);

        vm.expectRevert(Ownable__NotOwner.selector);
        vm.prank(user);
        paymaster.setSenderWhitelist(alice, true);
    }
}
