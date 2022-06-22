use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use chrono::{Local, DateTime}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Parcel {
    // SETUP CONTRACT STATE
    parcel_id:u8,
    sender_name: String,
    sender_phone_no: u8,
    receiver_name: String,
    receiver_phone_no:u8,
    delivery_charges:u32,
    destination: String,
    is_fragile:bool,
    date_sent:DateTime<Local>,
    date_received: DateTime<Local>,
    is_received:bool
}

pub struct Parcel_tracker{
    //Set up contract method
    parcel_id:u8,
    has_been_dispatched:bool,
    current_location:String,
    has_arrived:bool
}

#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE
   pub fn new (sender:&str,phone:&u8)
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
