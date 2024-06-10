 // deploy.js
 const hre = require("hardhat");

 async function main() {
     const [deployer] = await hre.ethers.getSigners();
 
     console.log(
         "Deploying contracts with the account:",
         deployer.address
     );
 
     // getBalance is a method on the provider, not the signer
     console.log("Account balance:", (await deployer.provider.getBalance(deployer.address)).toString());
 
     const XCDP = await hre.ethers.getContractFactory("XCDPCore");
     const XCDPContract = await XCDP.deploy([deployer.address]);
 
     await XCDPContract.waitForDeployment();
 
     console.log("XCDP contract deployed to:", await XCDPContract.getAddress());
 }
 
 main()
     .then(() => process.exit(0))
     .catch((error) => {
         console.error(error);
         process.exit(1);
 });