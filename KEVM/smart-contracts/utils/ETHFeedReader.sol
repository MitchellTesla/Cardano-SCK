pragma solidity ^0.6.6;

interface OracleSecurityModule {
    function peek() external view returns (bytes32, bool);

    function peep() external view returns (bytes32, bool);

    function bud(address) external view returns (uint256);

    function read() external view returns (bytes32);
}

contract ETHFeedReader {

    address public owner;

    // KOVAN PIP_ETH address
    OracleSecurityModule public constant osm = OracleSecurityModule(0x75dD74e8afE8110C8320eD397CcCff3B8134d981);

    constructor() public {
        owner = msg.sender;
    }

    //returns the future feed value
    function getPrice() view external returns (uint256) {
        (bytes32 val, bool has) = osm.peep();
        uint256 result = uint256(val);
        return result;
    }
}


