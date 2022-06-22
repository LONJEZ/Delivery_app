use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env, log};
use std::collections::HashMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    trackers: HashMap<u16, ParcelTracker>,
    parcels: HashMap<u16, Parcel>,
    ids: u16,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Parcel {
    // SETUP CONTRACT STATE
    sender_name: String,
    sender_phone_no: usize,
    receiver_name: String,
    receiver_phone_no:usize,
    delivery_charges:u32,
    destination: String,
    is_fragile:bool,
    date_sent:String,
    date_received: String,
    is_received:bool
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct ParcelTracker{
    //Set up contract method
    parcel_id:u16,
    current_location:String,
    has_arrived:bool
}

impl Default for Contract {
    fn default() -> Self {
        Contract {
            trackers: HashMap::new(),
            parcels: HashMap::new(),
            ids:1,
        }
    }
}

#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE
    #[private]
   pub fn new_parcel(&mut self,sender_name: String, sender_phone: usize, receiver_name: String, receiver_phone:usize, 
    charges:u32, destination: String, is_fragile:bool, date:String,) {
    let new_parcel = Parcel {
        sender_name,
        sender_phone_no: sender_phone,
        receiver_name: receiver_name,
        receiver_phone_no: receiver_phone,
        delivery_charges: charges,
        destination,
        is_fragile,
        date_sent: date,
        date_received:"".to_string(),
        is_received: false,
    };

    log!("Id is {}", &self.ids);
    self.parcels.insert(self.ids, new_parcel);
    self.ids += 1;
   }

   #[payable]
   pub fn pay(&mut self, id: u16){
    let tokens = env::attached_deposit() / 10u128.pow(22);
    if let Some(parcel) = self.parcels.get_mut(&id) {
        parcel.delivery_charges = parcel.delivery_charges - tokens as u32;
    }

    if self.parcels[&id].delivery_charges > 1 {
        log!("You still owe {}", self.parcels[&id].delivery_charges);
    } else {
        log!("Your package has been dispatched, your tracking id: {}", &id);
    }
   }

   #[private]
   pub fn dispatch(&mut self, id: u16, location: String){
    if self.parcels[&id].delivery_charges > 10 {
        log!("Client still owes {}", self.parcels[&id].delivery_charges);
        return;
    }
    let new_tracker = ParcelTracker {
        parcel_id: id,
        current_location: location,
        has_arrived: false,
    };

    self.trackers.insert(id, new_tracker);
   }

   pub fn track_package(&self, id: u16, phone: usize) -> String {
    if self.parcels[&id].sender_phone_no != phone {
        log!("Only package owners can tracker packages !");
    }
    self.trackers[&id].current_location.clone()
   }
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
    // use near_sdk::test_utils::{get_logs,accounts};
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // TESTS HERE

    #[test]
    fn test_new_parcel(){
        // let mut context = get_context(accounts(1));
        let mut contract = Contract::default();

        contract.new_parcel("joe".to_string(),12345678, "doe".to_string(), 87654321, 200, "juja".to_string(), true, "10-6-2022".to_string());
        
        assert_eq!(1, contract.parcels.len())
    }

    #[test]
    fn test_pay(){
        let mut context = get_context(vec![], false);
        context.attached_deposit = 100 * 10u128.pow(22);
        context.is_view = false;
        testing_env!(context);

        let mut contract = Contract::default();

        contract.new_parcel("joe".to_string(),12345678, "doe".to_string(), 87654321, 100, "juja".to_string(), true, "10-6-2022".to_string());
        contract.pay(1);

        assert!(contract.parcels[&1].delivery_charges < 1);
    }

    #[test]
    fn test_dispatch(){
        let mut context = get_context(vec![], false);
        context.attached_deposit = 100 * 10u128.pow(22);
        context.is_view = false;
        testing_env!(context);

        let mut contract = Contract::default();

        contract.new_parcel("joe".to_string(),12345678, "doe".to_string(), 87654321, 100, "juja".to_string(), true, "10-6-2022".to_string());
        contract.pay(1);
        contract.dispatch(1, "tudor".to_string());

        assert_eq!(1, contract.trackers.len())
    }
}
