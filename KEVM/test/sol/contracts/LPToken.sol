pragma solidity ^0.4.11;

import 'zeppelin-solidity/contracts/token/MintableToken.sol';

contract LPToken is MintableToken {
  string public name = "Frontier LP Token";
  string public symbol = "FPLP";
  uint256 public decimals = 18;
}
