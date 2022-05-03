use near_sdk::{ext_contract};

//trait outlining the functions we'll be calling stored on the external counter contract
#[ext_contract(ext_counter_contract)]
trait CounterContract {
    //Interface of the method stored on the external contract to get the current counter value.
    fn get_num(
        &self,
    ) -> i8;

    //Interface of the method stored on the external contract to increment the current counter value.
    fn increment(
        &self,
    ) -> i8;
}