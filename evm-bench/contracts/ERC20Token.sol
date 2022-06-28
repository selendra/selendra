// SPDX-License-Identifier: GPL-3.0-or-later

pragma solidity >=0.7.0 <0.9.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract ERC20Token is ERC20 {
    constructor() ERC20("Test Token", "tToken") {
        _mint(msg.sender, 1000e18);
    }

     function approve_many(uint n, uint256 amount) public returns (bool) {
        for (uint i = 0; i < n; i++) {
            approve(address(bytes20(keccak256(abi.encode(i)))), amount);
        }
        return true;
    }

    function transfer_many(uint n, uint256 amount) public returns (bool) {
        for (uint i = 0; i < n; i++) {
            transfer(address(bytes20(keccak256(abi.encode(i)))), amount);
        }
        return true;
    }
}
