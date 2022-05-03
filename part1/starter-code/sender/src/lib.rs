use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ext_contract, AccountId, Balance, Gas, Promise};
use near_sdk::near_bindgen;

//Used to attach zero NEAR to a call
const NO_DEPOSIT: Balance = 0;
//Amount of GAS to attach to the cross contract call to get the number
const GAS_FOR_GET_NUM: Gas = Gas(10_000_000_000_000);

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
//defining the contract struct that holds state
pub struct Contract {}

#[near_bindgen]
impl Contract {
    //performs a cross contract call to an external counter contract which will log the current value
    pub fn check_counter(&self, ext_contract_id: AccountId) -> Promise {
        /*
            FILL THIS IN
        */
        todo!(); //remove this line once code is filled in
    }
}
