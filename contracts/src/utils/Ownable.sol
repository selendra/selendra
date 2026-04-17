// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

/**
 * @title Ownable
 * @notice Simple ownable contract with owner management
 */
abstract contract Ownable {
    error Ownable__NotOwner();
    error Ownable__ZeroAddress();

    address private _owner;

    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    constructor() {
        _transferOwnership(msg.sender);
    }

    modifier onlyOwner() {
        if (owner() != msg.sender) revert Ownable__NotOwner();
        _;
    }

    function owner() public view virtual returns (address) {
        return _owner;
    }

    function renounceOwnership() public virtual onlyOwner {
        _transferOwnership(address(0));
    }

    function transferOwnership(address newOwner) public virtual onlyOwner {
        if (newOwner == address(0)) revert Ownable__ZeroAddress();
        _transferOwnership(newOwner);
    }

    function _transferOwnership(address newOwner) internal virtual {
        address oldOwner = _owner;
        _owner = newOwner;
        emit OwnershipTransferred(oldOwner, newOwner);
    }
}
