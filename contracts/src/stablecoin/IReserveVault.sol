// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/**
 * @title IReserveVault
 * @notice Interface for the reserve vault that backs wUSDT
 */
interface IReserveVault {
    error ReserveVault__NotVault();
    error ReserveVault__InsufficientReserves();
    error ReserveVault__ZeroAmount();
    error ReserveVault__ZeroAddress();
    error ReserveVault__InvalidAmount();

    event Deposit(address indexed user, uint256 amount);
    event Withdrawal(address indexed user, uint256 amount);
    event Mint(address indexed to, uint256 amount);
    event Burn(address indexed from, uint256 amount);
    event WrappedUSDTSet(address indexed oldAddress, address indexed newAddress);
    event TokensRescued(address indexed token, address indexed to, uint256 amount);
    event EmergencyWithdraw(address indexed to, uint256 amount);

    function deposit(uint256 amount) external;

    function mint(address to, uint256 amount) external;

    function burn(address from, uint256 amount) external;

    function mintWithReserves(address to, uint256 amountWrapped, uint256 amountReserves) external;

    function burnAndWithdraw(address to, uint256 amountWrapped, uint256 amountReserves) external;

    function totalReserves() external view returns (uint256);

    function totalWrapped() external view returns (uint256);

    function usdt() external view returns (IERC20);

    function withdrawExcessReserves(uint256 amount) external;
    function emergencyWithdraw(uint256 amount) external;
    function claimDustReserves(uint256 amount) external;
    function setWrappedUSDT(address _wrappedUSDT) external;
    function rescueTokens(address token, uint256 amount) external;
}
