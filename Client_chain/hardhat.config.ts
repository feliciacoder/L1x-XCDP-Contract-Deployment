require("@nomicfoundation/hardhat-toolbox");

// Example Only. Please use MPC or env
const PRIVATE_KEY = "xx";

module.exports = {
  solidity: "0.8.24",
  networks: {
    chain: {
      url: "rpc",
      accounts: [PRIVATE_KEY] 
    }
  }
};
