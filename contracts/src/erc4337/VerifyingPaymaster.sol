// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

import {IEntryPoint} from "account-abstraction/interfaces/IEntryPoint.sol";
import {PackedUserOperation} from "account-abstraction/interfaces/PackedUserOperation.sol";
import {IPaymaster} from "account-abstraction/interfaces/IPaymaster.sol";
import "../utils/Ownable.sol";
import "./IVerifyingPaymaster.sol";

/**
 * @title VerifyingPaymaster
 * @notice ERC-4337 paymaster that verifies signatures from a trusted backend
 * @dev Based on eth-infinitism/account-abstraction v0.7 VerifyingPaymaster
 * @dev Implements IPaymaster for ERC-4337 compliance
 */
contract VerifyingPaymaster is Ownable, IVerifyingPaymaster, IPaymaster {
    error VerifyingPaymaster__InvalidSignature();
    error VerifyingPaymaster__InvalidNonce();
    error VerifyingPaymaster__NotWhitelistedSender();

    IEntryPoint public immutable entryPoint;
    address public trustedSigner;

    /// @dev Per-user nonces prevent transaction conflicts between concurrent users
    mapping(address => uint256) public nonces;

    mapping(address => bool) public whitelistedSenders;

    constructor(IEntryPoint _entryPoint, address _trustedSigner) {
        if (address(_entryPoint) == address(0)) revert Ownable__ZeroAddress();
        if (_trustedSigner == address(0)) revert Ownable__ZeroAddress();

        entryPoint = _entryPoint;
        trustedSigner = _trustedSigner;
    }

    receive() external payable {
        emit DepositReceived(msg.sender, msg.value);
    }

    function validatePaymasterUserOp(
        PackedUserOperation calldata userOp,
        bytes32 userOpHash,
        uint256 maxCost
    ) external returns (bytes memory context, uint256 validationData) {
        _requireFromEntryPoint();

        if (!whitelistedSenders[userOp.sender]) {
            revert VerifyingPaymaster__NotWhitelistedSender();
        }

        (bytes memory paymasterData, bytes memory signature) = abi.decode(
            userOp.paymasterAndData[20:],
            (bytes, bytes)
        );

        bytes32 hash = _hashPaymaster(userOpHash, paymasterData, nonces[userOp.sender]);
        if (!verifySigner(hash, signature)) {
            revert VerifyingPaymaster__InvalidSignature();
        }

        uint256 nonceBefore = nonces[userOp.sender];
        nonces[userOp.sender]++;

        context = abi.encodePacked(nonceBefore);
        uint256 currentDeposit = entryPoint.balanceOf(address(this));
        if (currentDeposit < maxCost) {
            revert VerifyingPaymaster__InsufficientDeposit();
        }

        // validationData: sigFailed=0 (success), validUntil=0 (indefinite), validAfter=0 (immediate)
        validationData = 0;
        return (context, validationData);
    }

    function postOp(
        PostOpMode mode,
        bytes calldata context,
        uint256 actualGasCost,
        uint256 actualUserOpFeePerGas
    ) external {
        _requireFromEntryPoint();
        // No post-op processing needed for simple paymaster
        // mode indicates whether userOp succeeded or reverted, but we pay regardless
    }

    function _hashPaymaster(
        bytes32 userOpHash,
        bytes memory paymasterData,
        uint256 _nonce
    ) internal pure returns (bytes32) {
        return keccak256(abi.encode(userOpHash, paymasterData, _nonce));
    }

    function verifySigner(
        bytes32 hash,
        bytes memory signature
    ) public view returns (bool) {
        if (signature.length != 65) {
            return false;
        }

        bytes32 r;
        bytes32 s;
        uint8 v;

        assembly {
            r := mload(add(add(signature, 0x20), 0))
            s := mload(add(add(signature, 0x20), 0x20))
            v := byte(0, mload(add(add(signature, 0x20), 0x40)))
        }

        if (v < 27) v += 27;
        if (v > 28) return false;
        if (uint256(s) > 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0) return false;

        address signer = ecrecover(hash, v, r, s);
        return signer == trustedSigner;
    }

    function deposit() external payable override {
        entryPoint.depositTo{value: msg.value}(address(this));
        emit DepositReceived(msg.sender, msg.value);
    }

    function withdraw(address payable to, uint256 amount) external override onlyOwner {
        entryPoint.withdrawTo(to, amount);
        emit Withdrawn(to, amount);
    }

    function setTrustedSigner(address newSigner) external override onlyOwner {
        if (newSigner == address(0)) revert Ownable__ZeroAddress();
        address oldSigner = trustedSigner;
        trustedSigner = newSigner;
        emit TrustedSignerSet(oldSigner, newSigner);
    }

    function setSenderWhitelist(address sender, bool status) external override onlyOwner {
        whitelistedSenders[sender] = status;
        emit SenderWhitelisted(sender, status);
    }

    function getDeposit() external view override returns (uint256) {
        return entryPoint.balanceOf(address(this));
    }

    function _requireFromEntryPoint() internal view {
        if (msg.sender != address(entryPoint)) {
            revert VerifyingPaymaster__InvalidSignature();
        }
    }
}
