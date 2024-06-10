const hre = require("hardhat");

const { ethers } = hre;

async function main() {
    const [deployer, user] = await ethers.getSigners();

    console.log("Deployer address:", deployer.address);

    const simpleSwap = await ethers.getContractFactory("XCDPCore");
    const simpleSwapAddress = "0x870FeAB9128411642264CeF12A229ee882eD3f46";
    const simpleSwapContract = simpleSwap.attach(simpleSwapAddress);

    
    let tx = await simpleSwapContract.connect(deployer)._l1xSend(
        "helloWorld",
        // Destination Address => BSC Testnet
        "0x870FeAB9128411642264CeF12A229ee882eD3f46",
        "bscTestnet"
    );

    let receipt = await tx.wait(); // Wait for the transaction to be mined
    console.log("Event topic:", JSON.stringify(receipt));

    // Get the logs from the receipt
    let logs = receipt.logs;

    // Assuming there is only one log emitted
    if (logs.length > 0) {
        console.log("Event topic:", logs[0].topics[0]);
    } else {
        console.log("No logs emitted.");
    }

    console.log("send message tx hash:", tx.hash);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });