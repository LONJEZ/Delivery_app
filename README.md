# DELIVERY APP USING RUST

## Getting started
The smart contract under the delivery app is a contract which can be applied in logistic businesses in registering, identifying and tracking parcels on transist.It allows the logistic company to create a new parcel, generate a unique identification id, accepts payment for the parcel and generate parcel information which is used for tracking.

## How it works

### Imports

Imports all the necessary dependancies needed for this project

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env, log};
use std::collections::HashMap;

Super struct to hold parcel and tracker declaration:

        #[near_bindgen]
        #[derive(BorshDeserialize, BorshSerialize)]
        
        pub struct Contract {
            trackers: HashMap<u16, ParcelTracker>,
            parcels: HashMap<u16, Parcel>,
            ids: u16,
        }

I initialize the defaults  contract to store trackers,parcels and ids using the Default keyword.

impl Default for Contract {
    fn default() -> Self {
        Contract {
            trackers: HashMap::new(),
            parcels: HashMap::new(),
            ids:1,
        }
    }
}

Here I create a struct parcel to define the parcel components.

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

Here I create a struct parcelTracker to define the tracker component
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
 
pub struct ParcelTracker{
    //Set up contract method
    parcel_id:u16,
    current_location:String,
    has_arrived:bool
}

Here I apply the implementation contract methods to create a new parcel
        #[near_bindgen]
        impl Contract {
            
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

 Here I implement the pay method  to check payment for delivery charges and generate tracking id
 
 
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


 Here I impliments  dispatch method to check if the amount paid is equal to  the delivery charges and initiate tracking of the parcel
       
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

Here I implement track_package method  to query parcel location. This method check if the parcel id and the number are equal if the are not the location cannot be accessed.

   pub fn track_package(&self, id: u16, phone: usize) -> String {
    if self.parcels[&id].sender_phone_no != phone {
        log!("Only package owners can tracker packages !");
    }
    self.trackers[&id].current_location.clone()
   }
}


Here this function impliments a dummy near account  used for testing

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

 #[test]

This test cofirms if the new_parcel method is able to create a new parcel

            fn test_new_parcel(){
                // let mut context = get_context(accounts(1));
                let mut contract = Contract::default();

                contract.new_parcel("joe".to_string(),12345678, "doe".to_string(), 87654321, 200, "juja".to_string(), true, "10-6-2022".to_string());
                
                assert_eq!(1, contract.parcels.len())
            }

#[test]
This test confirms if the pay function is able to check the amount paid for the new parcel and generate a tracking id
           
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
This test confirm if the dispatch method is able to check on the amount paid and see if its matches the delivery charges before initiating the tracking
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

A [smart contract] written in [Rust] for an app initialized with [create-near-app]

[smart contract]: https://docs.near.org/docs/develop/contracts/overview
[rust]: https://www.rust-lang.org/
[create-near-app]: https://github.com/near/create-near-app
[correct target]: https://github.com/near/near-sdk-rs#pre-requisites
[cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
