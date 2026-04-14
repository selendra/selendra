// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

/**
 * @title IReserveVault
 * @notice Interface for the reserve vault that backs wUSDT
 */
interface IReserveVault {
    error ReserveVault__NotVault();
    error ReserveVault__InsufficientReserves();
    error ReserveVault__ZeroAmount();
    error ReserveVault__ZeroAddress();

    event Deposit(address indexed user, uint256 amount);
    event Withdrawal(address indexed user, uint256 amount);
    event Mint(address indexed to, uint256 amount);
    event Burn(address indexed from, uint256 amount);

    function deposit(uint256 amount) external;

    function withdraw(uint256 amount) external;

    function mint(address to, uint256 amount) external;

    function burn(address from, uint256 amount) external;

    function mintWithReserves(address to, uint256 amountWrapped, uint256 amountReserves) external;

    function burnAndWithdraw(address to, uint256 amountWrapped, uint256 amountReserves) external;

    function totalReserves() external view returns (uint256);

    function totalWrapped() external view returns (uint256);
}
