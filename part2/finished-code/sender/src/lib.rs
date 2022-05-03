use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Balance, Gas, Promise};
use near_sdk::near_bindgen;

//Used to attach zero NEAR to a call
const NO_DEPOSIT: Balance = 0;
//Amount of GAS to attach to the cross contract call to get the number
const GAS_FOR_GET_NUM: Gas = Gas(10_000_000_000_000);
//Amount of GAS to attach to the cross contract call to increment or decrement the counter
const GAS_FOR_INCREMENT_COUNTER: Gas = Gas(15_000_000_000_000);

use crate::external_traits::*;

mod external_traits;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
//defining the contract struct that holds state
pub struct Contract {}

#[near_bindgen]
impl Contract {
    //performs a cross contract call to an external counter contract which will log the current value
    pub fn check_counter(&self, ext_contract_id: AccountId) -> Promise {
        ext_counter_contract::get_num(
            //receiving contract that we're calling 
            ext_contract_id, 
            //attaching no deposit
            NO_DEPOSIT,
            //attaching 10 TGas
            GAS_FOR_GET_NUM
        )
    }

    //performs a cross contract call to an external counter contract which will log the current value
    pub fn increment_counter(&self, ext_contract_id: AccountId) {
        ext_counter_contract::increment(
            //receiving contract that we're calling 
            ext_contract_id, 
            //attaching no deposit
            NO_DEPOSIT,
            //attaching 15 TGas
            GAS_FOR_INCREMENT_DECREMENT
        );
    }
}
