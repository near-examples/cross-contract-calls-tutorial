use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    /*
        FILL THIS IN
    */
}

#[near_bindgen]
impl Counter {
    /// Public method: Returns the counter value.
    pub fn get_num(&self) -> i8 {
        /*
            FILL THIS IN
        */
        todo!(); //remove this line once code is filled in
    }

    /// Public method: Increment the counter.
    pub fn increment(&mut self) {
        /*
            FILL THIS IN
        */
    }

    /// Public method: Decrement the counter.
    pub fn decrement(&mut self) {
        /*
            FILL THIS IN
        */
    }

    /// Public method - Reset to zero.
    pub fn reset(&mut self) {
        /*
            FILL THIS IN
        */
    }
}