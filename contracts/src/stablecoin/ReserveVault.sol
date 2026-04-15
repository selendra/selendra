// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {Ownable} from "../utils/Ownable.sol";
import {Pausable} from "../utils/Pausable.sol";
import "./IReserveVault.sol";

/**
 * @title ReserveVault
 * @notice Holds USDT reserves and manages wUSDT minting/burning
 * @dev Only the associated WrappedUSDT contract can call mint/burn
 */
contract ReserveVault is Ownable, Pausable, IReserveVault {
    using SafeERC20 for IERC20;

    IERC20 public immutable usdt;
    address public wrappedUSDT;

    uint256 public totalReserves;
    uint256 public totalWrapped;

    error ReserveVault__NotWrappedContract();

    constructor(IERC20 _usdt) {
        if (address(_usdt) == address(0)) revert Ownable__ZeroAddress();
        usdt = _usdt;
    }

    modifier onlyWrappedContract() {
        if (msg.sender != wrappedUSDT) revert ReserveVault__NotWrappedContract();
        _;
    }

    function setWrappedUSDT(address _wrappedUSDT) external onlyOwner {
        if (_wrappedUSDT == address(0)) revert Ownable__ZeroAddress();
        address oldAddress = wrappedUSDT;
        wrappedUSDT = _wrappedUSDT;
        emit WrappedUSDTSet(oldAddress, _wrappedUSDT);
    }

    function deposit(uint256 amount) external whenNotPaused override {
        if (amount == 0) revert ReserveVault__ZeroAmount();

        uint256 balanceBefore = usdt.balanceOf(address(this));
        usdt.safeTransferFrom(msg.sender, address(this), amount);
        uint256 balanceAfter = usdt.balanceOf(address(this));

        // Verify actual amount received (protects against fee-on-transfer tokens)
        uint256 actualAmount = balanceAfter - balanceBefore;
        if (actualAmount != amount) revert ReserveVault__InvalidAmount();

        totalReserves += amount;
        emit Deposit(msg.sender, amount);
    }

    /// @notice Withdraw function removed - CRITICAL SECURITY FIX
    /// @dev Users must withdraw via WrappedUSDT.unwrap() which calls burnAndWithdraw()
    /// Direct withdrawals were allowing anyone to withdraw anyone else's funds

    /// @notice Emergency withdraw - owner only, for emergency situations
    function emergencyWithdraw(uint256 amount) external onlyOwner whenNotPaused {
        if (amount == 0) revert ReserveVault__ZeroAmount();
        if (totalReserves < amount) revert ReserveVault__InsufficientReserves();

        // Reserve invariant: ensure withdrawal doesn't break backing ratio
        if ((totalReserves - amount) * 1e12 < totalWrapped) revert ReserveVault__InsufficientReserves();

        totalReserves -= amount;

        usdt.safeTransfer(owner(), amount);

        emit EmergencyWithdraw(owner(), amount);
    }

    /// @notice Withdraw excess reserves (dust) - owner only
    /// @dev Allows withdrawing reserves that exceed the minimum required to back wrapped tokens
    function withdrawExcessReserves(uint256 amount) external onlyOwner whenNotPaused {
        if (amount == 0) revert ReserveVault__ZeroAmount();
        if (totalReserves < amount) revert ReserveVault__InsufficientReserves();

        // Must maintain enough reserves to back all wrapped tokens
        uint256 minRequiredReserves = (totalWrapped + 1e12 - 1) / 1e12; // Round up
        if (totalReserves - amount < minRequiredReserves) revert ReserveVault__InsufficientReserves();

        totalReserves -= amount;

        usdt.safeTransfer(msg.sender, amount);

        emit Withdrawal(msg.sender, amount);
    }

    /// @notice Claim truncation dust — callable only by wrappedUSDT contract
    /// @dev Dust accumulates from 18→6 decimal conversion in WrappedUSDT
    function claimDustReserves(uint256 amount) external onlyWrappedContract whenNotPaused {
        if (amount == 0) return;
        if (totalReserves < amount) revert ReserveVault__InsufficientReserves();

        // Must maintain backing for all wrapped tokens
        uint256 minRequired = (totalWrapped + 1e12 - 1) / 1e12;
        if (totalReserves - amount < minRequired) revert ReserveVault__InsufficientReserves();

        totalReserves -= amount;
        // Transfer to the wrappedUSDT contract's owner
        usdt.safeTransfer(Ownable(wrappedUSDT).owner(), amount);
    }

    function mint(address to, uint256 amount) external onlyWrappedContract whenNotPaused override {
        if (amount == 0) revert ReserveVault__ZeroAmount();
        if (to == address(0)) revert ReserveVault__ZeroAddress();

        totalWrapped += amount;
        emit Mint(to, amount);
    }

    function mintWithReserves(address to, uint256 amountWrapped, uint256 amountReserves) external onlyWrappedContract whenNotPaused {
        if (amountWrapped == 0) revert ReserveVault__ZeroAmount();
        if (amountReserves == 0) revert ReserveVault__ZeroAmount();
        if (to == address(0)) revert ReserveVault__ZeroAddress();

        totalWrapped += amountWrapped;
        totalReserves += amountReserves;
        emit Mint(to, amountWrapped);
    }

    function burnAndWithdraw(address to, uint256 amountWrapped, uint256 amountReserves) external onlyWrappedContract whenNotPaused {
        if (amountWrapped == 0) revert ReserveVault__ZeroAmount();
        if (amountReserves == 0) revert ReserveVault__ZeroAmount();
        if (totalReserves < amountReserves) revert ReserveVault__InsufficientReserves();

        totalWrapped -= amountWrapped;
        totalReserves -= amountReserves;

        // Reserve invariant: wrapped supply must not exceed reserves (accounting for 1e12 decimal diff)
        // This check prevents undercollateralization
        if (totalReserves * 1e12 < totalWrapped) revert ReserveVault__InsufficientReserves();

        usdt.safeTransfer(to, amountReserves);

        emit Burn(to, amountWrapped);
    }

    function burn(address from, uint256 amount) external onlyWrappedContract whenNotPaused override {
        if (amount == 0) revert ReserveVault__ZeroAmount();

        totalWrapped -= amount;
        emit Burn(from, amount);
    }

    function pauseDeposits() external onlyOwner {
        _pause();
    }

    function unpauseDeposits() external onlyOwner {
        _unpause();
    }

    function rescueTokens(address token, uint256 amount) external onlyOwner {
        if (token == address(usdt)) revert ReserveVault__ZeroAddress();
        if (amount == 0) revert ReserveVault__ZeroAmount();

        // Check contract has sufficient balance
        if (IERC20(token).balanceOf(address(this)) < amount) revert ReserveVault__InsufficientReserves();

        IERC20(token).safeTransfer(msg.sender, amount);
        emit TokensRescued(token, msg.sender, amount);
    }
}
