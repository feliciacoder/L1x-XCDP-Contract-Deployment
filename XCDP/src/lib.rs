// Import necessary crates and modules
use borsh::{BorshDeserialize, BorshSerialize};
use ethers::abi::Token;
use l1x_sdk::{contract, store::LookupMap};
use serde::{Deserialize, Serialize};

// Define constants for storage keys
const STORAGE_CONTRACT_KEY: &[u8; 7] = b"message";
const STORAGE_EVENTS_KEY: &[u8; 6] = b"events";

// Import ethers utilities for ABI and event parsing
use ethers::{
    abi::{decode, ParamType},
    prelude::parse_log,
};

// Define data structures for messages
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct XCDPSendMessage {
    message: String,
}

// This structure is used for solidity compatibility
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XCDPSendMessageSolidity {
    message: String,
}

// Conversion trait to allow easy transformations between Solidity and custom Rust structs
impl From<XCDPSendMessageSolidity> for XCDPSendMessage {
    fn from(event: XCDPSendMessageSolidity) -> Self {
        Self {
            message: event.message,
        }
    }
}

// Define the event structure that this contract can parse and emit
#[derive(Clone, Debug, EthEvent, Serialize, Deserialize)]
#[ethevent(name = "XTalkMessageInitiated")]
pub struct XTalkMessageInitiated {
    message: Vec<u8>,
    destination_network: String,
    destination_smart_contract_address: [u8; 32],
}

// Payload structure for inter-chain messages
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Payload {
    data: Vec<u8>,
    destination_network: String,
    destination_contract_address: [u8; 32],
}

// Main contract structure storing all event data
#[derive(BorshSerialize, BorshDeserialize)]
pub struct XCDPCore {
    events: LookupMap<String, XCDPSendMessage>,
    total_events: u64,
}

// Default constructor for the contract
impl Default for XCDPCore {
    fn default() -> Self {
        Self {
            events: LookupMap::new(STORAGE_EVENTS_KEY.to_vec()),
            total_events: u64::default(),
        }
    }
}

// Contract trait implementation containing all business logic
#[contract]
impl XCDPCore {

    // Function to load existing contract data from storage
    fn load() -> Self {
        match l1x_sdk::storage_read(STORAGE_CONTRACT_KEY) {
            Some(bytes) => match Self::try_from_slice(&bytes) {
                Ok(contract) => contract,
                Err(_) => panic!("Unable to parse contract bytes"),
            },
            None => panic!("The contract isn't initialized"),
        }
    }

    // Function to save contract state to storage
    fn save(&mut self) {
        match borsh::BorshSerialize::try_to_vec(self) {
            Ok(encoded_contract) => {
                l1x_sdk::storage_write(STORAGE_CONTRACT_KEY, &encoded_contract);
                log::info!("Saved event data successfully");
            }
            Err(_) => panic!("Unable to save contract"),
        };
    }

    // Constructor to initialize a new contract
    pub fn new() {
        let mut contract = Self::default();
        contract.save();
    }

    // Handler to process incoming events and save the decoded data
    pub fn save_event_data(event_data: Vec<u8>, global_tx_id: String) {
        l1x_sdk::msg(&format!(
            "********************global tx id {} **************",
            global_tx_id
        ));

        let mut contract = Self::load();

        log::info!("Received event data!!!");
        assert!(!global_tx_id.is_empty(), "global_tx_id cannot be empty");
        assert!(!event_data.is_empty(), "event_data cannot be empty");
        assert!(
            !contract.events.contains_key(&global_tx_id),
            "event is saved already"
        );

        let event_data = match base64::decode(event_data) {
            Ok(data) => data,
            Err(_) => panic!("Can't decode base64 event_data"),
        };

        let log: ethers::types::Log =
            serde_json::from_slice(&event_data).expect("Can't deserialize Log object");

        l1x_sdk::msg(&format!("{:#?}", log));
        let event_id = log.topics[0].to_string();
        if let Ok(message_event) = parse_log::<XTalkMessageInitiated>(log.clone()) {
            let event = decode(
                &[
                    ParamType::String,
                ],
                &message_event.message,
            )
            .unwrap();

            let event = XCDPSendMessageSolidity {
                message: event[0].clone().into_string().unwrap(),
            };

            contract.save_message_event(global_tx_id, event_id, event, message_event.destination_network, message_event.destination_smart_contract_address);
        } else {
            panic!("invalid event!")
        }

        contract.save()
    }

    // Function to combine parts of an event into a single storage key
    pub fn to_key(global_tx_id: String, event_type: String) -> String {
        global_tx_id.to_owned() + "-" + &event_type
    }
}

