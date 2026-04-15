// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Ownable} from "../utils/Ownable.sol";
import {Pausable} from "../utils/Pausable.sol";
import {IReserveVault} from "./IReserveVault.sol";

/**
 * @title WrappedUSDT
 * @notice 1:1 wrapped representation of USDT for on-chain operations
 * @dev Fully backed by USDT reserves in ReserveVault
 */
contract WrappedUSDT is ERC20, Ownable, Pausable {
    using SafeERC20 for IERC20;

    IERC20 public immutable usdt;
    IReserveVault public vault;

    /// @notice Accumulated dust from 18→6 decimal truncation (in 18-decimal units)
    uint256 public accumulatedDust;

    error WrappedUSDT__NotVault();
    error WrappedUSDT__TransferFailed();
    error WrappedUSDT__MintFailed();
    error WrappedUSDT__BurnFailed();
    error WrappedUSDT__ZeroAmount();
    error WrappedUSDT__InvalidVault();

    event Wrapped(address indexed user, uint256 amount);
    event Unwrapped(address indexed user, uint256 amount);
    event DustClaimed(address indexed owner, uint256 amount);

    constructor(IERC20 _usdt, IReserveVault _vault) ERC20("Wrapped USDT", "wUSDT") {
        if (address(_usdt) == address(0)) revert Ownable__ZeroAddress();
        if (address(_vault) == address(0)) revert Ownable__ZeroAddress();

        // Validate vault uses same USDT address
        if (address(_vault.usdt()) != address(_usdt)) revert WrappedUSDT__InvalidVault();

        usdt = _usdt;
        vault = _vault;
    }

    function decimals() public pure override returns (uint8) {
        return 18;
    }

    function wrap(uint256 amount) external whenNotPaused {
        if (amount == 0) revert WrappedUSDT__ZeroAmount();

        uint256 amount18 = _convertTo18(amount);
        uint256 vaultBalanceBefore = usdt.balanceOf(address(vault));

        usdt.safeTransferFrom(msg.sender, address(vault), amount);

        // Verify vault actually received the USDT (protects against fee-on-transfer tokens)
        uint256 vaultBalanceAfter = usdt.balanceOf(address(vault));
        if (vaultBalanceAfter - vaultBalanceBefore != amount) revert WrappedUSDT__TransferFailed();

        vault.mintWithReserves(msg.sender, amount18, amount);
        _mint(msg.sender, amount18);

        emit Wrapped(msg.sender, amount);
    }

    function unwrap(uint256 amount) external whenNotPaused {
        if (amount == 0) revert WrappedUSDT__ZeroAmount();

        uint256 amount6 = _convertTo6(amount);

        _burn(msg.sender, amount);
        vault.burnAndWithdraw(msg.sender, amount, amount6);

        emit Unwrapped(msg.sender, amount6);
    }

    function mint(address to, uint256 amount) external {
        if (msg.sender != address(vault)) revert WrappedUSDT__NotVault();
        _mint(to, amount);
    }

    function burn(address from, uint256 amount) external {
        if (msg.sender != address(vault)) revert WrappedUSDT__NotVault();
        _burn(from, amount);
    }

    function pauseTransfers() external onlyOwner {
        _pause();
    }

    function unpauseTransfers() external onlyOwner {
        _unpause();
    }

    /// @notice Withdraw accumulated truncation dust to owner (prevent loss over time)
    function claimDust() external onlyOwner {
        uint256 dust = accumulatedDust;
        if (dust == 0) return;

        uint256 dustIn6 = dust / 1e12;
        if (dustIn6 == 0) return;

        accumulatedDust = 0;

        // Use vault's claimDustReserves — callable by wrappedUSDT contract
        vault.claimDustReserves(dustIn6);

        emit DustClaimed(owner(), dustIn6);
    }

    function _convertTo18(uint256 amount6) internal pure returns (uint256) {
        return amount6 * 1e12;
    }

    function _convertTo6(uint256 amount18) internal returns (uint256) {
        uint256 amount6 = amount18 / 1e12;
        uint256 remainder = amount18 - (amount6 * 1e12);
        if (remainder > 0) {
            accumulatedDust += remainder;
        }
        return amount6;
    }

    function _update(address from, address to, uint256 value) internal override whenNotPaused {
        super._update(from, to, value);
    }
}
