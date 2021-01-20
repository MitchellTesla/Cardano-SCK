pragma solidity ^0.4.11;

import 'zeppelin-solidity/contracts/crowdsale/RefundableCrowdsale.sol';
import './LPToken.sol';

contract Syndicate is Crowdsale {

  function Syndicate(uint256 _startBlock, uint256 _endBlock, uint256 _rate, address _wallet) Crowdsale(_startBlock, _endBlock, _rate, _wallet) {
  }

  // creates the token to be sold.
  // override this method to have crowdsale of a specific MintableToken token.
  function createTokenContract() internal returns (MintableToken) {
    return new LPToken();
  }

