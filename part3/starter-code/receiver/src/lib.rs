use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupSet};
use near_sdk::{log, env, near_bindgen, AccountId, PanicOnDefault, require};

use crate::restrictions::*;

mod restrictions;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Counter {
    //keep track of the owner of the contract
    owner_id: AccountId,
    //keep track of the current value of the counter
    val: i8,
    //keep track of a list of accounts that are allowed to modify the state of the counter
    allow_list: LookupSet<AccountId>
}

#[near_bindgen]
impl Counter {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let this = Self {
            //store the owner ID as a string
            owner_id: owner_id,
            //default the counter value to 0
            val: 0,
            //default allow list to an empty set with a prefix of "allow-list" in bytes.
            allow_list: LookupSet::new(b"allow-list".to_vec()),
        };

        this
    }

    /// Public method: Returns the counter value.
    pub fn get_num(&self) -> i8 {
        return self.val;
    }

    /// Public method: Increment the counter.
    pub fn increment(&mut self) {
        assert_access();
        self.val += 1;
        log!("Increased number to {}", self.val);
        return self.val;
    }

    /// Public method: Decrement the counter.
    pub fn decrement(&mut self) {
        assert_access();
        self.val -= 1;
        log!("Decreased number to {}", self.val);
        return self.val;
    }

    /// Public method - Reset to zero.
    pub fn reset(&mut self) {
        assert_access();
        self.val = 0;
        log!("Reset counter to zero");
        return self.val;
    }
}