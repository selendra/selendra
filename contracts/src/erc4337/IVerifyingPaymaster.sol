// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

/**
 * @title IVerifyingPaymaster
 * @notice Interface for the verifying paymaster
 */
interface IVerifyingPaymaster {
    error VerifyingPaymaster__NotTrustedSigner();
    error VerifyingPaymaster__NotWhitelisted();
    error VerifyingPaymaster__InsufficientDeposit();
    error VerifyingPaymaster__WithdrawFailed();

    event DepositReceived(address indexed sender, uint256 amount);
    event Withdrawn(address indexed to, uint256 amount);
    event TrustedSignerSet(address indexed oldSigner, address indexed newSigner);
    event SenderWhitelisted(address indexed sender, bool status);

    function verifySigner(
        bytes32 hash,
        bytes calldata signature
    ) external view returns (bool);

    function deposit() external payable;

    function withdraw(address payable to, uint256 amount) external;

    function setTrustedSigner(address newSigner) external;

    function setSenderWhitelist(address sender, bool status) external;

    function getDeposit() external view returns (uint256);
}
