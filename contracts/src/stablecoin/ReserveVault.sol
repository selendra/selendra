// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Ownable} from "../utils/Ownable.sol";
import {Pausable} from "../utils/Pausable.sol";
import "./IReserveVault.sol";

/**
 * @title ReserveVault
 * @notice Holds USDT reserves and manages wUSDT minting/burning
 * @dev Only the associated WrappedUSDT contract can call mint/burn
 */
contract ReserveVault is Ownable, Pausable, IReserveVault {
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
        wrappedUSDT = _wrappedUSDT;
    }

    function deposit(uint256 amount) external whenNotPaused override {
        if (amount == 0) revert ReserveVault__ZeroAmount();

        bool success = usdt.transferFrom(msg.sender, address(this), amount);
        if (!success) revert ReserveVault__InsufficientReserves();

        totalReserves += amount;
        emit Deposit(msg.sender, amount);
    }

    function withdraw(uint256 amount) external whenNotPaused override {
        if (amount == 0) revert ReserveVault__ZeroAmount();
        if (totalReserves < amount) revert ReserveVault__InsufficientReserves();

        // Reserve invariant: ensure withdrawal doesn't break backing ratio
        if ((totalReserves - amount) * 1e12 < totalWrapped) revert ReserveVault__InsufficientReserves();

        totalReserves -= amount;
        
        bool success = usdt.transfer(msg.sender, amount);
        if (!success) revert ReserveVault__InsufficientReserves();

        emit Withdrawal(msg.sender, amount);
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

        bool success = usdt.transfer(to, amountReserves);
        if (!success) revert ReserveVault__InsufficientReserves();

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
        
        IERC20(token).transfer(msg.sender, amount);
    }
}
