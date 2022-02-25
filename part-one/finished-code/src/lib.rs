use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ext_contract, AccountId, Balance, Gas};
use near_sdk::near_bindgen;

const NO_DEPOSIT: Balance = 0;
const GAS_FOR_CHECK_COUNTER: Gas = Gas(10_000_000_000_000);

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
//defining the contract struct that holds state
pub struct Contract {}

#[ext_contract(ext_counter_contract)]
trait CounterContract {
    //Interface of the method stored on the external contract to get the current counter value.
    fn get_num(
        &self,
    ) -> i8;
}

#[near_bindgen]
impl Contract {
    //performs a cross contract call to an external counter contract which will log the current value
    pub fn check_counter(&self, ext_contract_id: AccountId) {
        ext_counter_contract::get_num(
            //receiving contract that we're calling 
            ext_contract_id, 
            //attaching no deposit
            NO_DEPOSIT,
            //attaching 10 TGas
            GAS_FOR_CHECK_COUNTER
        );
    }
}
