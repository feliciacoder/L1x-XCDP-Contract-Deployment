// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

    /* XCDP Core to implement the interface.*/
contract XCDPCore {
    address public xtalkNodeAddress; //Provided in endpoint
    mapping(bytes32 => XCDPReceiveMessage) public XCDPData;
    
    /* Sending message instruction. You can make this
    compatible with Destination or if you want to transform
    the message on X-Talk you can send raw instructions and
    prepare the payload on X-Talk Contract*/
    struct XCDPSendMessage {
        string message;
    }
    
    /* Same with Receiving the message where you will be
    able to provide any struct based on how you want to
    receive it based on the transformed payload */
    struct XCDPReceiveMessage {
        string message;
    }
    
    /* This is the encoded format of the paylaod to be 
    sent. This will be sent to X-Talk Contract. Provide 
    destination details here or add it into X-Talk if you
    want to run logic and decide where to send the paylaod */
    event XTalkMessageInitiated(
        bytes message,
        string destinationNetwork,
        address destinationSmartContractAddress
    );

    /* This is the function to be called when you want to 
    send the message. The message is encoded into bytes 
    before emitting */
    function _l1xSend(
        string memory message,
        address destinationSmartContractAddress, 
        string memory destinationNetwork
    ) external {

        XCDPSendMessage memory myMessage = XCDPSendMessage({
            message: message
        });

        // Convert the struct to bytes
        bytes memory messageBytes = abi.encode(myMessage);
        emit XTalkMessageInitiated(messageBytes, destinationNetwork, destinationSmartContractAddress);
    }
    
    /* decoding the message to retrieve the stringified message, 
    and storing it along with the global transaction id ( the message identifier ) */
    function _l1xReceive(bytes32 globalTxId, bytes memory message) external {
        require(msg.sender == xtalkNodeAddress, "Caller is not xtalk node");
        XCDPReceiveMessage memory XCDPReceiveMessageData;
        (XCDPReceiveMessageData.message) = abi.decode(message, (string));
        XCDPData[globalTxId] = XCDPReceiveMessageData;
    }
}
