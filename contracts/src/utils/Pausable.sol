// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.28;

/**
 * @title Pausable
 * @notice Emergency pause mechanism
 */
abstract contract Pausable {
    error Pausable__Paused();
    error Pausable__NotPaused();

    bool private _paused;

    event Paused(address account);
    event Unpaused(address account);

    constructor() {
        _paused = false;
    }

    modifier whenNotPaused() {
        if (paused()) revert Pausable__Paused();
        _;
    }

    modifier whenPaused() {
        if (!paused()) revert Pausable__NotPaused();
        _;
    }

    function paused() public view virtual returns (bool) {
        return _paused;
    }

    function _pause() internal virtual {
        _paused = true;
        emit Paused(msg.sender);
    }

    function _unpause() internal virtual {
        _paused = false;
        emit Unpaused(msg.sender);
    }
}
